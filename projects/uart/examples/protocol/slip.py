"""
=================
Simple SLIP codec
=================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: February 2023
"""

# Constants
END     = 0xC0
ESC     = 0xDB
ESC_END = 0xDC
ESC_ESC = 0xDD


# ┌────────────────────────────────────────┐
# │ Encode function                        │
# └────────────────────────────────────────┘

def encode(buffer: bytes, prepend_end: bool = False) -> bytes:
    """
    Encode a given buffer in the SLIP format
    :param buffer: Input buffer to convert
    :param prepend_end: Append END character at beginning of encoded buffer?

    :return: Encoded bytes
    """

    o = bytearray([] if not prepend_end else [END])

    i = 0
    while i < len(buffer):
        c = buffer[i]
        i += 1

        if c == END:
            o.append(ESC)
            o.append(ESC_END)
        elif c == ESC:
            o.append(ESC)
            o.append(ESC_ESC)
        else:
            o.append(c)

    # Append packet end
    o.append(END)
    
    return bytes(o)


# ┌────────────────────────────────────────┐
# │ Decoder class                          │
# └────────────────────────────────────────┘

class Decoder:
    """
    Stateful class to decode incoming SLIP data
    """

    def __init__(self):
        self.buf         = bytearray([])
        self.is_escaping = False

    def reset(self):
        self.buf         = bytearray([])
        self.is_escaping = False

    def feed(self, buffer: bytes) -> (int, bool):
        """
        Feeds decoder and decode a given SLIP buffer into raw data

        :param buffer: Input buffer to decode
        :return: The number of bytes consumed from input, and if a frame is available
        """

        i = 0
        while i < len(buffer):
            c  = buffer[i]
            i += 1

            if self.is_escaping:
                self.is_escaping = False

                if c == ESC_END:
                    self.buf.append(END)
                elif c == ESC_ESC:
                    self.buf.append(ESC)
                else:
                    raise ValueError(f"Unknown escape character code: {c}")
            else:
                if c == END:
                    return (i, True,)

                elif c == ESC:
                    self.is_escaping = True

                else:
                    self.buf.append(c)

        # Input buffer processed, but no packet end yet detected
        return (i, False)


# ┌────────────────────────────────────────┐
# │ Unitary tests                          │
# └────────────────────────────────────────┘

if __name__ == "__main__":
    in_buf  = bytes([0x01, 0x02, END, ESC, 0xFF])
    out_buf = encode(in_buf)

    print(in_buf.hex(":", 1))
    print(out_buf.hex(":", 1))

    decoder = Decoder()

    nbytes, this_is_the_end = decoder.feed(out_buf)
    assert(nbytes          == len(out_buf))
    assert(this_is_the_end == True       )

    print(decoder.buf.hex(":", 1))

    ###

    good = False
    try:
        decoder.reset()
        decoder.feed(bytes([0x00, ESC, 0x02]))
    except ValueError as exc:
        good = True
    finally:
        if not good:
            raise AssertionError("Exception not thrown")
