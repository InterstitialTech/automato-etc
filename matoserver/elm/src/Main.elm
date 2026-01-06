port module Main exposing (main)

import AutomatoListing
import AutomatoView
import Browser
import Browser.Events
import Browser.Navigation
import Common
import DisplayMessage
import Element as E exposing (Element)
import Element.Font as EF
import GenDialog as GD
import Html exposing (Html)
import Http
import Json.Decode as JD
import Json.Encode as JE
import LocalStorage as LS
import Messages as M
import Payload
import PublicInterface as PI
import Route exposing (Route(..), parseUrl, routeTitle, routeUrl)
import SerialError
import SerialListing
import ShowMessage
import Task
import Time
import Toop
import Url exposing (Url)
import Url.Parser as UP exposing ((</>))
import Util
import WindowKeys


type Msg
    = ShowMessageMsg ShowMessage.Msg
    | PublicReplyData (Result Http.Error M.ServerResponse)
    | AutomatoMsgReplyData AutomatoView.MsgWhat (Result Http.Error M.ServerResponse)
    | LoadUrl String
    | InternalUrl Url
    | SelectedText JD.Value
    | UrlChanged Url
    | WindowSize Util.Size
    | DisplayMessageMsg (GD.Msg DisplayMessage.Msg)
    | Zone Time.Zone
    | WkMsg (Result JD.Error WindowKeys.Key)
    | ReceiveLocalVal { for : String, name : String, value : Maybe String }
    | SerialListingMsg SerialListing.Msg
    | AutomatoListingMsg AutomatoListing.Msg
    | AutomatoViewMsg AutomatoView.Msg
    | TauriPublicReplyData JD.Value
    | Noop


type State
    = PubShowMessage ShowMessage.Model (Maybe State)
    | DisplayMessage DisplayMessage.GDModel State
    | SerialListing SerialListing.Model
    | AutomatoListing AutomatoListing.Model
    | AutomatoView AutomatoView.Model


type alias Flags =
    { location : String
    , useragent : String
    , tauri : Bool
    , mobile : Bool
    , width : Int
    , height : Int
    }


type alias SavedRoute =
    { route : Route
    , save : Bool
    }


type alias Model =
    { state : State
    , size : Util.Size
    , location : String
    , tauri : Bool
    , appname : String
    , navkey : Browser.Navigation.Key
    , timezone : Time.Zone
    , savedRoute : SavedRoute
    , fontsize : Int
    , requestIdCount : Int
    }


type alias PreInitModel =
    { flags : Flags
    , url : Url
    , key : Browser.Navigation.Key
    , mbzone : Maybe Time.Zone
    , mbfontsize : Maybe Int
    }


type PiModel
    = Ready Model
    | PreInit PreInitModel


urlRequest : Browser.UrlRequest -> Msg
urlRequest ur =
    case ur of
        Browser.Internal url ->
            InternalUrl url

        Browser.External str ->
            LoadUrl str


routeState : Model -> Route -> ( State, Cmd Msg )
routeState model route =
    case route of
        Top ->
            if (stateRoute model.state).route == Top then
                ( model.state, Cmd.none )

            else
                let
                    ( m, c ) =
                        initialPage model
                in
                ( m.state, c )

        AutomatoViewR id ->
            ( (displayMessageDialog model "loading automato info").state
            , sendPIMsg model.tauri
                model.location
                (M.PrAutomatoMsg
                    { id = id
                    , message = Payload.PeReadinfo
                    }
                )
            )


stateRoute : State -> SavedRoute
stateRoute state =
    case state of
        AutomatoView mod ->
            { route = AutomatoViewR mod.id
            , save = True
            }

        _ ->
            { route = Top
            , save = False
            }


showMessage : Msg -> String
showMessage msg =
    case msg of
        DisplayMessageMsg _ ->
            "DisplayMessage"

        ShowMessageMsg _ ->
            "ShowMessageMsg"

        PublicReplyData urd ->
            "PublicReplyData: "
                ++ (Result.map PI.showServerResponse urd
                        |> Result.mapError Util.httpErrorString
                        |> (\r ->
                                case r of
                                    Ok m ->
                                        "message: " ++ m

                                    Err e ->
                                        "error: " ++ e
                           )
                   )

        AutomatoMsgReplyData what urd ->
            "AutomatoMsgReplyData: "
                ++ (Result.map PI.showServerResponse urd
                        |> Result.mapError Util.httpErrorString
                        |> (\r ->
                                case r of
                                    Ok m ->
                                        "message: " ++ m

                                    Err e ->
                                        "error: " ++ e
                           )
                   )

        LoadUrl _ ->
            "LoadUrl"

        InternalUrl _ ->
            "InternalUrl"

        SelectedText _ ->
            "SelectedText"

        UrlChanged _ ->
            "UrlChanged"

        WindowSize _ ->
            "WindowSize"

        Noop ->
            "Noop"

        WkMsg _ ->
            "WkMsg"

        ReceiveLocalVal _ ->
            "ReceiveLocalVal"

        Zone _ ->
            "Zone"

        SerialListingMsg _ ->
            "SerialListingMsg"

        AutomatoListingMsg _ ->
            "AutomatoListingMsg"

        AutomatoViewMsg _ ->
            "AutomatoViewMsg"

        TauriPublicReplyData _ ->
            "TauriPublicReplyData"


showState : State -> String
showState state =
    case state of
        DisplayMessage _ _ ->
            "DisplayMessage"

        PubShowMessage _ _ ->
            "PubShowMessage"

        SerialListing _ ->
            "SerialListing"

        AutomatoListing _ ->
            "AutomatoListing"

        AutomatoView _ ->
            "AutomatoView"


unexpectedMsg : Model -> Msg -> Model
unexpectedMsg model msg =
    unexpectedMessage model (showMessage msg)


unexpectedMessage : Model -> String -> Model
unexpectedMessage model msg =
    displayMessageDialog model
        ("unexpected message - " ++ msg ++ "; state was " ++ showState model.state)


viewState : Util.Size -> State -> Model -> Element Msg
viewState size state model =
    case state of
        PubShowMessage em _ ->
            E.map ShowMessageMsg <| ShowMessage.view em

        DisplayMessage em _ ->
            -- render is at the layout level, not here.
            E.none

        SerialListing em ->
            E.map SerialListingMsg <| SerialListing.view em

        AutomatoListing em ->
            E.map AutomatoListingMsg <| AutomatoListing.view size em

        AutomatoView em ->
            E.map AutomatoViewMsg <| AutomatoView.view size model.timezone em


sendPIMsg : Bool -> String -> M.PublicMessage -> Cmd Msg
sendPIMsg tauri location msg =
    sendPIMsgExp tauri location msg PublicReplyData


sendPIMsgExp : Bool -> String -> M.PublicMessage -> (Result Http.Error M.ServerResponse -> Msg) -> Cmd Msg
sendPIMsgExp tauri location msg tomsg =
    if tauri then
        sendPIValueTauri <| M.publicMessageEncoder msg

    else
        Http.post
            { url = location ++ "/public"
            , body = Http.jsonBody (M.publicMessageEncoder msg)
            , expect = Http.expectJson tomsg M.serverResponseDecoder
            }



-- sendPIMsg : FileUrlInfo -> Data.PublicRequest -> Cmd Msg
-- sendPIMsg fui msg =
--     sendPIMsgExp fui msg PublicReplyData
-- sendPIMsgExp : FileUrlInfo -> Data.PublicRequest -> (Result Http.Error ( Time.Posix, Data.PublicReply ) -> Msg) -> Cmd Msg
-- sendPIMsgExp fui msg tomsg =
--     if fui.tauri then
--         sendPIValueTauri <| Data.publicRequestEncoder msg
--     else
--         HE.postJsonTask
--             { url = fui.location ++ "/public"
--             , body = Http.jsonBody (Data.publicRequestEncoder msg)
--             , decoder = Data.publicReplyDecoder
--             }
--             |> Task.andThen (\x -> Task.map (\posix -> ( posix, x )) Time.now)
--             |> Task.attempt tomsg


piview : PiModel -> { title : String, body : List (Html Msg) }
piview pimodel =
    case pimodel of
        Ready model ->
            view model

        PreInit model ->
            { title = "initializing"
            , body = []
            }


view : Model -> { title : String, body : List (Html Msg) }
view model =
    { title =
        routeTitle model.appname model.savedRoute.route
    , body =
        [ case model.state of
            DisplayMessage dm _ ->
                Html.map DisplayMessageMsg <|
                    GD.layout
                        (Just { width = min 600 model.size.width, height = min 500 model.size.height })
                        dm

            _ ->
                E.layout [ EF.size model.fontsize, E.width E.fill ] <| viewState model.size model.state model
        ]
    }


piupdate : Msg -> PiModel -> ( PiModel, Cmd Msg )
piupdate msg initmodel =
    case initmodel of
        Ready model ->
            let
                ( m, c ) =
                    urlupdate msg model
            in
            ( Ready m, c )

        PreInit imod ->
            let
                nmod =
                    case msg of
                        Zone zone ->
                            { imod | mbzone = Just zone }

                        ReceiveLocalVal lv ->
                            let
                                default =
                                    16

                                defaultsaveonclonk =
                                    True

                                defaultpageincrement =
                                    25
                            in
                            case lv.name of
                                "fontsize" ->
                                    case lv.value of
                                        Just v ->
                                            case String.toInt v of
                                                Just i ->
                                                    { imod | mbfontsize = Just i }

                                                Nothing ->
                                                    { imod | mbfontsize = Just default }

                                        Nothing ->
                                            { imod | mbfontsize = Just default }

                                _ ->
                                    { imod | mbfontsize = Nothing }

                        _ ->
                            imod
            in
            case Toop.T2 nmod.mbzone nmod.mbfontsize of
                Toop.T2 (Just zone) (Just fontsize) ->
                    let
                        ( m, c ) =
                            init imod.flags imod.url imod.key zone fontsize
                    in
                    ( Ready m, c )

                _ ->
                    ( PreInit nmod, Cmd.none )


{-| urlUpdate: all URL code shall go here! regular code shall not worry about urls!
this function calls actualupdate where the app stuff happens.
url messages and state based url changes are done here.
-}
urlupdate : Msg -> Model -> ( Model, Cmd Msg )
urlupdate msg model =
    let
        ( nm, cmd ) =
            case msg of
                InternalUrl url ->
                    let
                        ( state, icmd ) =
                            parseUrl url
                                |> Maybe.map (routeState model)
                                |> Maybe.withDefault ( model.state, Cmd.none )
                    in
                    ( { model | state = state }, icmd )

                LoadUrl urlstr ->
                    -- load foreign site
                    -- ( model, Browser.Navigation.load urlstr )
                    ( model, Cmd.none )

                UrlChanged url ->
                    -- we get this from forward and back buttons.  if the user changes the url
                    -- in the browser address bar, its a site reload so this isn't called.
                    case parseUrl url of
                        Just route ->
                            if route == (stateRoute model.state).route then
                                ( model, Cmd.none )

                            else
                                let
                                    ( st, rscmd ) =
                                        routeState model route
                                in
                                -- swap out the savedRoute, so we don't write over history.
                                ( { model
                                    | state = st
                                    , savedRoute =
                                        let
                                            nssr =
                                                stateRoute st
                                        in
                                        { nssr | save = False }
                                  }
                                , rscmd
                                )

                        Nothing ->
                            -- load foreign site
                            -- ( model, Browser.Navigation.load (Url.toString url) )
                            ( model, Cmd.none )

                _ ->
                    -- not an url related message!  pass it on to the 'actualupdate'
                    -- this is where all the app stuff happens.
                    actualupdate msg model

        sr =
            stateRoute nm.state
    in
    -- when the route changes, change the address bar, optionally pushing what's there to
    -- browser history.
    if sr.route /= nm.savedRoute.route then
        ( { nm | savedRoute = sr }
        , if model.savedRoute.save then
            Cmd.batch
                [ cmd
                , Browser.Navigation.pushUrl nm.navkey
                    (routeUrl sr.route)
                ]

          else
            Cmd.batch
                [ cmd
                , Browser.Navigation.replaceUrl nm.navkey
                    (routeUrl sr.route)
                ]
        )

    else
        ( nm, cmd )


displayMessageDialog : Model -> String -> Model
displayMessageDialog model message =
    { model
        | state =
            DisplayMessage
                (DisplayMessage.init Common.buttonStyle
                    message
                    (E.map (\_ -> ()) (viewState model.size model.state model))
                )
                model.state
    }


actualupdate : Msg -> Model -> ( Model, Cmd Msg )
actualupdate msg model =
    case ( msg, model.state ) of
        ( ReceiveLocalVal lv, _ ) ->
            -- update the font size
            ( model, Cmd.none )

        ( WindowSize s, _ ) ->
            ( { model | size = s }, Cmd.none )

        ( TauriPublicReplyData val, state ) ->
            case JD.decodeValue M.serverResponseDecoder val of
                Ok td ->
                    actualupdate (PublicReplyData (Ok td)) model

                Err e ->
                    ( displayMessageDialog model <| JD.errorToString e
                    , Cmd.none
                    )

        ( PublicReplyData urd, state ) ->
            case urd of
                Err e ->
                    ( displayMessageDialog model <| Util.httpErrorString e, Cmd.none )

                Ok uiresponse ->
                    case uiresponse of
                        M.SrGenericError e ->
                            ( displayMessageDialog model <| e, Cmd.none )

                        M.SrSerialPorts infolist ->
                            ( { model
                                | state =
                                    SerialListing (SerialListing.init infolist)
                              }
                            , Cmd.none
                            )

                        M.SrSerialPortOpened s ->
                            ( model, sendPIMsg model.tauri model.location <| M.PrGetAutomatoList )

                        M.SrAutomatos x ->
                            ( { model
                                | state =
                                    AutomatoListing (AutomatoListing.init x)
                              }
                            , Cmd.none
                            )

                        M.SrAutomatoMsg am ->
                            case ( model.state, am.message ) of
                                -- ( AutomatoView av, _ ) ->
                                --     handleAutomatoView model (AutomatoView.onAutomatoMsg am what av)
                                ( _, Payload.PeReadinforeply info ) ->
                                    handleAutomatoView model
                                        (AutomatoView.init am.id
                                            info
                                            model.requestIdCount
                                        )

                                _ ->
                                    ( model, Cmd.none )

                        M.SrSerialError se ->
                            ( displayMessageDialog model (JE.encode 2 (SerialError.errorEncoder se)), Cmd.none )

        ( AutomatoMsgReplyData what urd, state ) ->
            case urd of
                Err e ->
                    ( displayMessageDialog model <| Util.httpErrorString e, Cmd.none )

                Ok uiresponse ->
                    case uiresponse of
                        M.SrGenericError e ->
                            ( displayMessageDialog model <| e, Cmd.none )

                        M.SrSerialPorts infolist ->
                            ( { model
                                | state =
                                    SerialListing (SerialListing.init infolist)
                              }
                            , Cmd.none
                            )

                        M.SrSerialPortOpened s ->
                            ( model, sendPIMsg model.tauri model.location <| M.PrGetAutomatoList )

                        M.SrAutomatos x ->
                            ( { model
                                | state =
                                    AutomatoListing (AutomatoListing.init x)
                              }
                            , Cmd.none
                            )

                        M.SrAutomatoMsg am ->
                            case ( model.state, am.message ) of
                                ( AutomatoView av, _ ) ->
                                    handleAutomatoView model (AutomatoView.onAutomatoMsg am what av)

                                ( _, Payload.PeReadinforeply info ) ->
                                    handleAutomatoView model
                                        (AutomatoView.init am.id
                                            info
                                            model.requestIdCount
                                        )

                                _ ->
                                    -- TODO handle errors always!
                                    ( model, Cmd.none )

                        M.SrSerialError se ->
                            case model.state of
                                AutomatoView av ->
                                    handleAutomatoView model (AutomatoView.onSerialError se what av)

                                DisplayMessage _ (AutomatoView av) ->
                                    handleAutomatoView model (AutomatoView.onSerialError se what av)

                                _ ->
                                    -- TODO handle errors always!
                                    ( model, Cmd.none )

        ( DisplayMessageMsg bm, DisplayMessage bs prevstate ) ->
            case GD.update bm bs of
                GD.Dialog nmod ->
                    ( { model | state = DisplayMessage nmod prevstate }, Cmd.none )

                GD.Ok _ ->
                    case prevstate of
                        PubShowMessage _ (Just ps) ->
                            ( { model | state = ps }, Cmd.none )

                        _ ->
                            ( { model | state = prevstate }, Cmd.none )

                GD.Cancel ->
                    ( { model | state = prevstate }, Cmd.none )

        ( Noop, _ ) ->
            ( model, Cmd.none )

        ( DisplayMessageMsg GD.Noop, _ ) ->
            ( model, Cmd.none )

        ( SerialListingMsg ms, SerialListing st ) ->
            let
                ( nm, cmd ) =
                    SerialListing.update ms st
            in
            case cmd of
                SerialListing.Selected s ->
                    ( { model | state = SerialListing nm }
                    , sendPIMsg model.tauri model.location <|
                        M.PrOpenSerialPort s
                    )

                -- SerialListing.New ->
                --     ( { model | state = SerialEdit (SerialEdit.initNew login) login }
                --     , Cmd.none
                --     )
                SerialListing.Done ->
                    ( { model | state = SerialListing nm }, Cmd.none )

                -- SerialListing.Settings ->
                --     ( { model
                --         | state =
                --             UserSettings (UserSettings.init login model.fontsize model.saveonclonk model.pageincrement) login model.state
                --       }
                --     , Cmd.none
                --     )
                SerialListing.None ->
                    ( { model | state = SerialListing nm }, Cmd.none )

        ( AutomatoListingMsg ms, AutomatoListing st ) ->
            let
                ( nm, cmd ) =
                    AutomatoListing.update ms st
            in
            case cmd of
                AutomatoListing.Selected id ->
                    ( { model | state = AutomatoListing nm }
                    , sendPIMsg model.tauri model.location <|
                        M.PrAutomatoMsg
                            { id = id
                            , message = Payload.PeReadinfo
                            }
                    )

                -- AutomatoListing.New ->
                --     ( { model | state = AutomatoEdit (AutomatoEdit.initNew login) login }
                --     , Cmd.none
                --     )
                AutomatoListing.Done ->
                    ( { model | state = AutomatoListing nm }, Cmd.none )

                -- AutomatoListing.Settings ->
                --     ( { model
                --         | state =
                --             UserSettings (UserSettings.init login model.fontsize model.saveonclonk model.pageincrement) login model.state
                --       }
                --     , Cmd.none
                --     )
                AutomatoListing.None ->
                    ( { model | state = AutomatoListing nm }, Cmd.none )

        ( AutomatoViewMsg ms, AutomatoView st ) ->
            handleAutomatoView model (AutomatoView.update ms st model.timezone)

        ( x, y ) ->
            ( unexpectedMsg model x
            , Cmd.none
            )


handleAutomatoView : Model -> ( AutomatoView.Model, AutomatoView.Command ) -> ( Model, Cmd Msg )
handleAutomatoView model ( nm, cmd ) =
    case cmd of
        AutomatoView.Done ->
            ( { model | state = AutomatoView nm }
            , sendPIMsg model.tauri model.location <| M.PrGetAutomatoList
            )

        AutomatoView.ShowError e ->
            ( displayMessageDialog { model | state = AutomatoView nm } e, Cmd.none )

        AutomatoView.SendAutomatoMsg am what ->
            ( { model
                | state = AutomatoView nm
                , requestIdCount = 0
              }
            , sendPIMsgExp model.tauri model.location (M.PrAutomatoMsg am) (AutomatoMsgReplyData what)
            )

        AutomatoView.None ->
            ( { model | state = AutomatoView nm }, Cmd.none )


preinit : Flags -> Url -> Browser.Navigation.Key -> ( PiModel, Cmd Msg )
preinit flags url key =
    ( PreInit
        { flags = flags
        , url = url
        , key = key
        , mbzone = Nothing
        , mbfontsize = Nothing
        }
    , Cmd.batch
        [ Task.perform Zone Time.here
        , LS.getLocalVal { for = "", name = "fontsize" }
        ]
    )



-- initialPage : Model -> ( Model, Cmd Msg )
-- initialPage curmodel =
--     ( { curmodel
--         | state = PubShowMessage { message = "retrieving automato list" } Nothing
--       }
--     , sendPIMsg curmodel.location <| M.PrGetAutomatoList
--     )
--         |> (\( m, c ) ->
--                 ( m
--                 , Cmd.batch
--                     [ Browser.Navigation.replaceUrl m.navkey
--                         (routeUrl (stateRoute m.state).route)
--                     , c
--                     ]
--                 )
--            )


initialPage : Model -> ( Model, Cmd Msg )
initialPage curmodel =
    ( { curmodel
        | state = PubShowMessage { message = "retrieving serial port list" } Nothing
      }
    , sendPIMsg curmodel.tauri curmodel.location <| M.PrGetSerialPortList
    )
        |> (\( m, c ) ->
                ( m
                , Cmd.batch
                    [ Browser.Navigation.replaceUrl m.navkey
                        (routeUrl (stateRoute m.state).route)
                    , c
                    ]
                )
           )


init : Flags -> Url -> Browser.Navigation.Key -> Time.Zone -> Int -> ( Model, Cmd Msg )
init flags url key zone fontsize =
    let

        imodel =
                { state =
                    PubShowMessage { message = "loading..." } Nothing
                , size = { width = flags.width, height = flags.height }
                , location = flags.location
                , tauri = flags.tauri
                , appname = "matoserver"
                , navkey = key
                , timezone = zone
                , savedRoute = { route = Top, save = False }
                , fontsize = fontsize
                , requestIdCount = 0
                }

        -- setkeys =
        --     skcommand <|
        --         WindowKeys.SetWindowKeys
        --             [ { key = "s", ctrl = True, alt = False, shift = False, preventDefault = True }
        --             , { key = "s", ctrl = True, alt = True, shift = False, preventDefault = True }
        --             , { key = "e", ctrl = True, alt = True, shift = False, preventDefault = True }
        --             , { key = "r", ctrl = True, alt = True, shift = False, preventDefault = True }
        --             , { key = "v", ctrl = True, alt = True, shift = False, preventDefault = True }
        --             , { key = "Enter", ctrl = False, alt = False, shift = False, preventDefault = False }
        --             ]
    in
    initialPage imodel



-- parseUrl (Debug.log "url" url)
--     |> Maybe.andThen
--         (\s ->
--             case s of
--                 Top ->
--                     Nothing
--                 _ ->
--                     Just s
--         )
--     |> Maybe.map
--         (routeState
--             imodel
--         )
--     |> Maybe.map
--         (\( rs, rcmd ) ->
--             ( { imodel
--                 | state = rs
--               }
--               -- , Cmd.batch [ rcmd, setkeys ]
--             , Cmd.batch [ rcmd ]
--             )
--         )
--     |> Maybe.withDefault
--         (let
--             ( m, c ) =
--                 initialPage imodel
--          in
--          ( m
--          , Cmd.batch
--             [ c
--             -- , setkeys
--             , Browser.Navigation.replaceUrl key "/"
--             ]
--          )
--         )


main : Platform.Program Flags PiModel Msg
main =
    Browser.application
        { init = preinit
        , view = piview
        , update = piupdate
        , subscriptions =
            \_ ->
                Sub.batch
                    [ receiveSelectedText SelectedText
                    , Browser.Events.onResize (\w h -> WindowSize { width = w, height = h })
                    , keyreceive
                    , LS.localVal ReceiveLocalVal
                    , receivePITauriResponse TauriPublicReplyData
                    ]
        , onUrlRequest = urlRequest
        , onUrlChange = UrlChanged
        }


port getSelectedText : List String -> Cmd msg


port receiveSelectedText : (JD.Value -> msg) -> Sub msg


port receiveKeyMsg : (JD.Value -> msg) -> Sub msg


port sendPIValueTauri : JD.Value -> Cmd msg


port receivePITauriResponse : (JD.Value -> msg) -> Sub msg


keyreceive =
    receiveKeyMsg <| WindowKeys.receive WkMsg


port sendKeyCommand : JE.Value -> Cmd msg


skcommand =
    WindowKeys.send sendKeyCommand
