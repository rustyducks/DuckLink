
#include <stdint.h>

#define clamp(lo, v, hi) ((v) < (lo)) ? (lo) : ((hi) < (v)) ? (hi) : (v);


class DuckMsg {
public:
  virtual void to_bytes(uint8_t *buffer) = 0;
  static uint16_t compute_cheksum(uint8_t *buffer, int len) {
    return 10;
  }
};
