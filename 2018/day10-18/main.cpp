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

class map {
  std::vector<std::pair<int, int>> pos;
  std::vector<std::pair<int, int>> vel;
  std::vector<std::pair<int, int>> cur;

  int min_x;
  int min_y;
  int max_x;
  int max_y;

public:
  addparticle(int x, int y, int vx, int vy) {
    pos.push_back(std::make_pair(x, y));
    vel.push_back(std::make_pair(vx, vy));
    cur.push_back(std::make_pair(x, y));
  }

  void update(int t) {
    for (int i = 0; i < pos.size(); ++i) {
      cur[i].first = pos[i].first + t * vel[i].first;
      cur[i].second = pos[i].second + t * vel[i].second;
    }
  }

  int calcbounds(int t) {
    min_x = std::numeric_limits<int>::max();
    max_x = std::numeric_limits<int>::min();
    min_y = min_x;
    max_y = max_y;

    for (auto c : cur) {
      if (min_x > c.first)
        min_x = c.first;
      if (max_x < c.first)
        max_x = c.first;
      if (min_y > c.first)
        min_y = c.first;
      if (max_y < c.first)
        max_y = c.first;
    }

    return (max_y - min_y + max_x - min_x);
  }

  void print(int t) {
    int w = max_x - min_x;
    int h = max_y - min_y;
    update(t);
    calcbounds(t);

    std::vector<std::vector<char>> drawing;
    for (int x = 0; x <= h; ++x) {
      std::vector<char> tmp;
      for (int y = 0; y <= w; ++y) {
        tmp.push_back('.');
      }
      drawing.push_back(tmp);
    }

    for (auto c : cur) {
      // std::cout << c.first - min_x << "\t" << c.second - min_y << "(" << h
      //           << "\t" << w << "\n";
      drawing[c.first - min_x][c.second - min_y] = '#';
    }

    for (auto dx : drawing) {
      for (auto dy : dx) {
        std::cout << dy;
      }
      std::cout << "\n";
    }
  }
};

int main() {
  map m;
  std::string line;
  while (getline(std::cin, line)) {
    auto left = line.find("vel");
    auto posstr = line.substr(0, left);
    auto velstr = line.substr(left, line.length());

    auto leftbrace = posstr.find("<");
    auto comma = posstr.find(",");
    auto rightbrace = posstr.find(">");
    int posx =
        std::atoi(posstr.substr(leftbrace + 1, comma - leftbrace).c_str());
    int posy = std::atoi(posstr.substr(comma + 1, posstr.length()).c_str());

    leftbrace = velstr.find("<");
    comma = velstr.find(",");
    rightbrace = velstr.find(">");
    int velx =
        std::atoi(velstr.substr(leftbrace + 1, comma - leftbrace).c_str());
    int vely = std::atoi(velstr.substr(comma + 1, velstr.length()).c_str());

    m.addparticle(posx, posy, velx, vely);
  }

  int prev_min = std::numeric_limits<int>::max();
  int min;
  m.update(0);
  min = m.calcbounds(0);
  int t = 1;
  while (min < prev_min) {
    prev_min = min;
    m.update(t);
    min = m.calcbounds(t);
    ++t;
  }
  --t;
  std::cout << t << std::endl;
  m.print(t);

  return 0;
}