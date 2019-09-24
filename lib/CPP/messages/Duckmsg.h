
#include <stdint.h>

#define clamp(lo, v, hi) ((v) < (lo)) ? (lo) : ((hi) < (v)) ? (hi) : (v);


class DuckMsg {

public:
    virtual uint8_t get_id() = 0;
    virtual char* get_name() = 0;


};
