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

class cart {
  int posx;
  int posy;
  int direction;
  int nextchoice;
  int id;

  void turnleft() {
    --direction;
    if (direction < 0)
      direction = 3;
  }

  void turnright() {
    ++direction;
    if (direction > 3)
      direction = 0;
  }

public:
  cart(int x, int y, int d, int num)
      : posx(x), posy(y), direction(d), id(num), nextchoice(-1) {}

  friend inline bool operator<(const cart &lhs, const cart &rhs) {
    if (lhs.posy < rhs.posy)
      return true;
    if ((lhs.posy == rhs.posy) && (lhs.posx < rhs.posx))
      return true;
    return false;
  }

  friend inline bool operator==(const cart &lhs, const cart &rhs) {
    return ((lhs.posy == rhs.posy) && (lhs.posx == rhs.posx));
  }

  const int &x() const { return posx; }
  const int &y() const { return posy; }
  const int &getid() const { return id; }

  void updatebyposition(char c) {
    if (c == '+')
      changedirection();
    if ((c == '\\') && ((direction == 0) || (direction == 2))) {
      turnleft();
      return;
    }
    if ((c == '\\') && ((direction == 1) || (direction == 3))) {
      turnright();
      return;
    }

    if ((c == '/') && ((direction == 0) || (direction == 2))) {
      turnright();
      return;
    }
    if ((c == '/') && ((direction == 1) || (direction == 3))) {
      turnleft();
      return;
    }
  }

  void move() {
    if (direction == 0)
      --posy;
    if (direction == 1)
      ++posx;
    if (direction == 2)
      ++posy;
    if (direction == 3)
      --posx;
  }

  void changedirection() {
    if (nextchoice == -1)
      turnleft();
    if (nextchoice == 1)
      turnright();
    ++nextchoice;
    if (nextchoice == 2)
      nextchoice = -1;
  }
};

class map {
  int width;
  int height;
  std::vector<std::vector<char>> cells;
  std::vector<cart> carts;
  std::pair<int, int> collision_pos;

  void sortcarts() { std::sort(carts.begin(), carts.end(), std::less<>()); }

public:
  map(int w, int h) : width(w), height(h) {
    for (int i = 0; i < h; ++i) {
      std::vector<char> row;
      for (int j = 0; j < w; ++j)
        row.push_back(' ');
      cells.push_back(row);
    }
  }

  void setrow(int r, std::string row) {
    for (int i = 0; i < row.length(); ++i) {
      int id = carts.size() + 1;
      char c = row[i];
      if ((c == '<')) { // direction 3
        cart car(i, r, 3, id);
        carts.push_back(car);
        cells[r][i] = '-';
      } else if (c == '^') { // direction 0
        cart car(i, r, 0, id);
        carts.push_back(car);
        cells[r][i] = '|';
      } else if (c == '>') { // direction 1
        cart car(i, r, 1, id);
        carts.push_back(car);
        cells[r][i] = '-';
      } else if (c == 'v') { // direction 2
        cart car(i, r, 2, id);
        carts.push_back(car);
        cells[r][i] = '|';
      } else {
        cells[r][i] = row[i];
      }
    }
  }

  bool update(bool inbetween = false) {
    sortcarts();

    for (auto &c : carts) {
      c.move();
      c.updatebyposition(cells[c.y()][c.x()]);
      if (inbetween) {
        if (!removecollisions())
          break;
      }
    }

    return collision();
  }

  void print() {
    std::vector<std::vector<char>> printmap = cells;

    for (auto c : carts) {
      printmap[c.y()][c.x()] = '*';
    }
    for (auto row : printmap) {
      for (auto c : row)
        std::cout << c;
      std::cout << "\n";
    }
  }

  const std::pair<int, int> &colpos() const { return collision_pos; }

  bool collision() {
    for (int i = 0; i < carts.size(); ++i) {
      for (int j = i + 1; j < carts.size(); ++j) {
        if (carts[i] == carts[j]) {
          collision_pos.first = carts[i].x();
          collision_pos.second = carts[i].y();
          return true;
        }
      }
    }
    return false;
  }

  bool removecollisions() {
    std::set<int> toberemoved;

    for (int i = 0; i < carts.size(); ++i) {
      for (int j = i + 1; j < carts.size(); ++j) {
        if (carts[i] == carts[j]) {
          toberemoved.insert(carts[i].getid());
          toberemoved.insert(carts[j].getid());
        }
      }
    }

    for (auto tbr : toberemoved) {
      for (int i = 0; i < carts.size(); ++i) {
        if (tbr == carts[i].getid()) {
          carts.erase(carts.begin() + i);
          break;
        }
      }
    }

    if (carts.size() == 1) {
      collision_pos.first = carts[0].x();
      collision_pos.second = carts[0].y();
      return false;
    } else
      return true;
  }

  void run(bool withremoving) {
    while (true) {
      bool res = update(withremoving);
      if (withremoving) {
        if (!removecollisions())
          return;
      } else {
        if (res)
          return;
      }
    }
  }
};

int main() {
  int width = 0;
  int height = 0;
  std::string line;
  std::vector<std::string> input;
  while (getline(std::cin, line)) {
    width = line.length();
    input.push_back(line);
  }
  height = input.size();

  map m1(width, height);
  map m2(width, height);
  for (int r = 0; r < input.size(); ++r) {
    m1.setrow(r, input[r]);
    m2.setrow(r, input[r]);
  }

  m1.run(false);
  logger::get(logtype::logINFO)
      << "Part 1: " << m1.colpos().first << "\t" << m1.colpos().second << "\n";

  m2.run(true);
  logger::get(logtype::logINFO)
      << "Part 2: " << m2.colpos().first << "\t" << m2.colpos().second << "\n";

  return 0;
}