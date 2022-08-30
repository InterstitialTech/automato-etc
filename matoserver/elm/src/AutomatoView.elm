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
import MsCommon as MS
import Payload exposing (AutomatoMsg)
import Round as R
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
    , what : Maybe String
    }


type alias Model =
    { automatoinfo : Payload.RemoteInfo
    , id : Int
    , temperature : Maybe Float
    , humidity : Maybe Float
    , fields : Dict Int Field
    , pendingMsgs : List PendingMsg
    , editedField : Maybe ( Int, String )
    }


type Command
    = Done
    | ShowError String
    | SendAutomatoMsg AutomatoMsg (Maybe String)
    | None


headerStyle : List (E.Attribute msg)
headerStyle =
    [ EF.bold ]


init : Int -> Payload.RemoteInfo -> ( Model, Command )
init id ai =
    let
        pendingMsgs =
            [ { automatoMsg = { id = id, message = Payload.PeReadtemperature }, what = Nothing }
            , { automatoMsg = { id = id, message = Payload.PeReadhumidity }, what = Nothing }
            ]
                ++ (List.range 0 (ai.fieldcount - 1)
                        |> List.map
                            (\i ->
                                { automatoMsg = { id = id, message = Payload.PeReadfield { index = i } }
                                , what = Nothing
                                }
                            )
                   )

        model =
            { automatoinfo = ai
            , id = id
            , temperature = Nothing
            , humidity = Nothing
            , fields = Dict.empty
            , pendingMsgs = List.drop 1 pendingMsgs
            , editedField = Nothing
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


onAutomatoMsg : AutomatoMsg -> Maybe String -> Model -> ( Model, Command )
onAutomatoMsg am mbwhat model =
    let
        nm =
            case am.message of
                Payload.PeReadfieldreply rfr ->
                    { model
                        | fields = Dict.insert rfr.index { rfr = rfr, value = Nothing } model.fields
                        , pendingMsgs =
                            model.pendingMsgs
                                ++ [ { automatoMsg = { id = model.id, message = Payload.PeReadmem <| readField rfr }
                                     , what =
                                        JE.encode 0 (JE.int rfr.index)
                                            |> Just
                                     }
                                   ]
                    }

                Payload.PeReadmemreply rmr ->
                    mbwhat
                        |> Maybe.andThen
                            (\what ->
                                JD.decodeString JD.int what
                                    |> Result.toMaybe
                            )
                        |> Maybe.andThen
                            (\i ->
                                Dict.get i model.fields
                            )
                        |> Maybe.andThen
                            (\field ->
                                Data.decodeValue field.rfr.format rmr
                                    |> Maybe.map
                                        (\val ->
                                            { model
                                                | fields =
                                                    Dict.insert field.rfr.index
                                                        { rfr = field.rfr
                                                        , value = Just val
                                                        }
                                                        model.fields
                                            }
                                        )
                            )
                        |> Maybe.withDefault model

                Payload.PeReadtemperaturereply f ->
                    { model
                        | temperature = Just f
                    }

                Payload.PeReadhumidityreply f ->
                    { model
                        | humidity = Just f
                    }

                _ ->
                    model
    in
    ( { nm | pendingMsgs = List.drop 1 nm.pendingMsgs }
    , case List.head nm.pendingMsgs of
        Just pm ->
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
                                                        editUpdates model
                                                            |> Util.isJust
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
                                                        , E.spacing 8
                                                        ]
                                                        [ E.text <| Data.showFieldValue v ]
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
                                                    [ E.text <| Data.showFieldValue v ]

                                    Nothing ->
                                        E.none
                      }
                    ]
                }
            ]
        , EI.button Common.buttonStyle { onPress = Just DonePress, label = E.text "done" }
        ]


editUpdates : Model -> Maybe ( Command, PendingMsg )
editUpdates model =
    model.editedField
        |> Maybe.andThen
            (\( efi, efs ) ->
                Dict.get efi model.fields
                    |> Maybe.map
                        (\fld ->
                            Data.strToFieldValue fld.rfr efs
                                |> Maybe.map
                                    (\fv ->
                                        ( SendAutomatoMsg
                                            { id = model.id
                                            , message =
                                                Payload.PeWritemem
                                                    { address = fld.rfr.offset
                                                    , data = Data.encodeFieldValue fv
                                                    }
                                            }
                                            Nothing
                                        , { automatoMsg =
                                                { id = model.id
                                                , message = Payload.PeReadmem <| readField fld.rfr
                                                }
                                          , what =
                                                JE.encode 0 (JE.int fld.rfr.index)
                                                    |> Just
                                          }
                                        )
                                    )
                        )
            )
        |> Maybe.withDefault Nothing


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
            case editUpdates model of
                Just ( send, pend ) ->
                    ( { model | editedField = Nothing, pendingMsgs = model.pendingMsgs ++ [ pend ] }
                    , send
                    )

                Nothing ->
                    ( model, None )

        EditCancel ->
            ( { model | editedField = Nothing }, None )

        EditRefresh ->
            editUpdates model
                |> Maybe.map
                    (\( _, pend ) ->
                        ( { model | editedField = Nothing }
                        , SendAutomatoMsg pend.automatoMsg pend.what
                        )
                    )
                |> Maybe.withDefault ( model, None )

        DonePress ->
            ( model, Done )

        Noop ->
            ( model, None )
