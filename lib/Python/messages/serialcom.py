#!/usr/bin/python3
import serial
import duckmsg
import messages
import time
import bitstring
import sys
import time
from enum import Enum


class SerialCom:

    class RcvState(Enum):
        Idle = 0
        Start1st = 1
        Start2nd = 2
        MsgId = 3
        MsgLen = 4
        

    def __init__(self, port, baudrate=115200):
        self.serial = serial.Serial(port, baudrate, timeout=0.01)
        self._rcv_state = SerialCom.RcvState.Idle
        self._nb_bytes_expected = 1
        self._msg_id = 0
        self._msg_len = 0

    def check_msgs(self):
        while self.serial.in_waiting >= self._nb_bytes_expected:
            if self._rcv_state == SerialCom.RcvState.Idle:  # wait for 0XFF
                if ord(self.serial.read()) == 0xFF:
                    self._rcv_state = SerialCom.RcvState.Start1st
                else:                                               # fallback to Idle
                    self._rcv_state = SerialCom.RcvState.Idle
            elif self._rcv_state == SerialCom.RcvState.Start1st:
                if ord(self.serial.read()) == 0xFF:
                    self._rcv_state = SerialCom.RcvState.Start2nd
                else:                                               # fallback to Idle
                    self._rcv_state = SerialCom.RcvState.Idle
            elif self._rcv_state == SerialCom.RcvState.Start2nd:
                self._msg_id = ord(self.serial.read())
                self._rcv_state = SerialCom.RcvState.MsgId
            elif self._rcv_state == SerialCom.RcvState.MsgId:
                self._msg_len = ord(self.serial.read())
                self._nb_bytes_expected = self._msg_len
                self._rcv_state = SerialCom.RcvState.MsgLen
            elif self._rcv_state == SerialCom.RcvState.MsgLen:
                payload = self.serial.read(self._msg_len)       # read message content
                if self.control_checksum(self._msg_id, self._msg_len, payload):
                    try:
                        msgClass = messages.MESSAGES[self._msg_id]
                    except KeyError:
                        print("message id {} unknown!".format(self._msg_id))
                    msg = msgClass()
                    msg.deserialize(payload)
                    self._nb_bytes_expected = 1
                    self._rcv_state = SerialCom.RcvState.Idle
                    if self._msg_id == 0:   # UID message
                        if not msg.uid == messages.UID:
                            raise(Exception("Warning: Ducklink versions differs : remote is {}, local is {}".format(msg.uid, messages.UID)))
                    return msg                 # We are now synchronised !
                else:
                    return None

    """
    Blocking function until a message is received
    """
    def next_message(self):
        while(True):
            msg = self.check_msgs()
            if msg is not None:
                return msg
            time.sleep(0.001)
    
    def flush_input(self):
        while self.serial.in_waiting > 0:
            self.serial.read(self.serial.in_waiting)
            
    @staticmethod
    def calculate_checksum(msg_bytes):
        ck_a = 0
        ck_b = 0
        for c in msg_bytes:
            ck_a = (ck_a + c) % 256
            ck_b = (ck_b + ck_a) % 256
        ck = (ck_a<<8) | ck_b
        return ck
    
    @staticmethod
    def control_checksum(msg_id, msg_len, payload):
        # reconstruct the message from ID to payload(excluding checksum)
        to_check = chr(msg_id).encode() + chr(msg_len).encode() + payload[:-2]
        ck = SerialCom.calculate_checksum(to_check)
        s = bitstring.BitStream(payload[-2:])
        rcv_ck, = s.unpack('uintle:16')         # coma to unpack the list as tuple
        if ck == rcv_ck:
            return True
        else:
            return False


    def send_msg(self, msg):
        start = bitstring.pack('uintle:16', 0XFFFF)
        payload = msg.serialize()
        chk = self.calculate_checksum(payload.tobytes())
        msg_stream = start + payload + bitstring.pack('uintle:16', chk)
        msg_bytes = msg_stream.tobytes()
        self.serial.write(msg_bytes)
    
    def close(self):
        self.serial.close()
        

if __name__ == '__main__':
    sercom = SerialCom(sys.argv[1], int(sys.argv[2]))
    while(True):
        msg = sercom.next_message()
        print(msg)

