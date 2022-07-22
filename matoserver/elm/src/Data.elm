module Data exposing (AutomatoId(..), ListAutomato, decodeListAutomato, getAutomatoIdVal, makeAutomatoId)

import Json.Decode as JD
import Json.Encode as JE
import Url.Builder as UB
import Util exposing (andMap)


type alias ListAutomato =
    { id : AutomatoId
    }



-- type alias RemoteInfo =
--     { protoversion : Float
--     , mac_address : String
--     , datalen : Int
--     , fieldcount : Int
--     }
-- type alias AutomatoInfo =
--     { id : AutomatoId
--     , remoteinfo : RemoteInfo
--     }
-------------------------------------------
-- Id types.  They're all ints underneath.
-------------------------------------------


type AutomatoId
    = AutomatoId Int


makeAutomatoId : Int -> AutomatoId
makeAutomatoId i =
    AutomatoId i


getAutomatoIdVal : AutomatoId -> Int
getAutomatoIdVal uid =
    case uid of
        AutomatoId i ->
            i



----------------------------------------
-- Json encoders/decoders
----------------------------------------


decodeListAutomato : JD.Decoder ListAutomato
decodeListAutomato =
    JD.succeed ListAutomato
        |> andMap (JD.field "id" JD.int |> JD.map makeAutomatoId)



-- decodeRemoteInfo : JD.Decoder RemoteInfo
-- decodeRemoteInfo =
--     JD.succeed RemoteInfo
--         |> andMap (JD.field "protoversion" JD.float)
--         |> andMap (JD.field "mac_address" JD.string)
--         |> andMap (JD.field "datalen" JD.int)
--         |> andMap (JD.field "fieldcount" JD.int)
-- |> andMap (JD.field "name" JD.string)
-- decodeProject : JD.Decoder Project
-- decodeProject =
--     JD.succeed Project
--         |> andMap (JD.field "id" JD.int |> JD.map makeProjectId)
--         |> andMap (JD.field "name" JD.string)
--         |> andMap (JD.field "description" JD.string)
--         |> andMap (JD.field "public" JD.bool)
--         |> andMap (JD.field "rate" <| JD.maybe JD.float)
--         |> andMap (JD.field "currency" <| JD.maybe JD.string)
--         |> andMap (JD.field "createdate" JD.int)
--         |> andMap (JD.field "changeddate" JD.int)
