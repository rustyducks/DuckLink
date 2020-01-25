#!/usr/bin/python3

def clamp(l,v,h):
    return max(min(v, h), l)

class DuckMsg:

    def get_name(self):
        return type(self).__name__

