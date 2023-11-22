"""
==================================================
Helper to handle HA device using a given transport
==================================================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

import threading

from queue      import Queue, Empty
from ..protocol import slip, common, uart, framing, parser

from loguru     import  logger

import traceback

class DeviceWrapper:
    def __init__(self, transport):
        self.transport        = transport
        self.decoder          = slip.Decoder()

        self.wqueue           = Queue() # Writing queue PC -> Device
        self.rqueue           = Queue() # Reading queue Device -> PC

        self._thread          = threading.Thread(target=self._worker, daemon=True)
        self._run             = threading.Event()

        # Flag indicating that the devices wait for a response,
        # and that no request should be sent.
        self._waits_response  = threading.Event()

    ######################################################
    #                 CONTEXT MANAGER                    #
    ######################################################

    def __enter__(self):
        self.start()
        return self

    def __exit__(self, exc_type, exc_value, exc_traceback):
        self.stop()


    ######################################################
    #               START/STOP OPERATIONS                #
    ######################################################

    def start(self):
        self._run.set()
        self._thread.start()


    def stop(self):
        self._run.clear()
        self.transport.cancel_read()
        self._thread.join()


    ######################################################
    #               HIGH LEVEL OPERATIONS                #
    ######################################################

    def send_request(self, req):
        self.wqueue.put(req)
        self.transport.cancel_read() # Trigger worker thread


    def read_response(self, timeout=None):
        resp = self.rqueue.get(block=True, timeout=timeout)

        return resp

    def request(self, req, expected_response_type):
        self.send_request(req)

        resp = self.read_response()
        if not isinstance(resp, expected_response_type):
            raise ValueError(f"Unexpected response for request {req}: {resp}")

        return resp


    ######################################################
    #               HIGH LEVEL GENERIC REQS              #
    ######################################################

    def version(self):
        ver = self.request(common.RequestVersion(), common.ResponseVersion)
        return ver.version

    
    def itf_type(self):
        itf = self.request(common.RequestItfType(), common.ResponseItfType)
        return itf.itfType

    def ping(self):
        self.request(common.RequestPing(), common.StatusGood)


    ######################################################
    #                 MESSAGE CALLBACK                   #
    ######################################################

    def message_callback(self, msg):
        return msg


    ######################################################
    #                  WORKER THREAD                     #
    ######################################################

    def _worker(self):
        print("test1")
        logger.info("Start worker thread...")

        while self._run.is_set():
            # Process writing queue
            print("test2")
            if not self._waits_response.is_set():
                try:
                    wr_msg = self.wqueue.get_nowait()
                    #logger.debug(f"Send message: {wr_msg}")

                    frame  = wr_msg.to_frame()
                    slipf  = slip.encode(frame.buffer())
                    print(slipf)

                    #logger.debug(f"TX: {slipf.hex(':')}")

                    self.transport.write(slipf)
                    self.transport.flush()

                    self._waits_response.set()

                except Empty:
                    pass

            # Process read
            c = self.transport.read(1)

            if c:
                nbytes, is_end = self.decoder.feed(c)

                # Is there a frame to process?
                if is_end:
                    try:
                        frame    = framing.MsgFrame.from_bytes(self.decoder.buf)
                        resp     = parser.from_frame(frame)

                        filtered = self.message_callback(resp)
                        if filtered is not None:
                            self.rqueue.put(filtered, timeout=1)
                            self._waits_response.clear()

                    except Exception as exc:
                        logger.error(f"Error decoding frame: {resp}")
                        logger.debug(traceback.format_exc())

                    finally:
                        self.decoder.reset()


    ######################################################
