module Data exposing (AutomatoId(..), FieldValue(..), ListAutomato, decodeListAutomato, decodeValue, getAutomatoIdVal, makeAutomatoId)

import Bytes
import Bytes.Decode
import Bytes.Encode
import Json.Decode as JD
import Json.Encode as JE
import Payload
import Url.Builder as UB
import Util exposing (andMap)


type alias ListAutomato =
    { id : AutomatoId
    }



-- type FieldFormat
--     = FfChar
--     | FfFloat
--     | FfUint8
--     | FfUint16
--     | FfUint32
--     | FfInt8
--     | FfInt16
--     | FfInt32
--     | FfOther
-- getFormat : Int -> Maybe FieldFormat
-- getFormat code =
--     case code of
--         0 ->
--             Just FfChar
--         1 ->
--             Just FfFloat
--         2 ->
--             Just FfUint8
--         3 ->
--             Just FfUint16
--         4 ->
--             Just FfUint32
--         5 ->
--             Just FfInt8
--         6 ->
--             Just FfInt16
--         7 ->
--             Just FfInt32
--         8 ->
--             Just FfOther
--         _ ->
--             Nothing


type FieldValue
    = FvChar Char
    | FvFloat Float
    | FvUint8 Int
    | FvUint16 Int
    | FvUint32 Int
    | FvInt8 Int
    | FvInt16 Int
    | FvInt32 Int
    | FvOther (List Int)


decodeValue : Int -> Payload.ReadmemReply -> Maybe FieldValue
decodeValue format rmr =
    let
        bytes =
            Bytes.Encode.encode <|
                Bytes.Encode.sequence
                    (List.map Bytes.Encode.unsignedInt8 rmr.data)
    in
    case format of
        0 ->
            List.head rmr.data
                |> Maybe.map (\i -> FvChar <| Char.fromCode i)

        1 ->
            Bytes.Decode.decode (Bytes.Decode.float32 Bytes.BE) bytes
                |> Maybe.map FvFloat

        2 ->
            Bytes.Decode.decode Bytes.Decode.unsignedInt8 bytes
                |> Maybe.map FvUint8

        3 ->
            Bytes.Decode.decode (Bytes.Decode.unsignedInt16 Bytes.BE) bytes
                |> Maybe.map FvUint16

        4 ->
            Bytes.Decode.decode (Bytes.Decode.unsignedInt32 Bytes.BE) bytes
                |> Maybe.map FvUint32

        5 ->
            Bytes.Decode.decode Bytes.Decode.signedInt8 bytes
                |> Maybe.map FvInt8

        6 ->
            Bytes.Decode.decode (Bytes.Decode.signedInt16 Bytes.BE) bytes
                |> Maybe.map FvInt16

        7 ->
            Bytes.Decode.decode (Bytes.Decode.signedInt32 Bytes.BE) bytes
                |> Maybe.map FvInt32

        8 ->
            Just <| FvOther rmr.data

        _ ->
            Nothing



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
