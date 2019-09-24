#ifndef MESSAGES_H
#define MESSAGES_H

#include <stdint.h>
#include <string.h>
#include "Duckmsg.h"

class DownToto: public DuckMsg {
public:
  DownToto() {
    _id = 0;
    _decimal = 0;
    _entier = 0;
    _name[0] = '\0';
    _vx = 0;
  }

  uint8_t get_id(){ return _id; }

  float get_decimal(){ return _decimal; }
  void set_decimal(float decimal){ _decimal = clamp(-340282346638528860000000000000000000000, decimal, 340282346638528860000000000000000000000); }

  int16_t get_entier(){ return _entier; }
  void set_entier(int16_t entier){ _entier = clamp(-32768, entier, 32767); }

  char* get_name(){ return _name; }
  void set_name(char* name) {
    strncpy(_name, name, 12);
  }

  float get_vx(){ return _vx; }
  void set_vx(float vx){ _vx = clamp(-340282346638528860000000000000000000000, vx, 340282346638528860000000000000000000000); }

private:
  uint8_t _id;
  float _decimal;
  int16_t _entier;
  char _name[12];
  float _vx;
};


class InterMCUProut: public DuckMsg {
public:
  InterMCUProut() {
    _id = 1;
    _odeur[0] = '\0';
  }

  uint8_t get_id(){ return _id; }

  char* get_odeur(){ return _odeur; }
  void set_odeur(char* odeur) {
    strncpy(_odeur, odeur, 10);
  }

private:
  uint8_t _id;
  char _odeur[10];
};


class UpPlop: public DuckMsg {
public:
  UpPlop() {
    _id = 2;
    _decimal = 0;
    _entier = 0;
    _name[0] = '\0';
  }

  uint8_t get_id(){ return _id; }

  float get_decimal(){ return _decimal; }
  void set_decimal(float decimal){ _decimal = clamp(-30, decimal, 1000); }

  int16_t get_entier(){ return _entier; }
  void set_entier(int16_t entier){ _entier = clamp(-32768, entier, 32767); }

  char* get_name(){ return _name; }
  void set_name(char* name) {
    strncpy(_name, name, 12);
  }

private:
  uint8_t _id;
  float _decimal;
  int16_t _entier;
  char _name[12];
};


class UpSpeedReport: public DuckMsg {
public:
  UpSpeedReport() {
    _id = 3;
    _vtheta = 0;
    _vx = 0;
    _vy = 0;
  }

  uint8_t get_id(){ return _id; }

  uint8_t get_vtheta(){ return _vtheta; }
  void set_vtheta(uint8_t vtheta){ _vtheta = clamp(0, vtheta, 10); }

  int8_t get_vx(){ return _vx; }
  void set_vx(int8_t vx){ _vx = clamp(-128, vx, 127); }

  int16_t get_vy(){ return _vy; }
  void set_vy(int16_t vy){ _vy = clamp(-2, vy, 10); }

private:
  uint8_t _id;
  uint8_t _vtheta;
  int8_t _vx;
  int16_t _vy;
};

#endif    // MESSAGES_H
