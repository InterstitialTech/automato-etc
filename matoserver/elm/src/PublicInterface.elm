module PublicInterface exposing (showServerResponse)

import Messages exposing (PublicMessage(..), ServerResponse(..))



-- type SendMsg
--     = GetAutomatoList
--     | SendAutomatoMsg Messages.AutomatoMsg
-- type ServerResponse
--     = ServerError String
--     | AutomatoList (List Data.ListAutomato)
--     | AutomatoMsg Messages.AutomatoMsg
--     | SerialError SE.Error


showServerResponse : ServerResponse -> String
showServerResponse sr =
    case sr of
        SrAutomatos _ ->
            "SrAutomatos"

        SrAutomatoMsg _ ->
            "SrAutomatoMsg"

        SrSerialPorts _ ->
            "SrSerialPorts"

        SrSerialError _ ->
            "SrSerialError"

        SrGenericError _ ->
            "SrGenericError"



-- encodeSendMsg : SendMsg -> JE.Value
-- encodeSendMsg sm =
--     case sm of
--         GetAutomatoList ->
--             JE.object
--                 [ ( "what", JE.string "GetAutomatoList" )
--                 ]
--         SendAutomatoMsg msg ->
--             JE.object
--                 [ ( "what", JE.string "AutomatoMsg" )
--                 , ( "data", Messages.automatoMsgEncoder msg )
--                 ]
-- serverResponseDecoder : JD.Decoder ServerResponse
-- serverResponseDecoder =
--     JD.at [ "what" ]
--         JD.string
--         |> JD.andThen
--             (\what ->
--                 case what of
--                     "server error" ->
--                         JD.map ServerError (JD.at [ "content" ] JD.string)
--                     "automatos" ->
--                         JD.at [ "content" ] (JD.list Payload.automatoIdDecoder)
--                             |> JD.map (List.map (\id -> { id = id }))
--                             |> JD.map AutomatoList
--                     "automatomsg" ->
--                         JD.at [ "content" ] Messages.automatoMsgDecoder
--                             |> JD.map AutomatoMsg
--                     "serial error" ->
--                         JD.at [ "content" ] SE.errorDecoder
--                             |> JD.map SerialError
--                     wat ->
--                         JD.succeed
--                             (ServerError ("invalid 'what' from server: " ++ wat))
--             )
