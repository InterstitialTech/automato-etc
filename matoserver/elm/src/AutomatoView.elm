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


type alias Model =
    { automatoinfo : Payload.RemoteInfo
    , id : Int
    , fields : Dict Int Payload.ReadFieldReply
    }


type Command
    = Done
    | ShowError String
    | SendAutomatoMsg AutomatoMsg
    | None


headerStyle : List (E.Attribute msg)
headerStyle =
    [ EF.bold ]


init : Int -> Payload.RemoteInfo -> ( Model, Command )
init id ai =
    ( { automatoinfo = ai
      , id = id
      , fields = Dict.empty
      }
    , if ai.fieldcount > 0 then
        SendAutomatoMsg { id = id, message = Payload.PeReadfield { index = 0 } }

      else
        None
    )


onAutomatoMsg : AutomatoMsg -> Model -> ( Model, Command )
onAutomatoMsg am model =
    case am.message of
        Payload.PeReadfieldreply info ->
            ( { model | fields = Dict.insert info.index info model.fields }
            , if info.index < model.automatoinfo.fieldcount - 1 then
                SendAutomatoMsg { id = model.id, message = Payload.PeReadfield { index = info.index + 1 } }

              else
                None
            )

        _ ->
            ( model, None )


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
        , E.column [ E.padding 15, E.spacing 15 ]
            (model.fields
                |> Dict.values
                |> List.map
                    (\fld ->
                        E.column []
                            [ E.text <| "index" ++ String.fromInt fld.index
                            , E.text <| "offset" ++ String.fromInt fld.offset
                            , E.text <| "length" ++ String.fromInt fld.length
                            , E.text <| "format" ++ String.fromInt fld.format
                            , E.text <| "name" ++ String.fromList (List.map Char.fromCode fld.name)
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
