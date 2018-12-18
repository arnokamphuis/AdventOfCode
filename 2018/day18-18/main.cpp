#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

enum groundtype { OPEN = 1, TREES = 2, LUMBER = 3 };

class map {
  int w;
  int h;
  int size;
  std::vector<groundtype> field;

public:
  map() { h = 0; }

  void addline(std::string line) {
    if (field.size() == 0) {
      w = line.length();
      size = w * w;
      field.resize(size, 0);
    }
    for (int i = 0; i < w; ++i) {
      groundtype gt;
      switch (line[i]) {
      case '#':
        gt = LUMBER;
        break;
      case '.':
        gt = OPEN;
        break;
      case '|':
        gt = TREES;
        break;
      }
      field[i + h * w] = gt;
    }

    ++h;
  }

  void update() {
    std::vector<groundtype> newfield;
    std::map<groundtype, int> counter;

    for (int x = 0; x < w; ++x) {
      for (int y = 0; y < w; ++y) {
        index = x + y * w;
        for (int i = 0; i < 3; ++i)
          counter[i] = 0;
        for (int s = -1; s < 2; ++s) {
          for (int t = -1; t < 2; ++t) {
            int offindex = (x + s) + (y + t) * w;
            if ((offindex >= 0) && (offindex < size)) {
              // counter[]
            }
          }
        }
        newfield[i] = field[i];
      }
    }

    field = newfield;
  }
};

int main() {
  std::string line;
  std::vector<std::string> input;

  map m;
  while (getline(std::cin, line)) {
    m.addline(line);
    input.push_back(line);
  }

  return 0;
}