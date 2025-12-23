module AutomatoListing exposing (..)

import Data
import Element as E exposing (Element)
import MsCommon as MC
import Payload
import Route
import Util


type Msg
    = SelectPress Payload.AutomatoId
    | DonePress


type alias Model =
    { automatos : List Payload.AutomatoId
    }


type Command
    = Selected Payload.AutomatoId
    | Done
    | None


init : List Payload.AutomatoId -> Model
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
                        { url = Route.AutomatoViewR la |> Route.routeUrl
                        , label = E.text <| Data.showAutomatoId la
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
