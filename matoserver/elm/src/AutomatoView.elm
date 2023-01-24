module AutomatoView exposing (..)

import Calendar
import Common
import Csv
import Data
import Dict exposing (Dict)
import Element as E exposing (Element)
import Element.Background as EBk
import Element.Border as EBd
import Element.Events as EE
import Element.Font as EF
import Element.Input as EI
import Json.Decode as JD
import Json.Encode as JE
import Messages exposing (AutomatoMsg)
import MsCommon as MS
import Payload
import Round as R
import SerialError
import Set
import TDict exposing (TDict)
import TSet exposing (TSet)
import TangoColors as TC
import Time
import Toop
import Util
import WindowKeys as WK


type Msg
    = DonePress
    | SelectField Int
    | EditField String
    | EditUpdate
    | EditCancel
    | EditRefresh
    | Noop


type alias Field =
    { rfr : Payload.ReadFieldReply
    , value : Maybe Data.FieldValue
    }


type alias PendingMsg =
    { automatoMsg : AutomatoMsg
    , what : MsgWhat
    }


type alias MsgWhat =
    { id : Int
    , field : Maybe Int
    }


type alias Model =
    { automatoinfo : Payload.RemoteInfo
    , id : Int
    , temperature : Maybe Float
    , humidity : Maybe Float
    , fields : Dict Int Field
    , pendingMsgs : List PendingMsg
    , editedField : Maybe ( Int, String )
    , requestIdCount : Int
    }


type Command
    = Done
    | ShowError String
    | SendAutomatoMsg AutomatoMsg MsgWhat
    | None


headerStyle : List (E.Attribute msg)
headerStyle =
    [ EF.bold ]


init : Int -> Payload.RemoteInfo -> Int -> ( Model, Command )
init automatoid ai requestIdCount =
    let
        pendingMsgs0 =
            [ { automatoMsg = { id = automatoid, message = Payload.PeReadtemperature }
              , what = { id = 0, field = Nothing }
              }
            , { automatoMsg = { id = automatoid, message = Payload.PeReadhumidity }
              , what = { id = 0, field = Nothing }
              }
            ]
                ++ (List.range 0 (ai.fieldcount - 1)
                        |> List.map
                            (\i ->
                                { automatoMsg = { id = automatoid, message = Payload.PeReadfield { index = i } }
                                , what = { id = 0, field = Nothing }
                                }
                            )
                   )

        -- add sequential ids
        pendingMsgs =
            pendingMsgs0
                |> List.indexedMap
                    (\idx item ->
                        let
                            w =
                                item.what
                        in
                        { item | what = { w | id = idx + requestIdCount } }
                    )

        model =
            { automatoinfo = ai
            , id = automatoid
            , temperature = Nothing
            , humidity = Nothing
            , fields = Dict.empty

            -- , pendingMsgs = pendingMsgs
            , pendingMsgs = pendingMsgs
            , editedField = Nothing
            , requestIdCount = requestIdCount + List.length pendingMsgs
            }
    in
    ( model
    , case List.head pendingMsgs of
        Just pm ->
            SendAutomatoMsg pm.automatoMsg pm.what

        Nothing ->
            None
    )


readField : Payload.ReadFieldReply -> Payload.Readmem
readField rfr =
    { address = rfr.offset
    , length = rfr.length
    }


onSerialError : SerialError.Error -> MsgWhat -> Model -> ( Model, Command )
onSerialError se mw model =
    let
        _ =
            Debug.log "onSerialError: " se

        _ =
            Debug.log "pending what: " (List.head model.pendingMsgs |> Maybe.map .what)

        _ =
            Debug.log "incoming what: " mw

        pms =
            List.drop 1 model.pendingMsgs
    in
    ( { model | pendingMsgs = pms }
    , case List.head pms of
        Just pm ->
            SendAutomatoMsg pm.automatoMsg pm.what

        Nothing ->
            None
    )


onAutomatoMsg : AutomatoMsg -> MsgWhat -> Model -> ( Model, Command )
onAutomatoMsg am msgwhat model =
    let
        _ =
            Debug.log "onAutomatoMsg: " am

        _ =
            Debug.log "pending what: " (List.head model.pendingMsgs |> Maybe.map .what)

        _ =
            Debug.log "incoming what: " msgwhat
    in
    let
        nm0 =
            { model | pendingMsgs = List.drop 1 model.pendingMsgs }

        nm =
            case am.message of
                Payload.PeReadfieldreply rfr ->
                    { nm0
                        | fields = Dict.insert rfr.index { rfr = rfr, value = Nothing } nm0.fields
                        , pendingMsgs =
                            nm0.pendingMsgs
                                ++ [ { automatoMsg =
                                        { id = nm0.id
                                        , message = Payload.PeReadmem <| readField rfr
                                        }
                                     , what =
                                        { id = nm0.requestIdCount, field = Just rfr.index }
                                     }
                                   ]
                        , requestIdCount = nm0.requestIdCount + 1
                    }

                Payload.PeReadmemreply rmr ->
                    msgwhat.field
                        |> Maybe.andThen
                            (\i ->
                                Dict.get i nm0.fields
                            )
                        |> Maybe.andThen
                            (\field ->
                                Data.decodeValue field.rfr.format rmr
                                    |> Maybe.map
                                        (\val ->
                                            { nm0
                                                | fields =
                                                    Dict.insert field.rfr.index
                                                        { rfr = field.rfr
                                                        , value = Just val
                                                        }
                                                        nm0.fields
                                            }
                                        )
                            )
                        |> Maybe.withDefault nm0

                Payload.PeReadtemperaturereply f ->
                    { nm0
                        | temperature = Just f
                    }

                Payload.PeReadhumidityreply f ->
                    { nm0
                        | humidity = Just f
                    }

                _ ->
                    nm0
    in
    ( nm
    , case List.head nm.pendingMsgs of
        Just pm ->
            let
                _ =
                    Debug.log "sending" pm
            in
            SendAutomatoMsg pm.automatoMsg pm.what

        Nothing ->
            None
    )


view : Util.Size -> Time.Zone -> Model -> Element Msg
view size zone model =
    let
        maxwidth =
            700

        titlemaxconst =
            85
    in
    E.column []
        [ E.table [ E.spacing 8 ]
            { data =
                [ ( "protocol version: "
                  , String.fromFloat model.automatoinfo.protoversion
                  )
                , ( "macAddress: "
                  , String.fromInt model.automatoinfo.macAddress
                  )
                , ( "datalen: "
                  , String.fromInt model.automatoinfo.datalen
                  )
                , ( "fieldcount: "
                  , String.fromInt model.automatoinfo.fieldcount
                  )
                , ( "temperature: "
                  , case model.temperature of
                        Just f ->
                            String.fromFloat f

                        Nothing ->
                            "?"
                  )
                , ( "humidity: "
                  , case model.humidity of
                        Just f ->
                            String.fromFloat f

                        Nothing ->
                            "?"
                  )
                ]
            , columns =
                [ { header = E.none
                  , width = E.shrink
                  , view = \( name, _ ) -> E.text name
                  }
                , { header = E.none
                  , width = E.shrink
                  , view = \( _, value ) -> E.text value
                  }
                ]
            }
        , E.column [ E.padding 15, E.spacing 15 ]
            [ E.el [ E.centerX, EF.bold ] <| E.text "data fields"
            , E.table [ E.spacing 8 ]
                { data = Dict.values model.fields
                , columns =
                    [ { header = E.text <| "index"
                      , width = E.shrink
                      , view = \fld -> E.text <| String.fromInt fld.rfr.index
                      }
                    , { header = E.text <| "offset"
                      , width = E.shrink
                      , view = \fld -> E.text <| String.fromInt fld.rfr.offset
                      }
                    , { header = E.text <| "length"
                      , width = E.shrink
                      , view = \fld -> E.text <| String.fromInt fld.rfr.length
                      }
                    , { header = E.text <| "format"
                      , width = E.shrink
                      , view = \fld -> E.text <| String.fromInt fld.rfr.format
                      }
                    , { header = E.text <| "name"
                      , width = E.shrink
                      , view = \fld -> E.text <| String.fromList (List.map Char.fromCode fld.rfr.name)
                      }
                    , { header = E.text <| "value"
                      , width = E.shrink
                      , view =
                            \fld ->
                                case fld.value of
                                    Just v ->
                                        case
                                            model.editedField
                                                |> Maybe.andThen
                                                    (\ef ->
                                                        if Tuple.first ef == fld.rfr.index then
                                                            Just ef

                                                        else
                                                            Nothing
                                                    )
                                        of
                                            Just ( idx, str ) ->
                                                let
                                                    valid =
                                                        editUpdates model /= model

                                                    -- |> Util.isJust
                                                in
                                                E.column
                                                    [ EBk.color TC.gray
                                                    , E.paddingEach
                                                        { top = 0, right = 8, bottom = 8, left = 8 }
                                                    , E.spacing 8
                                                    ]
                                                    [ E.row
                                                        [ EF.bold
                                                        , EE.onClick (SelectField fld.rfr.index)
                                                        , E.width E.fill
                                                        , E.height E.fill
                                                        , E.spacing 8
                                                        ]
                                                        [ case Data.showFieldValue v of
                                                            "" ->
                                                                E.text " "

                                                            s ->
                                                                E.text s
                                                        ]
                                                    , EI.text
                                                        [ if not valid then
                                                            EF.color TC.red

                                                          else
                                                            EF.color TC.black
                                                        ]
                                                        { onChange = EditField
                                                        , text = str
                                                        , placeholder = Nothing
                                                        , label = EI.labelHidden "edited field"
                                                        }
                                                    , E.row [ E.width E.fill, E.spacing 8 ]
                                                        [ if valid then
                                                            EI.button Common.buttonStyle
                                                                { onPress = Just EditUpdate
                                                                , label = E.text "update"
                                                                }

                                                          else
                                                            EI.button Common.disabledButtonStyle
                                                                { onPress = Nothing
                                                                , label = E.text "update"
                                                                }
                                                        , EI.button Common.buttonStyle
                                                            { onPress = Just EditCancel
                                                            , label = E.text "cancel"
                                                            }
                                                        , EI.button Common.buttonStyle
                                                            { onPress = Just EditRefresh
                                                            , label = E.text "refresh"
                                                            }
                                                        ]
                                                    ]

                                            Nothing ->
                                                E.row [ EF.bold, EE.onClick (SelectField fld.rfr.index) ]
                                                    [ case Data.showFieldValue v of
                                                        "" ->
                                                            E.text " "

                                                        s ->
                                                            E.text s
                                                    ]

                                    Nothing ->
                                        E.none
                      }
                    ]
                }
            ]
        , EI.button Common.buttonStyle { onPress = Just DonePress, label = E.text "done" }
        ]


editUpdates :
    Model
    -> Model -- Maybe ( Model, Command, PendingMsg )
editUpdates model =
    model.editedField
        |> Maybe.andThen
            (\( efi, efs ) ->
                Dict.get efi model.fields
                    |> Maybe.andThen
                        (\fld ->
                            Data.strToFieldValue fld.rfr efs
                                |> Maybe.map
                                    (\fv ->
                                        let
                                            newmsgs =
                                                [ { automatoMsg =
                                                        { id = model.id
                                                        , message =
                                                            Payload.PeWritemem
                                                                { address = fld.rfr.offset
                                                                , data = Data.encodeFieldValue fv
                                                                }
                                                        }
                                                  , what =
                                                        { id = model.requestIdCount
                                                        , field = Nothing
                                                        }
                                                  }
                                                , { automatoMsg =
                                                        { id = model.id
                                                        , message = Payload.PeReadmem <| readField fld.rfr
                                                        }
                                                  , what =
                                                        { id = model.requestIdCount + 1
                                                        , field = Just fld.rfr.index
                                                        }
                                                  }
                                                ]
                                        in
                                        { model
                                            | requestIdCount = model.requestIdCount + List.length newmsgs
                                            , editedField = Nothing
                                            , pendingMsgs = model.pendingMsgs ++ newmsgs
                                        }
                                    )
                        )
            )
        |> Maybe.withDefault model


update : Msg -> Model -> Time.Zone -> ( Model, Command )
update msg model zone =
    case msg of
        SelectField i ->
            ( { model
                | editedField =
                    if Maybe.map Tuple.first model.editedField == Just i then
                        Nothing

                    else
                        Dict.get i model.fields
                            |> Maybe.map
                                (\fl ->
                                    case fl.value of
                                        Just v ->
                                            ( i, Data.showFieldValue v )

                                        Nothing ->
                                            ( i, "" )
                                )
              }
            , None
            )

        EditField s ->
            ( { model | editedField = model.editedField |> Maybe.map (\( i, _ ) -> ( i, s )) }, None )

        EditUpdate ->
            let
                nm =
                    editUpdates model
            in
            ( nm
            , case List.head nm.pendingMsgs of
                Just pm ->
                    let
                        _ =
                            Debug.log "editupdate sending" pm
                    in
                    SendAutomatoMsg pm.automatoMsg pm.what

                Nothing ->
                    None
            )

        -- case editUpdates model of
        --     Just ( nm, send, pend ) ->
        --         ( { nm | editedField = Nothing, pendingMsgs = model.pendingMsgs ++ [ pend ] }
        --         , send
        --         )
        --     Nothing ->
        --         ( model, None )
        EditCancel ->
            ( { model | editedField = Nothing }, None )

        EditRefresh ->
            -- editUpdates model
            --     |> Maybe.map
            --         (\( nm, _, pend ) ->
            --             ( { nm | editedField = Nothing }
            --             , SendAutomatoMsg pend.automatoMsg pend.what
            --             )
            --         )
            --     |> Maybe.withDefault ( model, None )
            Debug.todo "" ( model, None )

        DonePress ->
            ( model, Done )

        Noop ->
            ( model, None )
