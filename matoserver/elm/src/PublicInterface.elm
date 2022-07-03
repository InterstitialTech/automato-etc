module PublicInterface exposing
    ( SendMsg(..)
    , ServerResponse(..)
    , encodeSendMsg
    , serverResponseDecoder
    , showServerResponse
    )

import Data exposing (AutomatoId)
import Json.Decode as JD
import Json.Encode as JE


type SendMsg
    = GetAutomatoList
    | GetAutomatoInfo Data.AutomatoId



-- | GetAutomatoDetail Int


type ServerResponse
    = ServerError String
    | AutomatoList (List Data.ListAutomato)



-- | AutomatoDetail Data.ListAutomato


showServerResponse : ServerResponse -> String
showServerResponse sr =
    case sr of
        ServerError _ ->
            "ServerError"

        AutomatoList _ ->
            "AutomatoList"


encodeSendMsg : SendMsg -> JE.Value
encodeSendMsg sm =
    case sm of
        GetAutomatoList ->
            JE.object
                [ ( "what", JE.string "GetAutomatoList" )
                ]

        GetAutomatoInfo aid ->
            JE.object
                [ ( "what", JE.string "GetAutomatoInfo" )
                , ( "data", Data.getAutomatoIdVal aid |> JE.int )
                ]


serverResponseDecoder : JD.Decoder ServerResponse
serverResponseDecoder =
    JD.at [ "what" ]
        JD.string
        |> JD.andThen
            (\what ->
                case what of
                    "server error" ->
                        JD.map ServerError (JD.at [ "content" ] JD.string)

                    "projecttime" ->
                        JD.map AutomatoList (JD.at [ "content" ] (JD.list Data.decodeListAutomato))

                    wat ->
                        JD.succeed
                            (ServerError ("invalid 'what' from server: " ++ wat))
            )
