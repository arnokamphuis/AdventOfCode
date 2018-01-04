#include "logger.h"
#include <iostream>
#include <stdio.h>
#include <stdlib.h>

enum class state { esc, character, hex };

int main() {
  int ch;

  int c_file = 0;
  int c_mem = 0;
  int c_newfile = 0;

  state s = state::character;
  int skip_counter = 0;

  while ((ch = getchar()) != EOF) {
    if (ch != '\n') {
      ++c_file;
      ++c_newfile;

      switch (s) {
      case state::character: {
        if (ch != '"') {
          if (ch == '\\') {
            ++c_newfile;
            s = state::esc;
            skip_counter = 2;
          } else {
            ++c_mem;
          }
        } else {
          c_newfile += 2;
        }
      } break;
      case state::esc: {
        if (ch == 'x') {
          s = state::hex;
        } else {
          if ((ch == '\\') or (ch == '\"')) {
            ++c_mem;
            ++c_newfile;
          };
          s = state::character;
        }
      } break;
      case state::hex: {
        skip_counter--;
        if (skip_counter == 0) {
          s = state::character;
          ++c_mem;
        }
      } break;
      default: {
        logger::get(logtype::logERROR) << "This should not happen" << '\n';
      } break;
      }
    }
  }

  logger::get(logtype::logINFO) << "Part 1: " << (c_file - c_mem) << std::endl;
  logger::get(logtype::logINFO)
      << "Part 2: " << (c_newfile - c_file) << std::endl;
}