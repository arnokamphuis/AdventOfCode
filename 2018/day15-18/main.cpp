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

enum gnometype { ELF, GOBLIN };
enum direction { RIGHT, UP, LEFT, DOWN, NONE };

class gnome {
  std::pair<int, int> position;
  int id;
  gnometype t;
  int hitpoints;
  int attackpower;

public:
  gnome(int x, int y, int nr, gnometype gt) : id(nr), t(gt) {
    position.first = x;
    position.second = y;
    attackpower = 3;
    hitpoints = 200;
  }

  friend inline bool operator<(const gnome &lhs, const gnome &rhs) {
    if (lhs.position.second < rhs.position.second)
      return true;
    if ((lhs.position.second == rhs.position.second) &&
        (lhs.position.first < rhs.position.first))
      return true;
    return false;
  }

  int distance(const gnome &other) const {
    return abs(position.first - other.position.first) +
           abs(position.second - other.position.second);
  }

  char opposite() { return ((t == GOBLIN) ? 'E' : 'G'); }

  int &x() { return position.first; }
  int &y() { return position.second; }
  const int &getid() const { return id; }
  const gnometype &gettype() const { return t; }
  const int &gethitpoints() const { return hitpoints; }
  const int &getattackpower() const { return attackpower; }

  std::vector<std::pair<int, int>> neighbours() {
    std::vector<std::pair<int, int>> ret;
    ret.push_back(std::make_pair(position.first + 1, position.second + 0));
    ret.push_back(std::make_pair(position.first - 1, position.second + 0));
    ret.push_back(std::make_pair(position.first + 0, position.second - 1));
    ret.push_back(std::make_pair(position.first + 0, position.second + 1));
    return ret;
  }
};

class map {
  int width;
  int height;
  std::vector<gnome> gnomes;
  std::vector<std::vector<char>> cells;
  std::vector<std::vector<std::pair<int, int>>> ff;

  template <class T> void sort(std::vector<T> &vec) {
    std::sort(vec.begin(), vec.end(), std::less<>());
  }

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

  std::vector<std::pair<int, int>> inrange(const gnome &g) {
    std::vector<std::pair<int, int>> ret;
    for (auto og : gnomes) {
      if ((g.getid() != og.getid()) && (g.gettype() != og.gettype())) {
        std::vector<std::pair<int, int>> temp = og.neighbours();
        for (auto t : temp) {
          if (cells[t.first][t.second] == '.')
            ret.push_back(t);
        }
      }
    }
    return ret;
  }

  void initff() {
    for (auto row : cells) {
      std::vector<std::pair<int, int>> ffrow;
      for (auto r : row)
        ffrow.push_back(std::make_pair(std::numeric_limits<int>::max(), NONE));
      ff.push_back(ffrow);
    }
  }

  std::vector<std::vector<std::pair<int, int>>>
  floodfill(const std::pair<int, int> &pos, bool printfill = false) {
    if (ff.size() == 0)
      initff();

    // std::cout << "Doing floodfill from: " << pos.first << "\t" << pos.second
    //           << "\n";
    std::priority_queue<std::pair<int, std::pair<int, int>>,
                        std::vector<std::pair<int, std::pair<int, int>>>,
                        std::greater<std::pair<int, std::pair<int, int>>>>
        open;

    std::vector<std::vector<std::pair<int, int>>> cff = ff;

    open.push(std::make_pair(0, pos));
    cff[pos.first][pos.second].first = 0;

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

        if ((cells[cnb.first][cnb.second] == '.') &&
            (cff[cnb.first][cnb.second].first >
             (cff[current.first][current.second].first + 1))) {

          cff[cnb.first][cnb.second].first =
              cff[current.first][current.second].first + 1;

          if ((cnb.second - current.second) == +1)
            cff[cnb.first][cnb.second].second = UP;

          if ((cnb.second - current.second) == -1)
            cff[cnb.first][cnb.second].second = DOWN;

          if ((cnb.first - current.first) == +1)
            cff[cnb.first][cnb.second].second = LEFT;

          if ((cnb.first - current.first) == -1)
            cff[cnb.first][cnb.second].second = RIGHT;

          open.push(std::make_pair(cff[cnb.first][cnb.second].first, cnb));
        }
      }
    }

    if (printfill) {
      for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
          if (cff[x][y].first != std::numeric_limits<int>::max())
            std::cout << cff[x][y].first << "\t";
          else
            std::cout << " \t";
        }
        std::cout << "\n";
      }

      for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
          if (cff[x][y].first != std::numeric_limits<int>::max()) {
            if (cff[x][y].second == NONE)
              std::cout << " \t";
            if (cff[x][y].second == UP)
              std::cout << "^\t";
            if (cff[x][y].second == DOWN)
              std::cout << "v\t";
            if (cff[x][y].second == LEFT)
              std::cout << "<\t";
            if (cff[x][y].second == RIGHT)
              std::cout << ">\t";
          } else
            std::cout << " \t";
        }
        std::cout << "\n";
      }
    }

    return cff;
  }

public:
  map(int w, int h) : width(w), height(h) {
    for (int y = 0; y < h; ++y) {
      std::vector<char> col;
      for (int x = 0; x < w; ++x) {
        col.push_back(' ');
      }
      cells.push_back(col);
    }
  }

  void addline(int n, std::string line) {
    int y = n;
    int x = 0;
    for (auto l : line) {
      cells[x][y] = l;
      if ((l == 'G') || (l == 'E')) {
        gnome gn(x, y, gnomes.size() + 1, (l == 'G' ? GOBLIN : ELF));
        gnomes.push_back(gn);
      }
      ++x;
    }
  }

  void print(const std::vector<std::pair<int, int>> &items = {}, char c = ' ') {
    std::vector<std::vector<char>> pm = cells;
    for (auto i : items)
      pm[i.first][i.second] = c;
    for (int y = 0; y < height; ++y) {
      for (int x = 0; x < width; ++x)
        std::cout << pm[x][y];
      std::cout << "\n";
    }
  }

  void printorder(const std::vector<std::pair<int, int>> &items) {
    std::vector<std::vector<char>> pm = cells;
    int order = 0;
    for (auto i : items) {
      pm[i.first][i.second] = '0' + order;
      ++order;
    }
    for (int y = 0; y < height; ++y) {
      for (int x = 0; x < width; ++x)
        std::cout << pm[x][y];
      std::cout << "\n";
    }
  }

  bool checkinrange(gnome &g) const {
    if ((cells[g.x() + 1][g.y()] == g.opposite()) ||
        (cells[g.x() - 1][g.y()] == g.opposite()) ||
        (cells[g.x()][g.y() + 1] == g.opposite()) ||
        (cells[g.x()][g.y() - 1] == g.opposite()))
      return true;
    return false;
  }

  void turn() {

    sort(gnomes);

    int testid = -1;
    for (auto &g : gnomes) {
      // gnome &g = gnomes[5];
      if (!checkinrange(g)) {
        std::vector<std::pair<int, int>> targets;
        std::vector<std::pair<int, int>> potentials = inrange(g);

        // std::cout << "POSITION g: " << g.x() << "\t" << g.y() << std::endl;
        std::vector<std::vector<std::pair<int, int>>> cff =
            floodfill(std::make_pair(g.x(), g.y()), (g.getid() == testid));
        int closestdistance = std::numeric_limits<int>::max();

        if (g.getid() == testid)
          print(potentials, 'P');

        for (auto p : potentials) {
          if (cff[p.first][p.second].first < closestdistance) {
            closestdistance = cff[p.first][p.second].first;
          }
        }
        for (auto p : potentials) {
          if (cff[p.first][p.second].first == closestdistance) {
            targets.push_back(p);
          }
        }

        if (g.getid() == testid)
          print(targets, '@');

        if (targets.size() > 0) {
          sortpairpos(targets);
          if (g.getid() == testid)
            printorder(targets);

          std::pair<int, int> target = targets[0];
          std::vector<std::vector<std::pair<int, int>>> firststeps =
              floodfill(std::make_pair(target.first, target.second),
                        (g.getid() == testid));

          int mindist = std::numeric_limits<int>::max();
          int x = g.x();
          int y = g.y();

          int stepx = -1;
          int stepy = -1;
          if (firststeps[x + 0][y - 1].first < mindist) {
            mindist = firststeps[x + 0][y - 1].first;
            stepx = x;
            stepy = y - 1;
          }
          if (firststeps[x - 1][y + 0].first < mindist) {
            mindist = firststeps[x - 1][y + 0].first;
            stepx = x - 1;
            stepy = y;
          }
          if (firststeps[x + 1][y + 0].first < mindist) {
            mindist = firststeps[x + 1][y + 0].first;
            stepx = x + 1;
            stepy = y;
          }
          if (firststeps[x + 0][y + 1].first < mindist) {
            mindist = firststeps[x + 0][y + 1].first;
            stepx = x;
            stepy = y + 1;
          }

          std::vector<std::pair<int, int>> step;
          step.push_back(std::make_pair(stepx, stepy));

          if (g.getid() == testid) {
            print(step, '*');
          }

          cells[stepx][stepy] = cells[x][y];
          cells[x][y] = '.';
          g.x() = stepx;
          g.y() = stepy;
        } else {
          // std::cout << "No targets found for gnome: " << g.getid() << " ("
          //           << g.gettype() << ")\n";
        }
      } else {
        // std::cout << "No need to move: " << g.getid() << " (" << g.gettype()
        //           << ")\n";
      }

      if (checkinrange(g)) { // attack
        std::vector<gnome> enemies;
        for (auto &og : gnomes) {
          if ((og.getid() != g.getid()) && (og.gettype() != g.gettype()) &&
              (g.distance(og) == 1))
            enemies.push_back(og);
        }

        if (enemies.size() > 0) {
          int minhitpoints = std::numeric_limits<int>::max();
          for (auto og : enemies)
            if (minhitpoints > og.gethitpoints())
              minhitpoints = og.gethitpoints();

          for (int i = 0; i < enemies.size(); ++i) {
            if (enemies[i].gethitpoints() != minhitpoints) {
              enemies.erase(enemies.begin() + i);
              --i;
            }
          }

          if (enemies.size() > 1)
            sort(enemies);

          // std::cout << "enemies of " << g.getid() << " ";
          // for (auto enemy : enemies)
          //   std::cout << "(" << enemy.x() << "," << enemy.y() << ") ";
          // std::cout << "\n";

        } else { // no attack possible, therefore end turn
        }
      }
    }
    print();
  }

  void testfirstgnome() {
    sort(gnomes);

    // std::vector<std::pair<int, int>> test;
    // test.push_back(std::make_pair(7, 4));
    // print(test, 'T');

    turn();
    turn();
    turn();
    // turn();
    // turn();
  }
};

int main() {

  std::string line;
  std::vector<std::string> input;
  while (getline(std::cin, line))
    input.push_back(line);

  int w = line.length();
  int h = input.size();

  map m(w, h);
  int y = 0;
  for (auto line : input) {
    m.addline(y, line);
    ++y;
  }

  m.testfirstgnome();

  return 0;
}