#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <math.h>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

enum unittype { ELF = 0, GOBLIN = 1, UNDECIDED = -1 };
using position = std::pair<int, int>;

bool operator==(const position &p1, const position &p2) {
  return ((p1.first == p2.first) && (p1.second == p2.second));
}

class unit {
  int id;
  int x;
  int y;
  unittype ut;
  int hitpoints;
  int attackpower;

public:
  unit(int nr, int ix, int iy, unittype u, int hp = 200, int ap = 3)
      : id(nr), x(ix), y(iy), ut(u), hitpoints(hp) {
    if (u == GOBLIN)
      attackpower = 3;
    else
      attackpower = ap;
  }

  friend std::ostream &operator<<(std::ostream &os, unit &u) {
    os << u.gettypechar() << u.id << "(" << u.hitpoints << ")";
    return os;
  }

  const int &getid() const { return id; }
  const int &getx() const { return x; }
  const int &gety() const { return y; }
  position getpos() const { return std::make_pair(x, y); }
  void setpos(const position &p) {
    x = p.first;
    y = p.second;
  }

  const unittype &gettype() const { return ut; }
  char gettypechar() const { return (ut == ELF ? 'E' : 'G'); }

  const int &gethitpoints() const { return hitpoints; }

  unittype getopposition() const { return (ut == GOBLIN ? ELF : GOBLIN); }
  char getoppositionchar() const { return (ut == GOBLIN ? 'E' : 'G'); }

  bool stillalive() const { return hitpoints > 0; }

  void reducehp(int r) { hitpoints -= r; }

  void attack(unit &u) const { u.reducehp(attackpower); }

  void setattackpower(int ap) { attackpower = ap; }
};

class map {
  int width;
  int height;
  std::vector<char> cells;

  int fullrounds;
  int initialelvescount;
  int finalelvescount;

  std::vector<unit> units;

  std::vector<int> ff_basic;
  void initff() {
    ff_basic.resize(width * height, std::numeric_limits<int>::max());
  }

  inline int index(int x, int y) const { return x + y * width; }
  inline int index(const position &p) const {
    return p.first + p.second * width;
  }

  unit &getunit(position pos) {
    for (auto &u : units)
      if (u.getpos() == pos)
        return u;
  }

  void sortunits() {
    std::sort(units.begin(), units.end(), [](const unit &u1, const unit &u2) {
      if (u1.gety() < u2.gety())
        return true;
      else if ((u1.gety() == u2.gety()) && (u1.getx() < u2.getx()))
        return true;
      return false;
    });
  }

  void sortposition(std::vector<position> &positions) const {
    std::sort(positions.begin(), positions.end(),
              [](const position &p1, const position &p2) {
                if (p1.second < p2.second)
                  return true;
                else if ((p1.second == p2.second) && (p1.first < p2.first))
                  return true;
                return false;
              });
  }

  std::vector<std::pair<int, position>> getinrange(const unit &u) {
    char oc = u.getoppositionchar();
    std::vector<std::pair<int, position>> ret;
    std::vector<position> directions = {{0, -1}, {-1, 0}, {1, 0}, {0, 1}};

    for (auto d : directions) {
      position pos = {u.getx() + d.first, u.gety() + d.second};
      if (cells[index(pos)] == oc) {
        auto &enemy(getunit(pos));
        if (enemy.stillalive())
          ret.push_back(std::make_pair(enemy.gethitpoints(), pos));
      }
    }

    return ret;
  }

  std::vector<int> do_ff(position origin) {
    if (ff_basic.size() == 0)
      initff();
    std::vector<int> cff = ff_basic;

    std::vector<position> directions = {{0, -1}, {-1, 0}, {1, 0}, {0, 1}};

    std::priority_queue<std::pair<int, position>,
                        std::vector<std::pair<int, position>>,
                        std::greater<std::pair<int, position>>>
        open;

    cff[index(origin)] = 0;
    open.push(std::make_pair(0, origin));

    while (open.size() > 0) {
      std::pair<int, position> o = open.top();
      open.pop();

      position current = o.second;
      position nbs;
      for (auto d : directions) {
        nbs = current;
        nbs.first += d.first;
        nbs.second += d.second;

        if ((cells[index(nbs)] == '.') &&
            (cff[index(nbs)] > (cff[index(current)] + 1))) {
          cff[index(nbs)] = cff[index(current)] + 1;
          open.push(std::make_pair(0, nbs));
        }
      }
    }

    return cff;
  }

  std::vector<position> findinrange(unittype ut) {
    std::vector<position> ret;

    std::vector<position> directions = {{0, -1}, {-1, 0}, {1, 0}, {0, 1}};

    for (auto u : units) {
      if ((u.gettype() == ut) && (u.stillalive())) {
        for (auto d : directions) {
          position p = std::make_pair(u.getx() + d.first, u.gety() + d.second);
          if (cells[index(p)] == '.')
            ret.push_back(p);
        }
      }
    }

    std::sort(
        ret.begin(), ret.end(), [](const position &p1, const position &p2) {
          return (p1.second < p2.second
                      ? true
                      : p1.second > p2.second ? false : p1.first < p1.second);
        });
    ret.erase(std::unique(ret.begin(), ret.end()), ret.end());
    return ret;
  }

  bool stillenemies(unittype ut) {
    int numbers =
        std::count_if(units.begin(), units.end(), [&ut](const unit &u) {
          return ((u.getopposition() == ut) && u.stillalive());
        });
    return numbers > 0;
  }

  int countelves() {
    return std::accumulate(units.begin(), units.end(), 0,
                           [](int a, const unit &u) {
                             if ((u.gettype() == ELF) && (u.stillalive()))
                               return a + 1;
                             else
                               return a;
                           });
  }

public:
  map(int w, int h)
      : width(w), height(h), fullrounds(0), initialelvescount(0),
        finalelvescount(0) {
    cells.resize(w * h, ' ');
  }

  void addline(int y, std::string line, int ap = 3) {
    int x = 0;
    for (auto c : line) {
      cells[index(x, y)] = c;
      if ((c == 'G') || (c == 'E'))
        units.push_back(
            unit(units.size() + 1, x, y, (c == 'G' ? GOBLIN : ELF), 200, ap));
      ++x;
    }
  }

  void print(std::vector<position> positions = {}, char c = ' ') {
    std::cout << "----------------------\n";
    std::vector<char> pm = cells;
    for (auto p : positions)
      pm[index(p)] = c;
    for (int y = 0; y < height; ++y) {
      for (int x = 0; x < width; ++x) {
        std::cout << pm[index(x, y)];
      }
      std::cout << std::endl;
    }
    std::cout << "----------------------\n";
  }

  void printunits() {
    sortunits();
    for (auto &u : units)
      std::cout << u << " ; ";
    std::cout << std::endl;
  }

  void setattackpower(int ap) {
    for (auto &u : units) {
      if (u.gettype() == ELF) {
        u.setattackpower(ap);
      }
    }
  }

  unittype winner() {
    if (!stillenemies(ELF))
      return ELF;
    else if (!stillenemies(GOBLIN))
      return GOBLIN;
    return UNDECIDED;
  }

  bool allelvesalive() { return (initialelvescount == finalelvescount); }

  void round() {
    if (initialelvescount == 0)
      initialelvescount = countelves();

    bool fullround = true;
    sortunits();

    for (auto &u : units) {
      if (u.stillalive()) {
        if (!stillenemies(u.gettype())) {
          fullround = false;
          break;
        }
        turn(u);
      }
    }

    auto last = remove_if(units.begin(), units.end(),
                          [](const unit &u) { return !u.stillalive(); });
    units.erase(last, units.end());

    if (fullround)
      ++fullrounds;

    if (winner() != UNDECIDED) {
      finalelvescount = countelves();
    }
  }

  int score() {
    sortunits();
    int sum = std::accumulate(
        units.begin(), units.end(), 0,
        [](int a, const unit &u) { return a + u.gethitpoints(); });
    return sum * fullrounds;
  }

  void turn(unit &u) {
    if (u.stillalive()) {

      std::vector<std::pair<int, position>> targets = getinrange(u);

      if (targets.size() == 0)
        move(u);

      attack(u);
    }
  }

  void move(unit &u) {
    unittype ut = u.getopposition();
    std::vector<position> inrange = findinrange(ut);

    std::vector<int> distances = do_ff(u.getpos());

    std::vector<std::pair<int, position>> reachable;
    for (auto ir : inrange)
      if (distances[index(ir)] < std::numeric_limits<int>::max())
        reachable.push_back(std::make_pair(distances[index(ir)], ir));

    std::vector<position> pr;
    for (auto r : reachable)
      pr.push_back(r.second);

    if (reachable.size() > 0) {
      int minimum = std::numeric_limits<int>::max();
      std::for_each(reachable.begin(), reachable.end(),
                    [&minimum](std::pair<int, position> &p) {
                      if (p.first < minimum)
                        minimum = p.first;
                    });

      std::vector<position> nearest;
      std::for_each(reachable.begin(), reachable.end(),
                    [&minimum, &nearest](std::pair<int, position> &p) {
                      if (p.first == minimum)
                        nearest.push_back(p.second);
                    });

      sortposition(nearest);

      position target = nearest[0];

      distances = do_ff(target);

      std::vector<position> directions = {{0, -1}, {-1, 0}, {1, 0}, {0, 1}};
      std::vector<std::pair<int, position>> steps;
      for (auto d : directions) {
        position step = std::make_pair(u.getx() + d.first, u.gety() + d.second);
        if (cells[index(step)] == '.')
          steps.push_back(std::make_pair(distances[index(step)], step));
      }

      if (steps.size() > 0) {

        minimum = std::numeric_limits<int>::max();
        std::for_each(steps.begin(), steps.end(),
                      [&minimum](std::pair<int, position> &p) {
                        if (p.first < minimum)
                          minimum = p.first;
                      });

        nearest.clear();
        std::for_each(steps.begin(), steps.end(),
                      [&minimum, &nearest](std::pair<int, position> &p) {
                        if (p.first == minimum)
                          nearest.push_back(p.second);
                      });

        sortposition(nearest);

        position step = nearest[0];

        cells[index(step)] = u.gettypechar();
        cells[index(u.getx(), u.gety())] = '.';
        u.setpos(step);
      }
    }
  }

  void attack(unit &u) {
    std::vector<std::pair<int, position>> targets = getinrange(u);
    if (targets.size() > 0) {

      int minimum = std::numeric_limits<int>::max();
      std::for_each(targets.begin(), targets.end(),
                    [&minimum](std::pair<int, position> &p) {
                      if (p.first < minimum)
                        minimum = p.first;
                    });

      std::vector<position> weakesttargets;
      std::for_each(targets.begin(), targets.end(),
                    [&minimum, &weakesttargets](std::pair<int, position> &p) {
                      if (p.first == minimum)
                        weakesttargets.push_back(p.second);
                    });

      sortposition(weakesttargets);

      position targetpos = weakesttargets[0];
      auto &enemy(getunit(targetpos));
      u.attack(enemy);

      if (!enemy.stillalive())
        cells[index(enemy.getpos())] = '.';
    }
  }
};

int main() {
  std::vector<std::string> input;
  std::string line;

  while (getline(std::cin, line))
    input.push_back(line);

  int w = input[0].length();
  int h = input.size();

  int y = 0;
  map m(w, h);
  for (auto l : input) {
    m.addline(y, l);
    ++y;
  }

  map cleanmap = m;

  while (m.winner() == UNDECIDED)
    m.round();

  int score = m.score();

  logger::get(logtype::logINFO) << "Part 1: " << score << std::endl;

  int ap;
  for (ap = 4; true; ++ap) {
    map em = cleanmap;
    em.setattackpower(ap);
    while (em.winner() == UNDECIDED)
      em.round();
    if (em.allelvesalive()) {
      score = em.score();
      break;
    }
  }
  logger::get(logtype::logINFO) << "Part 2: " << score << std::endl;

  return 0;
}