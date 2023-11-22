"""
=========================================
HA protocol common requests and responses
=========================================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

from dataclasses import dataclass
from .framing    import MsgFrame, Code, ItfType


# ┌────────────────────────────────────────┐
# │ Common requests                        │
# └────────────────────────────────────────┘

@dataclass
class RequestPing:
    # No data

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.Ping, data = bytes([]))

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "RequestPing":
        if frame.code != Code.Ping:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls()


@dataclass
class RequestItfType:
    # No data

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.ItfType, data = bytes([]))

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "RequestItfType":
        if frame.code != Code.ItfType:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls()


@dataclass
class RequestVersion:
    # No data

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.Version, data = bytes([]))

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "RequestVersion":
        if frame.code != Code.Version:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls()


@dataclass
class RequestIdGet:
    # No data

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.IdGet, data = bytes([]))

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "RequestIdGet":
        if frame.code != Code.IdGet:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls()


# ┌────────────────────────────────────────┐
# │ Common responses                       │
# └────────────────────────────────────────┘

@dataclass
class ResponseVersion:
    version: str

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.VersionResp, data = self.version.encode("utf-8"))

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "ResponseVersion":
        if frame.code != Code.VersionResp:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(version = frame.data.decode("utf-8"))


@dataclass
class ResponseItfType:
    itfType: ItfType

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.ItfTypeResp, data = self.itfType.value)

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "ResponseItfType":
        if frame.code != Code.ItfTypeResp:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(itfType = ItfType(frame.data[0]))


@dataclass
class ResponseId:
    id_bytes: bytes

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.IdResp, data = self.id_bytes)

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "ResponseId":
        if frame.code != Code.IdResp:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(id_bytes = frame.data)


# ┌────────────────────────────────────────┐
# │ Status codes                           │
# └────────────────────────────────────────┘

@dataclass
class StatusGood:
    # No data.
    
    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.Good, data=bytes([]))

    @classmethod
    def from_frame(cls, frame: MsgFrame) -> "StatusGood":
        if frame.code != Code.Good:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls()


@dataclass
class StatusErrGeneric:
    msg: str

    def to_frame(self) -> MsgFrame:
        return MsgFrame(code = Code.ErrGeneric, data=self.msg.encode("utf-8"))

    @classmethod
    def from_frame(self, frame: MsgFrame) -> "StatusErrGeneric":
        if frame.code != Code.ErrGeneric:
            raise ValueError(f"Invalid code for {cls}: {frame.code}")
        return cls(msg = frame.data.decode("utf-8"))

# TODO: ErrInvalidArgs, ErrUnknownCode, ErrBusy, ErrCRC
