module AutomatoListing exposing (..)

import AutomatoView
import Common
import Data
import Dialog as D
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
    = SelectPress Int
    | DonePress


type alias Model =
    { automatos : List Data.ListAutomato
    }


type Command
    = Selected Int
    | Done
    | None


init : List Data.ListAutomato -> Model
init automatos =
    { automatos = automatos }


view : Data.LoginData -> Util.Size -> Model -> Element Msg
view ld size model =
    let
        maxwidth =
            700

        titlemaxconst =
            85
    in
    E.el
        [ E.width E.fill
        , EBk.color TC.lightGrey
        ]
    <|
        E.column
            [ E.spacing TC.defaultSpacing
            , E.padding 8
            , E.width (E.maximum maxwidth E.fill)
            , E.centerX
            , EBk.color TC.lightGrey
            ]
            [ E.column
                [ E.padding 8
                , EBd.rounded 10
                , EBd.width 1
                , EBd.color TC.darkGrey
                , EBk.color TC.white
                , E.spacing TC.defaultSpacing
                ]
                [ E.table [ E.spacing 5, E.width E.fill, E.centerX ]
                    { data = model.automatos
                    , columns =
                        [ { header = E.none
                          , width =
                                -- E.fill
                                -- clipX doesn't work unless max width is here in px, it seems.
                                -- E.px <| min maxwidth size.width - titlemaxconst
                                E.px <| min maxwidth size.width - 32
                          , view =
                                \n ->
                                    E.row
                                        [ E.centerY
                                        , E.clipX
                                        , E.width E.fill
                                        ]
                                        [ E.link
                                            [ E.height <| E.px 30 ]
                                            { url =
                                                Route.routeUrl
                                                    (Route.AutomatoTimeR (Data.getAutomatoIdVal n.id)
                                                        (AutomatoTime.showViewMode AutomatoTime.Clonks)
                                                    )
                                            , label = E.text n.name
                                            }
                                        ]
                          }
                        ]
                    }
                ]
            ]


update : Msg -> Model -> Data.LoginData -> ( Model, Command )
update msg model ld =
    case msg of
        SelectPress id ->
            ( model
            , Selected id
            )

        NewPress ->
            ( model, New )

        DonePress ->
            ( model, Done )

        SettingsPress ->
            ( model, Settings )
