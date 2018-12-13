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
  cart(int x, int y, int d) : posx(x), posy(y), direction(d), nextchoice(-1) {}

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
      char c = row[i];
      if ((c == '<')) { // direction 3
        cart car(i, r, 3);
        carts.push_back(car);
        cells[r][i] = '-';
      } else if (c == '^') { // direction 0
        cart car(i, r, 0);
        carts.push_back(car);
        cells[r][i] = '|';
      } else if (c == '>') { // direction 1
        cart car(i, r, 1);
        carts.push_back(car);
        cells[r][i] = '-';
      } else if (c == 'v') { // direction 2
        cart car(i, r, 2);
        carts.push_back(car);
        cells[r][i] = '|';
      } else {
        cells[r][i] = row[i];
      }
    }
  }

  bool update() {
    sortcarts();

    for (auto &c : carts) {
      c.move();
      c.updatebyposition(cells[c.y()][c.x()]);
    }

    return (collision() > -1);
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

  int collision() {
    sortcarts();
    for (int i = 0; i < carts.size() - 1; ++i) {
      if (carts[i] == carts[i + 1]) {
        collision_pos.first = carts[i].x();
        collision_pos.second = carts[i].y();
        return i;
      }
    }
    return -1;
  }

  bool removecollisions() {
    int i;
    while ((i = collision()) > -1) {
      cart car1 = *(carts.begin() + i + 1);
      cart car2 = *(carts.begin() + i);
      carts.erase(carts.begin() + i + 1);
      carts.erase(carts.begin() + i);
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
      bool res = update();
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