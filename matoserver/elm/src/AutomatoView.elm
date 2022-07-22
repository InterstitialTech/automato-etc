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
import Payload
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
    }


type Command
    = Done
    | ShowError String
    | None


headerStyle : List (E.Attribute msg)
headerStyle =
    [ EF.bold ]


init : Time.Zone -> Int -> Payload.RemoteInfo -> Model
init zone id ai =
    { automatoinfo = ai
    , id = id
    }


view : Util.Size -> Time.Zone -> Model -> Element Msg
view size zone model =
    let
        maxwidth =
            700

        titlemaxconst =
            85
    in
    E.none


update : Msg -> Model -> Time.Zone -> ( Model, Command )
update msg model zone =
    case msg of
        DonePress ->
            ( model, Done )

        Noop ->
            ( model, None )
