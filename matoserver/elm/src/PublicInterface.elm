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
import Payload


type SendMsg
    = GetAutomatoList
    | SendAutomatoMsg Payload.AutomatoMsg


type ServerResponse
    = ServerError String
    | AutomatoList (List Data.ListAutomato)
    | AutomatoMsg Payload.AutomatoMsg


showServerResponse : ServerResponse -> String
showServerResponse sr =
    case sr of
        ServerError _ ->
            "ServerError"

        AutomatoList _ ->
            "AutomatoList"

        AutomatoMsg _ ->
            "AutomatoMsg"


encodeSendMsg : SendMsg -> JE.Value
encodeSendMsg sm =
    case sm of
        GetAutomatoList ->
            JE.object
                [ ( "what", JE.string "GetAutomatoList" )
                ]

        SendAutomatoMsg msg ->
            JE.object
                [ ( "what", JE.string "SendAutomatoMsg" )
                , ( "data", Payload.automatoMsgEncoder msg )
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

                    "automatos" ->
                        JD.at [ "content" ] (JD.list JD.int)
                            |> JD.map (List.map (\id -> { id = Data.makeAutomatoId id }))
                            |> JD.map AutomatoList

                    wat ->
                        JD.succeed
                            (ServerError ("invalid 'what' from server: " ++ wat))
            )
