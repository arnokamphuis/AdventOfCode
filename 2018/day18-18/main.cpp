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
  int runs;

  std::map<std::vector<groundtype>, int> history;

public:
  map() {
    runs = 0;
    h = 0;
  }

  void print() {
    for (int y = 0; y < w; ++y) {
      for (int x = 0; x < w; ++x) {
        int index = x + y * w;
        if (field[index] == OPEN)
          std::cout << ".";
        if (field[index] == TREES)
          std::cout << "|";
        if (field[index] == LUMBER)
          std::cout << "#";
      }
      std::cout << std::endl;
    }
  }

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

  int update() {
    ++runs;
    std::vector<groundtype> newfield = field;
    std::map<groundtype, int> counter;

    for (int x = 0; x < w; ++x) {
      for (int y = 0; y < w; ++y) {
        int index = x + y * w;
        counter[OPEN] = 0;
        counter[TREES] = 0;
        counter[LUMBER] = 0;

        int left = ((x == 0) ? 0 : -1);
        int right = ((x == (w - 1)) ? 0 : 1);

        int top = ((y == 0) ? 0 : -1);
        int bottom = ((y == (w - 1)) ? 0 : 1);

        for (int s = left; s <= right; ++s) {
          for (int t = top; t <= bottom; ++t) {
            if (!((s == 0) && (t == 0))) {
              int offindex = (x + s) + (y + t) * w;
              if ((offindex >= 0) && (offindex < size)) {
                counter[field[offindex]] += 1;
              }
            }
          }
        }

        groundtype gt = field[index];

        // std::cout << "trans: " << index << "(" << gt << ") : " <<
        // counter[OPEN]
        //           << " " << counter[TREES] << " " << counter[LUMBER]
        //           << std::endl;

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

    if (history.find(field) != history.end())
      return runs - history[field];
    else
      history[field] = runs;
    return -1;
  }

  void doruns(int count) {
    int delta;
    for (int i = 0; i < count; ++i) {
      if ((delta = update()) != -1) {
        i += delta * ((count - i) / delta);
      }
    }
  }

  int calcscore() {
    std::map<groundtype, int> counter;
    counter[OPEN] = 0;
    counter[TREES] = 0;
    counter[LUMBER] = 0;
    for (int x = 0; x < w; ++x) {
      for (int y = 0; y < w; ++y) {
        int index = x + y * w;
        counter[field[index]] += 1;
      }
    }

    return counter[TREES] * counter[LUMBER];
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

  for (int i = 0; i < 10; ++i)
    m.update();
  logger::get(logtype::logINFO) << "Part 1: " << m.calcscore() << "\n";

  int runs = 1000000000;
  m.doruns(runs - 10);
  logger::get(logtype::logINFO) << "Part 2: " << m.calcscore() << "\n";

  return 0;
}