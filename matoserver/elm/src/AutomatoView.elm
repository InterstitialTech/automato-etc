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
            }
    in
    ( model
    , case List.head pendingMsgs of
        Just pm ->
            SendAutomatoMsg pm.automatoMsg pm.what

        Nothing ->
            None
    )



-- type FieldValue
--     = FvChar Char
--     | FvFloat Float
--     | FvUint8 Int
--     | FvUint16 Int
--     | FvUint32 Int
--     | FvInt8 Int
--     | FvInt16 Int
--     | FvInt32 Int
--     | FvOther (List Int)


showFieldValue : Data.FieldValue -> Element a
showFieldValue fv =
    case fv of
        Data.FvChar s ->
            E.text <| s

        Data.FvFloat f ->
            E.text <| String.fromFloat f

        Data.FvUint8 i ->
            E.text <| String.fromInt i

        Data.FvUint16 i ->
            E.text <| String.fromInt i

        Data.FvUint32 i ->
            E.text <| String.fromInt i

        Data.FvInt8 i ->
            E.text <| String.fromInt i

        Data.FvInt16 i ->
            E.text <| String.fromInt i

        Data.FvInt32 i ->
            E.text <| String.fromInt i

        Data.FvOther li ->
            E.text <| String.fromList (List.map Char.fromCode li)


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
        [ E.text <| "protocol version: " ++ String.fromFloat model.automatoinfo.protoversion
        , E.text <| "macAddress: " ++ String.fromInt model.automatoinfo.macAddress
        , E.text <| "datalen: " ++ String.fromInt model.automatoinfo.datalen
        , E.text <| "fieldcount: " ++ String.fromInt model.automatoinfo.fieldcount
        , E.text <|
            "temperature: "
                ++ (case model.temperature of
                        Just f ->
                            String.fromFloat f

                        Nothing ->
                            "?"
                   )
        , E.text <|
            "humidity: "
                ++ (case model.humidity of
                        Just f ->
                            String.fromFloat f

                        Nothing ->
                            "?"
                   )
        , E.column [ E.padding 15, E.spacing 15 ]
            (model.fields
                |> Dict.values
                |> List.map
                    (\fld ->
                        E.column []
                            [ E.text <| "index" ++ String.fromInt fld.rfr.index
                            , E.text <| "offset" ++ String.fromInt fld.rfr.offset
                            , E.text <| "length" ++ String.fromInt fld.rfr.length
                            , E.text <| "format" ++ String.fromInt fld.rfr.format
                            , E.text <| "name" ++ String.fromList (List.map Char.fromCode fld.rfr.name)
                            , case fld.value of
                                Just v ->
                                    E.el [ EF.bold ] (showFieldValue v)

                                Nothing ->
                                    E.none
                            ]
                    )
            )
        , EI.button Common.buttonStyle { onPress = Just DonePress, label = E.text "done" }
        ]


update : Msg -> Model -> Time.Zone -> ( Model, Command )
update msg model zone =
    case msg of
        DonePress ->
            ( model, Done )

        Noop ->
            ( model, None )
