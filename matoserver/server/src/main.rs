use clap::Arg;
mod config;
mod data;
mod interfaces;
mod messages;
mod util;
use actix_session::Session;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use config::Config;
use log::{error, info};
use messages::{PublicMessage, ServerResponse};
use serde_json;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

/*
use actix_files::NamedFile;

TODO don't hardcode these paths.  Use config.static_path
fn favicon(_req: &HttpRequest) -> Result<NamedFile> {
  let stpath = Path::new("static/favicon.ico");
  Ok(NamedFile::open(stpath)?)
}

fn sitemap(_req: &HttpRequest) -> Result<NamedFile> {
  let stpath = Path::new("static/sitemap.txt");
  Ok(NamedFile::open(stpath)?)
}
*/

// simple index handler
fn mainpage(_session: Session, data: web::Data<Config>, req: HttpRequest) -> HttpResponse {
    info!("remote ip: {:?}, request:{:?}", req.connection_info(), req);

    let mut staticpath = data.static_path.clone().unwrap_or(PathBuf::from("static/"));
    staticpath.push("index.html");
    match staticpath.to_str() {
        Some(path) => match util::load_string(path) {
            Ok(s) => {
                // search and replace with logindata!
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(s)
            }
            Err(e) => HttpResponse::from_error(actix_web::error::ErrorImATeapot(e)),
        },
        None => HttpResponse::from_error(actix_web::error::ErrorImATeapot("bad static path")),
    }
}

fn public(
    data: web::Data<Config>,
    item: web::Json<PublicMessage>,
    req: HttpRequest,
) -> HttpResponse {
    info!(
        "public msg: {:?} \n connection_info: {:?}",
        &item,
        req.connection_info()
    );

    match interfaces::public_interface(&data, item.into_inner()) {
        Ok(sr) => HttpResponse::Ok().json(sr),
        Err(e) => {
            error!("'public' err: {:?}", e);
            let se = ServerResponse {
                what: "server error".to_string(),
                content: serde_json::Value::String(e.to_string()),
            };
            HttpResponse::Ok().json(se)
        }
    }
}

fn defcon() -> Config {
    Config {
        ip: "127.0.0.1".to_string(),
        port: 8000,
        static_path: None,
        automato_ids: [].to_vec(),
    }
}

fn load_config() -> Config {
    match util::load_string("config.toml") {
        Err(e) => {
            error!("error loading config.toml: {:?}", e);
            defcon()
        }
        Ok(config_str) => match toml::from_str(config_str.as_str()) {
            Ok(c) => c,
            Err(e) => {
                error!("error loading config.toml: {:?}", e);
                defcon()
            }
        },
    }
}

fn main() {
    match err_main() {
        Err(e) => error!("error: {:?}", e),
        Ok(_) => (),
    }
}

#[actix_web::main]
async fn err_main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("matoserver")
        .version("1.0")
        .author("Ben Burdette")
        .about("web server for automato")
        // .arg(
        //     Arg::with_name("config")
        //         .short("c")
        //         .long("config")
        //         .value_name("FILE")
        //         .help("Specify config file")
        //         .takes_value(true),
        // )
        .arg(
            Arg::with_name("writeconfig")
                .short("w")
                .long("writeconfig")
                .value_name("FILE")
                .help("Write default config file")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("writeconfig") {
        Some(exportfile) => {
            // do that exporting...
            let config = defcon();

            util::write_string(exportfile, toml::to_string_pretty(&config)?.as_str())?;

            // util::write_string(
            //   exportfile,
            //   serde_json::to_string_pretty(&sqldata::export_db(config.db.as_path())?)?.as_str(),
            // )?;

            Ok(())
        }
        None => {
            // normal server ops
            env_logger::init();

            info!("server init!");

            let mut config = load_config();

            if config.static_path == None {
                for (key, value) in env::vars() {
                    if key == "MATOSERVER_STATIC_PATH" {
                        config.static_path = PathBuf::from_str(value.as_str()).ok();
                    }
                }
            }

            info!("config: {:?}", config);

            let c = config.clone();

            HttpServer::new(move || {
                let staticpath = c.static_path.clone().unwrap_or(PathBuf::from("static/"));
                App::new()
                    .data(c.clone()) // <- create app with shared state
                    .wrap(middleware::Logger::default())
                    .service(web::resource("/public").route(web::post().to(public)))
                    .service(actix_files::Files::new("/static/", staticpath))
                    .service(web::resource("/{tail:.*}").route(web::get().to(mainpage)))
            })
            .bind(format!("{}:{}", config.ip, config.port))?
            .run()
            .await?;

            Ok(())
        }
    }
}
