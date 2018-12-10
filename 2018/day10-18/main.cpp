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
  int size() { return cur.size(); }

  void addparticle(int x, int y, int vx, int vy) {
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
    max_y = max_x;

    for (auto c : cur) {
      if (min_x > c.first)
        min_x = c.first;
      if (max_x < c.first)
        max_x = c.first;
      if (min_y > c.second)
        min_y = c.second;
      if (max_y < c.second)
        max_y = c.second;
    }
    return ((max_y - min_y) + (max_x - min_x));
  }

  int calc_dist(int t) {
    int total = 0;
    for (auto c1 : cur) {
      int min_d = std::numeric_limits<int>::max();
      for (auto c2 : cur) {
        if (c1 != c2) {
          int d = std::max(std::abs(c1.first - c2.first),
                           std::abs(c1.second - c2.second));
          if (d < min_d)
            min_d = d;
        }
      }
      total += min_d;
    }
    return total;
  }

  void print(int t) {
    update(t);
    calcbounds(t);

    int w = max_x - min_x;
    int h = max_y - min_y;
    std::vector<std::vector<char>> drawing;
    for (int x = 0; x <= h; ++x) {
      std::vector<char> tmp;
      for (int y = 0; y <= w; ++y) {
        tmp.push_back('.');
      }
      drawing.push_back(tmp);
    }

    for (auto c : cur) {
      drawing[c.second - min_y][c.first - min_x] = '#';
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

  auto start = std::chrono::high_resolution_clock::now();
  m.update(0);
  int min = m.calc_dist(0);
  int t = 0;
  while (min != m.size()) {
    ++t;
    m.update(t);
    min = m.calc_dist(t);
  }
  auto stop = std::chrono::high_resolution_clock::now();

  logger::get(logtype::logINFO) << "Part 1: \n";
  m.print(t);
  logger::get(logtype::logINFO)
      << "\nin "
      << std::chrono::duration_cast<std::chrono::milliseconds>(stop - start)
             .count()
      << " ms\nPart 2: " << t << "\n";

  return 0;
}