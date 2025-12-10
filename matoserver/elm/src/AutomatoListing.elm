module AutomatoListing exposing (..)

import Common
import Data
import Element as E exposing (Element)
import Element.Background as EBk
import Element.Border as EBd
import Element.Font as EF
import Element.Input as EI
import Element.Region
import MsCommon as MC
import Payload
import Route
import TangoColors as TC
import Toop
import Util
import WindowKeys as WK


type Msg
    = SelectPress Payload.AutomatoId
    | DonePress


type alias Model =
    { automatos : List Data.ListAutomato
    }


type Command
    = Selected Payload.AutomatoId
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
    E.column [] <|
        E.text "automatos:"
            :: List.map
                (\la ->
                    E.link MC.myLinkStyle
                        { url = Route.AutomatoViewR la.id |> Route.routeUrl
                        , label = E.text <| Data.showAutomatoId la.id
                        }
                )
                model.automatos


update : Msg -> Model -> ( Model, Command )
update msg model =
    case msg of
        SelectPress id ->
            ( model
            , Selected id
            )

        DonePress ->
            ( model, Done )
