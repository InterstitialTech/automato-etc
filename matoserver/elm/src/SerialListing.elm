module SerialListing exposing (..)

import Data
import Element as E exposing (Element)
import Element.Input as EI
import Messages exposing (SerialPortInfo)
import MsCommon as MC
import Payload
import Route
import Util


type Msg
    = SelectPress Int
    | DonePress


type alias Model =
    { ports : List SerialPortInfo
    }


type Command
    = Selected SerialPortInfo
    | Done
    | None


init : List SerialPortInfo -> Model
init ports =
    { ports = ports }


view : Model -> Element Msg
view model =
    E.column [] <|
        E.text "serial ports:"
            :: List.indexedMap
                (\i la ->
                    EI.button [] { onPress = Just (SelectPress i), label = E.text la.portName }
                )
                model.ports


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        SelectPress i ->
            case List.head (List.drop i model.ports) of
                Just s ->
                    ( model
                    , Selected s
                    )

                Nothing ->
                    ( model, None )

        DonePress ->
            ( model, Done )
