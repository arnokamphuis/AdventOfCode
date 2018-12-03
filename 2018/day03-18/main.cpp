#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

//#include "basetimer.h"
#include "logger.h"

int main() {
  std::vector<std::vector<int8_t>> cloth;
  for (int i = 0; i < 1200; ++i) {
    cloth.push_back({});
    for (int j = 0; j < 1200; ++j) {
      cloth[i].push_back(0);
    }
  }

  std::vector<std::string> input;

  std::string magic = "";
  // Part 1
  std::string line;
  while (getline(std::cin, line)) {
    input.push_back(line);
  }

  for (auto line : input) {
    auto at_p = line.find('@');
    std::string id = line.substr(0, at_p);

    std::string part = line.substr(at_p + 2, line.length());
    auto at_c = part.find(':');
    std::string position = part.substr(0, at_c);
    std::string size = part.substr(at_c + 2, part.length());

    int posx;
    int posy;
    int sizex;
    int sizey;
    auto at_comma = position.find(',');
    auto at_x = size.find('x');

    posx = std::atoi(position.substr(0, at_comma).c_str());
    posy = std::atoi(position.substr(at_comma + 1, position.length()).c_str());

    sizex = std::atoi(size.substr(0, at_x).c_str());
    sizey = std::atoi(size.substr(at_x + 1, size.length()).c_str());

    for (int x = posx; x < posx + sizex; ++x) {
      for (int y = posy; y < posy + sizey; ++y) {
        ++cloth[x][y];
      }
    }
  }

  int64_t counter = 0;
  for (auto cx : cloth)
    for (auto cy : cx)
      if (cy > 1)
        ++counter;

  // Part 2
  for (auto line : input) {
    auto at_p = line.find('@');
    std::string id = line.substr(0, at_p);

    std::string part = line.substr(at_p + 2, line.length());
    auto at_c = part.find(':');
    std::string position = part.substr(0, at_c);
    std::string size = part.substr(at_c + 2, part.length());

    int posx;
    int posy;
    int sizex;
    int sizey;
    auto at_comma = position.find(',');
    auto at_x = size.find('x');

    posx = std::atoi(position.substr(0, at_comma).c_str());
    posy = std::atoi(position.substr(at_comma + 1, position.length()).c_str());

    sizex = std::atoi(size.substr(0, at_x).c_str());
    sizey = std::atoi(size.substr(at_x + 1, size.length()).c_str());

    bool founddouble = false;
    for (int x = posx; x < posx + sizex; ++x) {
      for (int y = posy; y < posy + sizey; ++y) {
        if (cloth[x][y] > 1)
          founddouble = true;
      }
    }
    if (!founddouble)
      magic = id;
  }

  logger::get(logtype::logINFO) << "Part 1: " << counter << "\n";
  logger::get(logtype::logINFO) << "Part 2: " << magic << "\n";
  return 0;
}