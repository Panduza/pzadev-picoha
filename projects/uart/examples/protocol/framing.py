"""
==========================
Dumb codec for HA protocol
==========================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: February 2023
"""

import crcmod

from enum        import IntEnum
from dataclasses import dataclass

crc16 = crcmod.predefined.mkCrcFun("crc-ccitt-false")

class Code(IntEnum):
    Ping            = 0x0000,
    ItfType         = 0x0001,
    Version         = 0x0002,
    IdGet           = 0x0003,

    DataTX          = 0x1000,
    DataRXGet       = 0x1001,
    BaudSet         = 0x1002,
    BaudGet         = 0x1003,
    SetParity       = 0x1004,
    SetStopBit      = 0x1005,
    SetDataSz       = 0x1006,
    HWFlowControl   = 0x1007,
    ComErrStart     = 0x1008,
    ComErrSize      = 0x1009,

    DataRX          = 0xEFFF,
    Baud            = 0xEFFE,

    VersionResp     = 0xFEFF,
    ItfTypeResp     = 0xFEFE,
    IdResp          = 0xFEFD,

    Good            = 0xFFFF,
    ErrGeneric      = 0xFFFE,
    ErrCRC          = 0xFFFD,
    ErrUnknownCode  = 0xFFFC,
    ErrInvalidArgs  = 0xFFFB,
    ErrBusy         = 0xFFFA


class ItfType(IntEnum):
    Unknown = 0x00
    Uart    = 0x01

@dataclass
class MsgFrame:
    code: Code
    data: bytes

    @classmethod
    def from_bytes(cls, bb: bytes) -> "MsgFrame":
        if len(bb) < 4:
            raise ValueError("Invalid length (<4) for input buffer")

        # Compute and validate crc
        crc_frame = (bb[-2]<<8) | bb[-1]
        crc_real  = crc16(bb[:-2])

        if crc_real != crc_frame:
            raise AssertionError(f"Invalid CRC: {crc_real} != {crc_frame}")

        # Get code
        code = Code((bb[0] << 8) | bb[1])

        # Build frame
        return cls(
            code = code,
            data = bb[2:-2]
        )

    def buffer(self) -> bytes:
        start_buf = self.code.value.to_bytes(2, "big") + self.data
        crc       = crc16(start_buf).to_bytes(2, "big")

        return start_buf + crc

if __name__ == "__main__":
    in_data = bytes([0x01, 0x02, 0xFF, 0xFF, 0x81, 0x1B])

    frame = MsgFrame.from_bytes(in_data)
    print(frame)
    print(in_data)
    print(frame.buffer())
