use clap::Arg;
mod config;
mod data;
mod interfaces;
mod messages;
mod util;
use actix_session::Session;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use automato::automatomsg as am;
use config::Config;
use data::{AutomatoMsg, ServerData};
use log::{error, info};
use messages::{PublicMessage, ServerResponse};
use serde_json;
use serial::{BaudRate, CharSize, FlowControl, Parity, PortSettings, SerialPort, StopBits};
use simple_error::bail;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
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
fn mainpage(_session: Session, data: web::Data<ServerData>, req: HttpRequest) -> HttpResponse {
    info!("remote ip: {:?}, request:{:?}", req.connection_info(), req);

    let mut staticpath = data
        .config
        .static_path
        .clone()
        .unwrap_or(PathBuf::from("static/"));
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
    data: web::Data<ServerData>,
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
            Arg::new("writeconfig")
                .short('w')
                .long("writeconfig")
                .value_name("FILE")
                .help("Write default config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("FILE")
                .help("serial port")
                .default_value("/dev/ttyUSB0")
                .takes_value(true),
        )
        .arg(
            Arg::new("baud")
                .short('b')
                .long("baud")
                .value_name("NUMBER")
                .help(
                    "baud rate: 110, 300, 600, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200",
                )
                .default_value("115200")
                .takes_value(true),
        )
        .arg(
            Arg::new("writeelmbindings")
                .long("writeelmbindings")
                .value_name("FILE")
                .help("Write elmbindings file")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("writeelmbindings") {
        Some(exportfile) => {
            let mut target = vec![];
            // elm_rs provides a macro for conveniently creating an Elm module with everything needed
            elm_rs::export!(
                "Payload",
                &mut target,
                am::RemoteInfo,
                am::Pinval,
                am::AnalogPinval,
                am::Pinmode,
                am::Readmem,
                am::ReadmemReply,
                am::Writemem,
                am::ReadField,
                am::ReadFieldReply,
                am::PayloadEnum,
                AutomatoMsg
            )
            .unwrap();
            let output = String::from_utf8(target).unwrap();

            util::write_string(exportfile, output.as_str())?;

            println!("wrote file: {}", exportfile);

            return Ok(());
        }

        None => (),
    }

    match matches.value_of("writeconfig") {
        Some(exportfile) => {
            let config = defcon();
            util::write_string(exportfile, toml::to_string_pretty(&config)?.as_str())?;

            Ok(())
        }
        None => {
            // normal server ops
            env_logger::init();

            info!("server init!");

            let (port, baud) = match (matches.value_of("port"), matches.value_of("baud")) {
                (Some(port), Some(baudstr)) => {
                    let baud = BaudRate::from_speed(baudstr.parse::<usize>()?);
                    (port, baud)
                }
                _ => bail!("arg failure"),
            };

            let mut config = load_config();

            if config.static_path == None {
                for (key, value) in env::vars() {
                    if key == "MATOSERVER_STATIC_PATH" {
                        config.static_path = PathBuf::from_str(value.as_str()).ok();
                    }
                }
            }

            info!("config: {:?}", config);

            let mut port = serial::open(port)?;

            let ps = PortSettings {
                baud_rate: baud,
                char_size: CharSize::Bits8,
                parity: Parity::ParityNone,
                stop_bits: StopBits::Stop1,
                flow_control: FlowControl::FlowNone,
            };
            port.configure(&ps)?;

            let mp = Arc::new(Mutex::new(port));

            let c = config.clone();

            HttpServer::new(move || {
                let staticpath = c.static_path.clone().unwrap_or(PathBuf::from("static/"));
                App::new()
                    .data(ServerData {
                        port: mp.clone(),
                        config: c.clone(),
                    }) // <- create app with shared state
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
