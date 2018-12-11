#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

class energy {
  int64_t serialnumber;
  int size;
  std::vector<std::vector<int64_t>> powerlevel;
  std::vector<std::vector<int64_t>> powercells;

  void printpl() {
    for (auto row : powerlevel) {
      for (auto p : row)
        std::cout << p << ",";
      std::cout << "\n";
    }
  }

public:
  energy(int serial, int s) : serialnumber(serial), size(s) {
    for (int i = 0; i < size; ++i) {
      std::vector<int64_t> row;
      for (int j = 0; j < size; ++j) {
        int64_t rankid = i + 1 + 10;
        int64_t power = ((rankid * (j + 1)) + serial) * rankid;
        power = (power % 1000) / 100 - 5;
        row.push_back(power);
      }
      powerlevel.push_back(row);
    }
    // printpl();
  }

  std::pair<int, int> find_max(int s) {
    powercells.clear();
    for (int i = 0; i <= size - s; ++i) {
      std::vector<int64_t> row;
      for (int j = 0; j <= size - s; ++j) {
        int64_t temp = 0;
        for (int u = 0; u < s; ++u) {
          for (int v = 0; v < s; ++v) {
            temp += powerlevel[i + u][j + v];
          }
        }
        row.push_back(temp);
      }
      powercells.push_back(row);
    }

    std::pair<int, int> coordinates;
    int64_t max = std::numeric_limits<int64_t>::min();
    for (int i = 0; i <= size - s; ++i) {
      for (int j = 0; j <= size - s; ++j) {
        if (powercells[i][j] > max) {
          max = powercells[i][j];
          coordinates.first = i + 1;
          coordinates.second = j + 1;
        }
      }
    }
    return coordinates;
  }

  int64_t getpowercell(std::pair<int, int> coordinates) {
    return powercells[coordinates.first - 1][coordinates.second - 1];
  }
  int64_t getpowerlevel(std::pair<int, int> coordinates) {
    return powerlevel[coordinates.first - 1][coordinates.second - 1];
  }
};

int main() {
  int serial = 9221;
  int size = 300;

  energy e(serial, size);
  std::pair<int, int> c = e.find_max(3);

  logger::get(logtype::logINFO)
      << "Part 1: " << c.first << "," << c.second << "\n";

  int64_t maxpower = std::numeric_limits<int64_t>::min();
  int maxsize = 0;
  std::pair<int, int> max_c;
  for (int i = 1; i < size; ++i) {
    c = e.find_max(i);
    int64_t p = e.getpowercell(c);
    if (p > maxpower) {
      maxpower = p;
      max_c = c;
      maxsize = i;
    }
  }
  logger::get(logtype::logINFO) << "Part 2: " << max_c.first << ","
                                << max_c.second << "," << maxsize << "\n";

  return 0;
}