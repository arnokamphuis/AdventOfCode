#include "logger.h"
#include <cmath>
#include <iostream>
#include <vector>

class grid {
private:
  int size;

  int dx;
  int dy;

  int cx;
  int cy;

  std::vector<std::vector<char>> cells;

  int got_infected;

  void turn_left() {
    int tmp = dy;
    dy = -dx;
    dx = tmp;
  }

  void turn_right() {
    int tmp = dy;
    dy = dx;
    dx = -tmp;
  }

  void reverse() {
    dx = -dx;
    dy = -dy;
  }

  void step() {
    cx += dx;
    cy += dy;
    if ((cx < 0) or (cy < 0) or (cx == size) or (cy == size))
      increase_size();
  }

public:
  explicit grid(int s) : size(s) {
    got_infected = 0;
    for (int i = 0; i < size; ++i) {
      std::vector<char> l;
      for (int j = 0; j < size; ++j)
        l.push_back('c');
      cells.push_back(l);
    }
    cx = std::floor((float)size / 2.0);
    cy = std::floor((float)size / 2.0);
    dx = 0;
    dy = -1;
  }

  void burst() {

    char state = cells[cy][cx];

    if (state == 'c') {
      cells[cy][cx] = 'w';
      turn_left();
    } else if (state == 'w') {
      cells[cy][cx] = 'i';
    } else if (state == 'i') {
      cells[cy][cx] = 'f';
      turn_right();
    } else if (state == 'f') {
      cells[cy][cx] = 'c';
      reverse();
    }

    if (cells[cy][cx] == 'i')
      ++got_infected;
    step();
  }

  void increase_size() {
    for (int i = 0; i < size; ++i) {
      cells[i].insert(cells[i].begin(), 'c');
      cells[i].push_back('c');
    }

    size += 2;

    std::vector<char> l;
    for (int j = 0; j < size; ++j)
      l.push_back('c');
    cells.insert(cells.begin(), l);
    cells.push_back(l);
    ++cx;
    ++cy;
  }

  void print() {
    for (auto cl : cells) {
      for (auto c : cl) {
        if (c == 'i')
          std::cout << "#";
        else if (c == 'c')
          std::cout << ".";
        else
          std::cout << c;
      }
      std::cout << '\n';
    }
  }

  void turn_on(int i, int j) { cells[j][i] = 'i'; }

  int get_infected() { return got_infected; }
};

int main() {
  grid *g = nullptr;

  int cx = 0;
  int cy = 0;
  std::string line;
  while (getline(std::cin, line)) {
    if (g == nullptr) {
      g = new grid(line.size());
    }
    cx = 0;
    for (auto c : line) {
      if (c == '#')
        g->turn_on(cx, cy);
      ++cx;
    }
    ++cy;
  }

  if (g != nullptr) {
    for (int i = 0; i < 10000000; ++i) {
      g->burst();
    }
    logger::get(logtype::logINFO) << "Part 2: " << g->get_infected() << '\n';

    delete g;
  } else {
    logger::get(logtype::logERROR)
        << "Input invalid: no part 2 executed" << '\n';
  }

  return 0;
}