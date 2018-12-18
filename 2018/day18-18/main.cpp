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
      field.resize(size, OPEN);
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
        int index = x + y * w;
        counter[OPEN] = 0;
        counter[TREES] = 0;
        counter[LUMBER] = 0;
        for (int s = -1; s < 2; ++s) {
          for (int t = -1; t < 2; ++t) {
            if (!((s == 0) && (t == 0))) {
              int offindex = (x + s) + (y + t) * w;
              if ((offindex >= 0) && (offindex < size)) {
                counter[field[offindex]] += 1;
              }
            }
          }
        }

        groundtype gt = field[index];

        if (gt == OPEN) {
          if (counter[TREES] >= 3)
            newfield[index] = TREES;
        } else if (gt == TREES) {
          if (counter[LUMBER] >= 3)
            newfield[index] = LUMBER;
        } else if (gt == LUMBER) {
          if ((counter[LUMBER] >= 1) && (counter[TREES] >= 1))
            newfield[index] = LUMBER;
          else
            newfield[index] = OPEN;
        }
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

  m.update();

  return 0;
}