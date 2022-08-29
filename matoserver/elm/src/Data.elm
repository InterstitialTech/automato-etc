module Data exposing (AutomatoId(..), FieldValue(..), ListAutomato, decodeListAutomato, decodeValue, encodeFieldValue, getAutomatoIdVal, makeAutomatoId, showFieldValue, strToFieldValue)

import Bytes
import Bytes.Decode
import Bytes.Encode
import Bytes.Extra as BE
import Json.Decode as JD
import Json.Encode as JE
import Payload
import Url.Builder as UB
import Util exposing (andMap)


type alias ListAutomato =
    { id : AutomatoId
    }


type FieldValue
    = FvChar String
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
            List.map Char.fromCode rmr.data
                |> String.fromList
                |> FvChar
                |> Just

        1 ->
            Bytes.Decode.decode (Bytes.Decode.float32 Bytes.LE) bytes
                |> Maybe.map FvFloat

        2 ->
            Bytes.Decode.decode Bytes.Decode.unsignedInt8 bytes
                |> Maybe.map FvUint8

        3 ->
            Bytes.Decode.decode (Bytes.Decode.unsignedInt16 Bytes.LE) bytes
                |> Maybe.map FvUint16

        4 ->
            Bytes.Decode.decode (Bytes.Decode.unsignedInt32 Bytes.LE) bytes
                |> Maybe.map FvUint32

        5 ->
            Bytes.Decode.decode Bytes.Decode.signedInt8 bytes
                |> Maybe.map FvInt8

        6 ->
            Bytes.Decode.decode (Bytes.Decode.signedInt16 Bytes.LE) bytes
                |> Maybe.map FvInt16

        7 ->
            Bytes.Decode.decode (Bytes.Decode.signedInt32 Bytes.LE) bytes
                |> Maybe.map FvInt32

        8 ->
            Just <| FvOther rmr.data

        _ ->
            Nothing


showFieldValue : FieldValue -> String
showFieldValue fv =
    case fv of
        FvChar s ->
            s

        FvFloat f ->
            String.fromFloat f

        FvUint8 i ->
            String.fromInt i

        FvUint16 i ->
            String.fromInt i

        FvUint32 i ->
            String.fromInt i

        FvInt8 i ->
            String.fromInt i

        FvInt16 i ->
            String.fromInt i

        FvInt32 i ->
            String.fromInt i

        FvOther li ->
            String.fromList (List.map Char.fromCode li)


encodeFieldValue : FieldValue -> List Int
encodeFieldValue fv =
    let
        bytes =
            case fv of
                FvChar s ->
                    Bytes.Encode.encode <|
                        Bytes.Encode.sequence
                            (List.map (Char.toCode >> Bytes.Encode.unsignedInt8) (String.toList s))

                FvFloat f ->
                    Bytes.Encode.encode (Bytes.Encode.float32 Bytes.LE f)

                FvUint8 i ->
                    Bytes.Encode.encode (Bytes.Encode.unsignedInt8 i)

                FvUint16 i ->
                    Bytes.Encode.encode (Bytes.Encode.unsignedInt16 Bytes.LE i)

                FvUint32 i ->
                    Bytes.Encode.encode (Bytes.Encode.unsignedInt32 Bytes.LE i)

                FvInt8 i ->
                    Bytes.Encode.encode (Bytes.Encode.signedInt8 i)

                FvInt16 i ->
                    Bytes.Encode.encode (Bytes.Encode.signedInt16 Bytes.LE i)

                FvInt32 i ->
                    Bytes.Encode.encode (Bytes.Encode.signedInt32 Bytes.LE i)

                FvOther li ->
                    Bytes.Encode.encode <|
                        Bytes.Encode.sequence
                            (List.map Bytes.Encode.unsignedInt8 li)
    in
    BE.toByteValues bytes


strToFieldValue : Payload.ReadFieldReply -> String -> Maybe FieldValue
strToFieldValue rfr str =
    case rfr.format of
        0 ->
            Just <| FvChar (String.left rfr.length str)

        1 ->
            String.toFloat str
                |> Maybe.map FvFloat

        2 ->
            String.toInt str
                |> Maybe.andThen
                    (\i ->
                        if i >= 0 && i < 256 then
                            Just <| FvUint8 i

                        else
                            Nothing
                    )

        3 ->
            String.toInt str
                |> Maybe.andThen
                    (\i ->
                        if i >= 0 && i < 65536 then
                            Just <| FvUint16 i

                        else
                            Nothing
                    )

        4 ->
            String.toInt str
                |> Maybe.andThen
                    (\i ->
                        if i >= 0 && i < 4294967296 then
                            Just <| FvUint32 i

                        else
                            Nothing
                    )

        5 ->
            String.toInt str
                |> Maybe.andThen
                    (\i ->
                        if i >= -128 && i <= 127 then
                            Just <| FvInt8 i

                        else
                            Nothing
                    )

        6 ->
            String.toInt str
                |> Maybe.andThen
                    (\i ->
                        if i >= -32768 && i < 32768 then
                            Just <| FvInt16 i

                        else
                            Nothing
                    )

        7 ->
            String.toInt str
                |> Maybe.andThen
                    (\i ->
                        if i >= -2147483648 && i < 2147483648 then
                            Just <| FvInt32 i

                        else
                            Nothing
                    )

        8 ->
            -- deal with this later.
            Nothing

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
