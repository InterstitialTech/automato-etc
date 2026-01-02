module PublicInterface exposing (showServerResponse)

import Messages exposing (PublicMessage(..), ServerResponse(..))


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

        SrSerialPortOpened _ ->
            "SrSerialPortOpened"
