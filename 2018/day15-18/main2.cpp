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

enum gnometype { ELF = 1, GOBLIN = 2 };
enum direction { RIGHT, UP, LEFT, DOWN, NONE };

class field {
  int w;
  int h;

  std::vector<std::vector<char>> cells;
  std::vector<std::vector<int>> ff;

  void initializefloodfill() {
    for (auto row : cells) {
      std::vector<int> ffrow;
      ffrow.resize(row.size());
      ff.push_back(ffrow);
    }
  }
  int postinsertrow() {
    std::vector<char> row;
    row.resize(w);
    cells.push_back(row);
    return cells.size() - 1;
  }

public:
  field() {
    w = 0;
    h = 0;
  }

  void addline(std::string line) {
    if (w == 0)
      w = line.length();

    int y = postinsertrow();
    for (int x = 0; x < w; ++x) {
      cells[y][x] = line[x];
    }
  }

  std::vector<std::pair<std::pair<int, int>, gnometype>> getgnomes() {
    std::vector<std::pair<std::pair<int, int>, gnometype>> ret;
    for (int x = 0; x < w; ++x) {
      for (int y = 0; y < w; ++y) {
        char c = cells[y][x];
        if ((c == 'G') || (c == 'E'))
          ret.push_back(std::make_pair(std::make_pair(x, y),
                                       ((c == 'G') ? GOBLIN : ELF)));
      }
    }
    return ret;
  }

  std::vector<std::vector<int>> floodfill(int x, int y) {
    if (ff.size() == 0)
      initializefloodfill();

    std::vector<std::vector<int>> cff = ff;

    std::priority_queue<std::pair<int, std::pair<int, int>>,
                        std::vector<std::pair<int, std::pair<int, int>>>,
                        std::greater<std::pair<int, std::pair<int, int>>>>
        open;

    open.push(std::make_pair(0, std::make_pair(x, y)));
    cff[y][x] = 0;

    while (open.size() > 0) {
      std::pair<int, std::pair<int, int>> o = open.top();
      open.pop();

      std::pair<int, int> current = o.second;
      std::vector<std::pair<int, int>> nbs;

      std::pair<int, int> nb = current;
      nb.first += 1;
      nbs.push_back(nb);
      nb.first -= 2;
      nbs.push_back(nb);
      nb.first += 1;

      nb.second += 1;
      nbs.push_back(nb);
      nb.second -= 2;
      nbs.push_back(nb);

      for (auto cnb : nbs) {

        if ((cells[cnb.second][cnb.second] == '.') &&
            (cff[cnb.second][cnb.second] >
             (cff[current.second][current.first] + 1))) {

          cff[cnb.second][cnb.first] = cff[current.second][current.first] + 1;

          open.push(std::make_pair(cff[cnb.first][cnb.second], cnb));
        }
      }
    }

    return cff;
  }

  bool isfree(int x, int y) const { return (cells[y][x] == '.'); }

  void move(int px, int py, int x, int y) {
    cells[y][x] = cells[py][px];
    cells[py][px] = '.';
  }
};

class gnome {
  int posx;
  int posy;
  int id;
  gnometype gt;

public:
  gnome(int x, int y, int nr, gnometype t) : posx(x), posy(y), id(nr), gt(t) {}

  friend inline bool operator<(const gnome &lhs, const gnome &rhs) {
    if (lhs.posy < rhs.posy)
      return true;
    if ((lhs.posy == rhs.posy) && (lhs.posx < rhs.posx))
      return true;
    return false;
  }

  friend bool operator==(const gnome &lhs, const gnome &rhs) {
    return (lhs.id == rhs.id);
  }

  int x() const { return posx; }
  int y() const { return posy; }
  gnometype gettype() const { return gt; }
  gnometype opposite() const { return (gt == ELF ? GOBLIN : ELF); }

  int distance(const gnome &og) const {
    return abs(posx - og.posx) + abs(posy - og.posy);
  }

  std::vector<std::pair<int, int>> neighbours() {
    std::vector<std::pair<int, int>> ret;
    ret.push_back(std::make_pair(posx + 1, posy + 0));
    ret.push_back(std::make_pair(posx - 1, posy + 0));
    ret.push_back(std::make_pair(posx + 0, posy - 1));
    ret.push_back(std::make_pair(posx + 0, posy + 1));
    return ret;
  }

  void move(int newx, int newy, field *f) {
    f->move(posx, posy, newx, newy);
    posx = newx;
    posy = newy;
  }
};

class battle {
  field *f;
  std::vector<gnome> gnomes;
  int fullrounds;

  void initializegnomes() {
    gnomes.clear();
    std::vector<std::pair<std::pair<int, int>, gnometype>> fetchedgnomes =
        f->getgnomes();
    for (auto fg : fetchedgnomes)
      gnome g(fg.first.first, fg.first.second, gnomes.size() + 1, fg.second);
  }

  bool checkinrange(const gnome &g) {
    for (const auto &og : gnomes)
      if ((g.opposite() == og.gettype()) && (g.distance(og) == 1))
        return true;
    return false;
  }

  std::vector<std::pair<int, int>> inrange(const gnome &g) {
    std::vector<std::pair<int, int>> ret;
    for (auto &og : gnomes) {
      if (g.gettype() == og.opposite()) {
        std::vector<std::pair<int, int>> temp = og.neighbours();
        for (auto t : temp) {
          if (f->isfree(t.first, t.second))
            ret.push_back(t);
        }
      }
    }
    return ret;
  }

  std::vector<std::pair<int, int>> findtargets(const gnome &g) {
    std::vector<std::pair<int, int>> targets;
    std::vector<std::pair<int, int>> potentials = inrange(g);

    std::vector<std::vector<int>> cff = f->floodfill(g.x(), g.y());

    int closestdistance = std::numeric_limits<int>::max();

    for (auto p : potentials)
      if (cff[p.first][p.second] < closestdistance)
        closestdistance = cff[p.first][p.second];

    if (closestdistance != std::numeric_limits<int>::max())
      for (auto p : potentials)
        if (cff[p.first][p.second] == closestdistance)
          targets.push_back(p);

    return targets;
  }

  std::pair<int, int> findnextposition(const gnome &g,
                                       std::pair<int, int> target) {

    std::vector<std::vector<int>> firststeps =
        f->floodfill(target.first, target.second);

    int mindist = std::numeric_limits<int>::max();
    int x = g.x();
    int y = g.y();

    int stepx = -1;
    int stepy = -1;
    if (firststeps[y - 1][x + 0] < mindist) {
      mindist = firststeps[y - 1][x + 0];
      stepx = x;
      stepy = y - 1;
    }
    if (firststeps[y + 0][x - 1] < mindist) {
      mindist = firststeps[y + 0][x - 1];
      stepx = x - 1;
      stepy = y;
    }
    if (firststeps[y + 0][x + 1] < mindist) {
      mindist = firststeps[y + 0][x + 1];
      stepx = x + 1;
      stepy = y;
    }
    if (firststeps[y + 1][x + 0] < mindist) {
      mindist = firststeps[y + 1][x + 0];
      stepx = x;
      stepy = y + 1;
    }

    return std::make_pair(stepx, stepy);
  }

  bool enemiesalive(const gnome &g) {
    gnometype t = g.gettype();
    for (auto &og : gnomes)
      if (og.opposite() == t)
        return true;
    return false;
  }

  void sortgnomes() { std::sort(gnomes.begin(), gnomes.end(), std::less<>()); }

  void sortpairpos(std::vector<std::pair<int, int>> &vec) {
    std::sort(
        vec.begin(), vec.end(),
        [](const std::pair<int, int> &lhs, const std::pair<int, int> &rhs) {
          if (lhs.second < rhs.second)
            return true;
          else if ((lhs.second == rhs.second) && (lhs.first < rhs.first))
            return true;
          else
            return false;
        });
  }

public:
  battle() : fullrounds(0) {}

  void setfield(field *fr) {
    f = fr;
    initializegnomes();
  }

  bool round() {
    sortgnomes();
    for (auto &g : gnomes) {
      if (!enemiesalive(g))
        return false;
      turn(g);
    }
    ++fullrounds;
    return true;
  }

  void turn(gnome &g) { // returns true if turn killed other gnome, false if
                        // everything is normal

    if (!checkinrange(g)) { // need to move

      std::vector<std::pair<int, int>> targets = findtargets(g);

      if (targets.size() > 0) { // found at least one target

        sortpairpos(targets);
        std::pair<int, int> target = targets[0];
        std::pair<int, int> nextpos = findnextposition(g, target);
        g.move(nextpos.first, nextpos.second, f);
      }
    }

    // attack
    // ...
  }
};

int main() {
  battle b;
  field f;

  std::string line;
  while (getline(std::cin, line))
    f.addline(line);

  b.setfield(&f);

  return 0;
}