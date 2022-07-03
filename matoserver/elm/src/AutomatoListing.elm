module AutomatoListing exposing (..)

-- import Dialog as D

import Common
import Data
import Element as E exposing (Element)
import Element.Background as EBk
import Element.Border as EBd
import Element.Font as EF
import Element.Input as EI
import Element.Region
import Route
import TangoColors as TC
import TcCommon as TC
import Toop
import Util
import WindowKeys as WK


type Msg
    = SelectPress Data.AutomatoId
    | DonePress


type alias Model =
    { automatos : List Data.ListAutomato
    }


type Command
    = Selected Data.AutomatoId
    | Done
    | None


init : List Data.ListAutomato -> Model
init automatos =
    { automatos = automatos }


view : Util.Size -> Model -> Element Msg
view size model =
    let
        maxwidth =
            700

        titlemaxconst =
            85
    in
    E.none


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        SelectPress id ->
            ( model
            , Selected id
            )

        DonePress ->
            ( model, Done )
