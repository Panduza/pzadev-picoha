"""
===================
Simple frame parser
===================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

from .framing import Code, MsgFrame

from .        import common, uart

__CODE_DICT = {
    Code.Ping:          common.RequestPing,
    Code.ItfType:       common.RequestItfType,
    Code.Version:       common.RequestVersion,
    Code.IdGet:         common.RequestIdGet,

    Code.DataTX:        uart.UartRequestDataTX,
    # Code.DataRXGet:     uart.UartRequestDataRXGet,
    # Code.BaudSet:       uart.UartRequestBaudSet,
    # Code.BaudGet:       uart.UartRequestBaudGet,
    # Code.SetParity:     uart.UartRequestSetParity,
    # Code.SetStopBit:    uart.UartRequestSetStopBit,
    # Code.SetDataSz:     uart.UartRequestSetDataSz,
    # Code.HWFlowControl: uart.UartRequestHWFlowControl,
    # Code.ComErrStart:   uart.UartRequestComErrStart,
    # Code.ComErrSize:    uart.UartRequestComErrSize,

    # Code.DataRX:        uart.UartResponseDataRX,
    # Code.Baud:          uart.UartResponseBaud,

    Code.VersionResp:   common.ResponseVersion,
    Code.ItfTypeResp:   common.ResponseItfType,

    Code.Good:          common.StatusGood,
    Code.ErrGeneric:    common.StatusErrGeneric,
    # Code.ErrCRC: TODO
    # Code.ErrUnknownCode: TODO
    # Code.ErrInvalidArgs: TODO
    # Code.ErrBusy: TODO
}

def from_frame(frame: MsgFrame):
    cls = __CODE_DICT.get(frame.code, None)

    if cls is None:
        raise KeyError(f"Unsupported code: {frame.code}")

    return cls.from_frame(frame)
