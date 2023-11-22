"""
=========================================
HA protocol requests and reponse for UART
=========================================

"""

from dataclasses import dataclass
from enum        import IntEnum

from .framing    import (MsgFrame, Code)


# ┌────────────────────────────────────────┐
# │ Specific types                         │
# └────────────────────────────────────────┘

class HWFlowControlState(IntEnum):
    Disable         = 0
    Enable          = 1


# ┌────────────────────────────────────────┐
# │ UART requests                          │
# └────────────────────────────────────────┘

@dataclass
class UartRequestDataTX:
    tx_data: []

    def to_frame(self) -> MsgFrame:
        return MsgFrame(
            code = Code.DataTX,
            data = bytes(self.tx_data)
        )

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "UartRequestDataTX":
        if frame.code != Code.DataTX:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(tx_data = frame.data[0])


# @dataclass
# class GpioRequestDirGet:
#     pin: int

#     def to_frame(self) -> MsgFrame:
#         return MsgFrame(
#             code = Code.GpioDirGet,
#             data = bytes([self.pin])
#         )

#     @classmethod
#     def from_frame(cls, frame: MsgFrame) -> "GpioRequestDirGet":
#         if frame.code != Code.GpioDirGet:
#             raise ValueError(f"Invalid code for {cls}: {frame.code}")
#         return cls(pin = frame.data[0])


# @dataclass
# class GpioRequestWrite:
#     pin: int
#     value: GpioValue

#     def to_frame(self) -> MsgFrame:
#         return MsgFrame(
#             code = Code.GpioWrite,
#             data = bytes([self.pin, self.value.value])
#         )

#     @classmethod
#     def from_frame(cls, frame: MsgFrame) -> "GpioRequestWrite":
#         if frame.code != Code.GpioWrite:
#             raise ValueError(f"Invalid code for {cls}: {frame.code}")
#         return cls(pin = frame.data[0], value = GpioValue(frame.data[1]))

# @dataclass
# class GpioRequestRead:
#     pin: int

#     def to_frame(self) -> MsgFrame:
#         return MsgFrame(
#             code = Code.GpioRead,
#             data = bytes([self.pin, self.value.value])
#         )

#     @classmethod
#     def from_frame(cls, frame: MsgFrame) -> "GpioRequestRead":
#         if frame.code != Code.GpioRead:
#             raise ValueError(f"Invalid code for {cls}: {frame.code}")
#         return cls(pin = frame.data[0])

# # ┌────────────────────────────────────────┐
# # │ GPIO responses                         │
# # └────────────────────────────────────────┘

# @dataclass
# class GpioResponseValue:
#     pin: int
#     value: GpioValue

#     def to_frame(self) -> MsgFrame:
#         return MsgFrame(
#             code = Code.GpioValue,
#             data = bytes([self.pin, self.value.value])
#         )

#     @classmethod
#     def from_frame(cls, frame: MsgFrame) -> "GpioResponseValue":
#         if frame.code != Code.GpioValue:
#             raise ValueError(f"Invalid code for {cls}: {frame.code}")
#         return cls(pin = frame.data[0], value = GpioValue(frame.data[1]))


# @dataclass
# class GpioResponseDir:
#     pin: int
#     value: GpioDir

#     def to_frame(self) -> MsgFrame:
#         return MsgFrame(
#             code = Code.GpioDir,
#             data = bytes([self.pin, self.value.value])
#         )

#     @classmethod
#     def from_frame(cls, frame: MsgFrame) -> "GpioResponseDir":
#         if frame.code != Code.GpioValue:
#             raise ValueError(f"Invalid code for {cls}: frame.code")

#         return cls(pin = frame.data[0], value = GpioDir(frame.data[1]))
