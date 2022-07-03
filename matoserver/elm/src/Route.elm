module Route exposing (Route(..), parseUrl, routeTitle, routeUrl)

import UUID exposing (UUID)
import Url exposing (Url)
import Url.Builder as UB
import Url.Parser as UP exposing ((</>))


type Route
    = AutomatoViewR Int
    | Top


routeTitle : String -> Route -> String
routeTitle appname route =
    case route of
        AutomatoViewR id ->
            "automatoview " ++ String.fromInt id

        Top ->
            appname


parseUrl : Url -> Maybe Route
parseUrl url =
    UP.parse
        (UP.oneOf
            [ UP.map AutomatoViewR <|
                UP.s
                    "automatoview"
                    </> UP.int
            , UP.map Top <| UP.top
            ]
        )
        url


routeUrl : Route -> String
routeUrl route =
    case route of
        Top ->
            UB.absolute [] []

        AutomatoViewR id ->
            UB.absolute [ "automatoview", String.fromInt id ] []
