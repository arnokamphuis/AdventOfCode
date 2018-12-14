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

enum dir { RIGHT = 0, UP = 1, LEFT = 2, DOWN = 3 };

class cart {
  std::pair<int, int> position;
  int id;
  int nextchoice;
  dir direction;

public:
  cart(int x, int y, int id, char c) : nextchoice(-1), id(id) {
    position = std::make_pair(y, x);
    id = id;
    switch (c) {
    case '^':
      direction = UP;
      break;
    case 'v':
      direction = DOWN;
      break;
    case '<':
      direction = LEFT;
      break;
    case '>':
      direction = RIGHT;
      break;
    }
  }

  const dir &getdirection() const { return direction; }

  friend inline bool operator<(const cart &lhs, const cart &rhs) {
    if (lhs.position.first < rhs.position.first)
      return true;
    if ((lhs.position.first == rhs.position.first) &&
        (lhs.position.second < rhs.position.second))
      return true;
    return false;
  }
  friend inline bool operator>(const cart &lhs, const cart &rhs) {
    if (lhs.position.first > rhs.position.first)
      return true;
    if ((lhs.position.first == rhs.position.first) &&
        (lhs.position.second > rhs.position.second))
      return true;
    return false;
  }

  const int &x() const { return position.second; }
  const int &y() const { return position.first; }
  const int &getid() const { return id; }

  void move() {
    switch (direction) {
    case UP:
      --position.first;
      break;
    case DOWN:
      ++position.first;
      break;
    case LEFT:
      --position.second;
      break;
    case RIGHT:
      ++position.second;
      break;
    }
  }

  void turnleft() {
    if (direction == UP)
      direction = LEFT;
    else if (direction == LEFT)
      direction = DOWN;
    else if (direction == DOWN)
      direction = RIGHT;
    else if (direction == RIGHT)
      direction = UP;
  }

  void turnright() {
    if (direction == UP)
      direction = RIGHT;
    else if (direction == LEFT)
      direction = UP;
    else if (direction == DOWN)
      direction = LEFT;
    else if (direction == RIGHT)
      direction = DOWN;
  }

  void update(const char &c) {
    if (!((c == '-') || (c == '|'))) {
      if (c == '/') {
        switch (direction) {
        case UP:
        case DOWN:
          turnright();
          break;
        case LEFT:
        case RIGHT:
          turnleft();
          break;
        }
      } else if (c == '\\') {
        switch (direction) {
        case UP:
        case DOWN:
          turnleft();
          break;
        case LEFT:
        case RIGHT:
          turnright();
          break;
        }
      } else if (c == '+') {
        switch (nextchoice) {
        case -1:
          turnleft();
          break;
        case 0:
          break;
        case +1:
          turnright();
          break;
        }
        ++nextchoice;
        if (nextchoice > 1)
          nextchoice = -1;
      }
    }
  }
};

class map {
  int weight;
  int height;

  std::vector<std::vector<char>> cells;
  std::vector<cart> carts;

  std::pair<int, int> collision_position;

public:
  map(int w, int h) : weight(w), height(h) {}

  void addline(const std::string &line) {
    std::vector<char> row;
    int x = 0;
    int y = cells.size();
    for (auto c : line) {
      char n = c;
      if ((n == '<') || (n == '>') || (n == '^') || (n == 'v')) {
        cart nc(x, y, carts.size() + 1, c);
        switch (nc.getdirection()) {
        case UP:
        case DOWN:
          n = '|';
          break;
        case LEFT:
        case RIGHT:
          n = '-';
          break;
        }
        carts.push_back(nc);
      }
      row.push_back(n);
      ++x;
    }
    cells.push_back(row);
  }

  void sortcarts() { std::sort(carts.begin(), carts.end(), std::less<>()); }

  void printcarts() {
    for (auto c : carts) {
      std::cout << c.x() << ", " << c.y() << " -> " << c.getdirection()
                << " => " << c.getid() << '\n';
    }
  }

  bool collision(const cart &c, std::pair<int, int> &p) {
    for (auto &c2 : carts) {
      if ((c.getid() != c2.getid()) && (c2.x() == c.x()) && (c2.y() == c.y())) {
        collision_position.first = c.x();
        collision_position.second = c.y();
        p.first = c.getid();
        p.second = c2.getid();
        return true;
      }
    }
    return false;
  }

  void remove(int cid) {
    for (int i = 0; i < carts.size(); ++i) {
      if (carts[i].getid() == cid) {
        carts.erase(carts.begin() + i);
        return;
      }
    }
  }

  void remove(const cart &c) {
    std::vector<int> tbr;
    tbr.push_back(c.getid());
    for (auto &c2 : carts)
      if ((c.getid() != c2.getid()) && (c2.x() == c.x()) && (c2.y() == c.y()))
        tbr.push_back(c2.getid());

    while (tbr.size() > 0) {
      for (int i = 0; i < carts.size(); ++i) {
        if (carts[i].getid() == tbr[0]) {
          //          std::cout << "removing: " << tbr[0] << "\n";
          carts.erase(carts.begin() + i);
          tbr.erase(tbr.begin());
          break;
        }
      }
    }
  }

  const std::pair<int, int> &getcollisionposition() const {
    return collision_position;
  }

  std::pair<int, int> lastposition() {
    std::pair<int, int> ret;
    ret.first = carts[0].x();
    ret.second = carts[0].y();
    return ret;
  }

  bool turn(bool removecarts) {
    sortcarts();

    std::set<int> tbr;

    for (auto &c : carts) {
      c.move();
      std::pair<int, int> p;
      if (collision(c, p)) {
        tbr.insert(p.first);
        tbr.insert(p.second);
      } else {
        c.update(cells[c.y()][c.x()]);
      }
    }

    if (removecarts) {
      for (auto cid : tbr) {
        remove(cid);
      }
      return carts.size() > 1;
    } else {
      return tbr.size() == 0;
    }
  }

  void printmap() {
    std::vector<std::vector<char>> pm = cells;

    for (auto c : carts) {
      char ch = '*';
      if (c.getdirection() == UP)
        ch = '^';
      if (c.getdirection() == DOWN)
        ch = 'v';
      if (c.getdirection() == LEFT)
        ch = '<';
      if (c.getdirection() == RIGHT)
        ch = '>';
      pm[c.y()][c.x()] = ch;
    }

    for (auto row : pm) {
      for (auto c : row)
        std::cout << c;
      std::cout << "\n";
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

  for (auto in : input) {
    m1.addline(in);
    m2.addline(in);
  }

  m1.sortcarts();
  while (m1.turn(false)) {
  }
  std::pair<int, int> pos1 = m1.getcollisionposition();
  logger::get(logtype::logINFO)
      << "Part 1: " << pos1.first << "\t" << pos1.second << "\n";

  m2.sortcarts();
  while (m2.turn(true)) {
  }
  std::pair<int, int> pos2 = m2.lastposition();
  logger::get(logtype::logINFO)
      << "Part 2: " << pos2.first << "\t" << pos2.second << "\n";
  return 0;
}