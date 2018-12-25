#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

enum material {
  EMPTY = -1,
  SPRING = 0,
  FALLING = 1,
  FILL = 3,
  CLAY = 8,
  WEIRD = 100,
  WEIRD2 = 101
};

class spring {
  int x;
  int y;
  bool active;

public:
  spring(int positionx, int positiony) : x(positionx), y(positiony) {
    active = true;
  }

  const int &getx() const { return x; }
  const int &gety() const { return y; }
  const bool &getactive() const { return active; }

  void deactivate() { active = false; }

  friend bool operator<(const spring &lhs, const spring &rhs) {
    if (lhs.y < rhs.y)
      return true;
    else if ((lhs.y == rhs.y) && (lhs.x > rhs.x))
      return true;
    return false;
  }
};

class map {
  int width;
  int height;
  int left;
  int top;

  int minclay;
  int maxclay;

  std::vector<std::vector<int>> cells;

  std::vector<spring> springs;

  void preinsertcolumn() {
    for (auto &r : cells)
      r.insert(r.begin(), EMPTY);
    --left;
    ++width;
  }

  void postinsertcolumn() {
    for (auto &r : cells)
      r.push_back(EMPTY);
    ++width;
  }

  void postinsertrow() {
    std::vector<int> row;
    for (int i = 0; i < width; ++i)
      row.push_back(EMPTY);
    cells.push_back(row);
    ++height;
  }

  void setvalue(int x, int y, int v) {
    if (cells[y - top][x - left] == SPRING) {
      erasespring(x, y);
    }
    cells[y - top][x - left] = v;
    if (v == CLAY) {
      if (maxclay < y)
        maxclay = y;
      if (minclay > y)
        minclay = y;
    }
  }

  int getvalue(int x, int y) { return cells[y - top][x - left]; }

  void erasespring(int x, int y) {
    for (int i = 0; i < springs.size(); ++i) {
      if ((springs[i].getx() == x) && (springs[i].gety() == y)) {
        springs.erase(springs.begin() + i);
        break;
      }
    }
  }

  void sortsprings() {
    std::sort(springs.begin(), springs.end(), std::less<>());
  }

public:
  map() {
    width = 1;
    height = 1;
    left = 500;
    top = 0;
    cells.push_back({SPRING});
    minclay = std::numeric_limits<int>::max();
    maxclay = std::numeric_limits<int>::min();
  }

  void print() {
    char c;
    for (auto row : cells) {
      for (auto v : row) {
        if (v == CLAY)
          c = '#';
        if (v == EMPTY)
          c = '.';
        if (v == SPRING)
          c = '+';
        if (v == FILL)
          c = '~';
        if (v == FALLING)
          c = '|';
        if (v == WEIRD)
          c = '@';
        if (v == WEIRD)
          c = '$';
        std::cout << c;
      }
      std::cout << "\n";
    }
    std::cout << std::flush;
  }

  void addline(std::string line) {
    int bottom = top + height - 1;
    int right = left + width - 1;

    auto commapos = line.find(',');
    std::string leftpart = line.substr(0, commapos);
    std::string rightpart = line.substr(commapos + 2, line.length());

    char l = leftpart[0];
    int lv = std::atoi(leftpart.substr(2, leftpart.length()).c_str());
    if (l == 'y')
      while (lv > (top + height - 1))
        postinsertrow();

    if (l == 'x') {
      while (lv < left)
        preinsertcolumn();
      while (lv > (left + width - 1))
        postinsertcolumn();
    }

    char r = rightpart[0];
    auto pointpos = rightpart.find("..");
    int rv_from = std::atoi(rightpart.substr(2, pointpos - 2).c_str());
    int rv_to =
        std::atoi(rightpart.substr(pointpos + 2, rightpart.length()).c_str());

    for (int rv = rv_from; rv <= rv_to; ++rv) {
      if (r == 'x') {
        while (rv < left)
          preinsertcolumn();
        while (rv > (left + width - 1))
          postinsertcolumn();
      } else {
        while (rv > (top + height - 1))
          postinsertrow();
      }
    }

    int x, y;
    for (int rindex = rv_from; rindex <= rv_to; ++rindex) {
      if (l == 'x') {
        x = lv;
        y = rindex;
      } else {
        x = rindex;
        y = lv;
      }
      setvalue(x, y, CLAY);
    }
  }

  void prepare() {
    preinsertcolumn();
    preinsertcolumn();
    postinsertcolumn();
    postinsertcolumn();
    springs.push_back(spring(500, 0));
  }

  int fill(int x, int y) {
    int cy = y;
    int cx = x;
    int val;
    val = getvalue(cx, cy);

    if (getvalue(cx, cy + 1) == EMPTY) {
      setvalue(cx, cy, SPRING);
      springs.push_back(spring(cx, cy));
      return -2;
    } else if (val != CLAY) {
      setvalue(cx, cy, FILL);
    } else
      return -1;

    return 0;
  }

  bool willbespring(int x, int y) {
    return ((getvalue(x, y) == EMPTY) && (getvalue(x, y + 1) == EMPTY));
  }

  void fillbassin(int x, int y) {
    int cx = x;
    int cy = y;

    int f1 = -1;
    int f2 = -1;

    int starty = cy;

    while ((f1 == -1) && (f2 == -1)) {
      setvalue(cx, cy, FILL);

      for (int i = -1; (cx + i) > left; --i)
        if ((f1 = fill(cx + i, cy)) < 0)
          break;
      for (int i = 1; (cx + i) < (left + width); ++i)
        if ((f2 = fill(cx + i, cy)) < 0)
          break;

      if ((f1 == -2) || (f2 == -2)) {
        for (int i = -1;
             (getvalue(cx + i, cy) != FALLING) &&
             (getvalue(cx + i, cy) != SPRING) && (getvalue(cx + i, cy) != CLAY);
             --i)
          setvalue(cx + i, cy, FALLING);
        for (int i = 1;
             (getvalue(cx + i, cy) != FALLING) &&
             (getvalue(cx + i, cy) != SPRING) && (getvalue(cx + i, cy) != CLAY);
             ++i)
          setvalue(cx + i, cy, FALLING);
        setvalue(cx, cy, FALLING);
      }
      --cy;
    }
  }

  void dospring() {
    int springcount = 0;
    while (springs.size() != 0) {
      ++springcount;

      sortsprings();
      spring s = springs[0];
      springs.erase(springs.begin());

      int cx = s.getx();
      int cy = s.gety() + 1;

      setvalue(cx, cy + 1, WEIRD2);

      int v;
      while ((cy < height) && ((v = getvalue(cx, cy)) != CLAY) && (v != FILL)) {
        setvalue(cx, cy, FALLING);
        ++cy;

        if ((cy - top - height) > 0) {
          s.deactivate();
          break;
        }
      }
      --cy;

      if ((v == CLAY) ||
          (v == FILL)) { // need to fill resevoir and place springs
        if (s.getactive()) {
          fillbassin(cx, cy);
        }
      }
    }
  }

  int watercount() {
    int counter = 0;
    for (int y = minclay; y <= maxclay; ++y) {
      for (int x = left; x < left + width; ++x) {
        int v = getvalue(x, y);
        if ((v == FILL) || (v == FALLING) || (v == SPRING))
          ++counter;
      }
    }
    return counter;
  }

  int waterstandingcount() {
    int counter = 0;
    for (int y = minclay; y <= maxclay; ++y) {
      for (int x = left; x < left + width; ++x) {
        int v = getvalue(x, y);
        if ((v == FILL))
          ++counter;
      }
    }
    return counter;
  }

  void run() {
    while (!springs.empty())
      dospring();
  }
};

int main() {

  map m;

  std::string line;
  std::vector<std::string> input;
  while (getline(std::cin, line)) {
    input.push_back(line);
    m.addline(line);
  }
  m.prepare();
  m.dospring();

  logger::get(logtype::logINFO) << "Part 1: " << m.watercount() << "\n";
  logger::get(logtype::logINFO) << "Part 2: " << m.waterstandingcount() << "\n";
  return 0;
}