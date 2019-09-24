#!/usr/bin/python3

def clamp(l,v,h):
    return max(min(v, h), l)

class DuckMsg:

    def __init__(self, msg_id, msg_name):
        self.id = msg_id
        self.name = msg_name

    def get_id(self):
        return self.id

    def get_name(self):
        return type(self).__name__

