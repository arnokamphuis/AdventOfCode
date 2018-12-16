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

  bool attacked(gnome &og) {
    hitpoints -= og.getattackpower();
    // std::cout << "gnome " << getid() << " is attacked by " << og.getid()
    //           << "\n";
    return (hitpoints <= 0);
  }

  bool stillalive() { return hitpoints > 0; }

  int distance(const gnome &other) const {
    // std::cout << "Calculating distance: " << id << " (" << position.first <<
    // ","
    //           << position.second << ") to ";
    // std::cout << other.getid() << " (" << other.position.first << ","
    //           << other.position.second << ")\n";
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
      std::cout << std::endl;
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

  bool othergnomesalive(gnome &g) {
    for (auto og : gnomes) {
      if ((og.gettype() != g.gettype()) && (og.stillalive()))
        return true;
    }
    return false;
  }

  bool turn(int round) {
    bool fullround = true;
    // std::cout << "Round " << round << " started: " << std::endl;
    sort(gnomes);
    // std::cout << "Sorted gnomes" << std::endl;
    int testid = -1;
    for (auto &g : gnomes) {
      if (g.stillalive()) {

        // std::cout << "Checking whether other gnomes are alive..." <<
        // g.getid()
        //           << std::endl;
        if (!othergnomesalive(g))
          fullround = false;

        // std::cout << "Checking if they are within range of " << g.getid()
        //           << std::endl;
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

          if (closestdistance != std::numeric_limits<int>::max()) {
            for (auto p : potentials) {
              if (cff[p.first][p.second].first == closestdistance) {
                targets.push_back(p);
              }
            }
          }

          if (g.getid() == testid) {
            print(targets, '@');
            std::cout << "Closest distance: " << closestdistance << std::endl;
          }

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
          if (g.getid() == testid) {
            std::cout << "No need to move: " << g.getid() << " (" << g.gettype()
                      << ")\n";
          }
        }

        if (checkinrange(g)) { // attack
          if (g.getid() == testid) {
            std::cout << "Attacking...\n";
          }

          std::vector<gnome> enemies;
          for (auto &og : gnomes) {
            if ((og.getid() != g.getid()) && (og.gettype() != g.gettype()) &&
                (g.distance(og) == 1) && og.stillalive())
              enemies.push_back(og);
          }

          if (g.getid() == testid) {
            std::cout << "Potentially " << enemies.size()
                      << " enemies nearby.\n";
          }

          if (enemies.size() > 0) {
            int minhitpoints = std::numeric_limits<int>::max();
            for (auto og : enemies) {
              if (minhitpoints > og.gethitpoints())
                minhitpoints = og.gethitpoints();
            }
            if (g.getid() == testid) {
              std::cout << "minimum hit points: " << minhitpoints << "\n";
            }

            for (int i = 0; i < enemies.size(); ++i) {
              if (enemies[i].gethitpoints() != minhitpoints) {
                enemies.erase(enemies.begin() + i);
                --i;
              }
            }

            if (g.getid() == testid) {
              std::cout << "selected number of enemies: " << enemies.size()
                        << "\n";
            }

            if (enemies.size() > 1)
              sort(enemies);

            int enemyid = enemies[0].getid();

            if (g.getid() == testid) {
              std::cout << "ID of enemy: " << enemyid << "\n";
            }

            bool died = false;
            for (auto &og : gnomes) {
              if (g.getid() == testid)
                std::cout << og.getid() << ",";

              if (og.getid() == enemyid) {
                if (og.attacked(g)) {
                  // std::cout << g.getid() << " killed " << og.getid() << "\t"
                  //           << og.gethitpoints() << std::endl;
                }
              }
            }
          } else { // no attack possible, therefore end turn
          }
        }
      }
    }

    // std::cout << "Round " << round << " finished. Cleaning up dead gnomes\n";

    for (int i = 0; i < gnomes.size(); ++i) {
      if (!gnomes[i].stillalive()) {
        // std::cout << "Removing " << gnomes[i].getid() << "located at ("
        //           << gnomes[i].x() << "," << gnomes[i].y() << "\n";
        cells[gnomes[i].x()][gnomes[i].y()] = '.';
        gnomes.erase(gnomes.begin() + i);
        --i;
      }
    }
    // std::cout << "===============================================" <<
    // std::endl; print(); std::cout <<
    // "===============================================" << std::endl;
    return fullround;
  }

  int battle() {
    int count = 0;
    sort(gnomes);

    while (turn(count + 1)) {
      // std::cout << "done with turn: print...." << std::endl;
      // print();
      // printfitness();
      // std::cout << "done with printing" << std::endl;
      ++count;
    }

    // print();
    // printfitness();
    // std::cout << "Total rounds: " << count << "\n";

    return count * total_hitpoint();
  }

  int total_hitpoint() {
    int total = 0;
    for (auto g : gnomes) {
      total += g.gethitpoints();
    }
    return total;
  }

  void printfitness() {
    sort(gnomes);
    std::cout
        << "-------------------------------------------------------------\n";
    for (auto g : gnomes) {
      std::cout << (g.gettype() == 0 ? 'E' : 'G') << "-" << g.getid() << "("
                << g.gethitpoints() << ")\n";
    }
    std::cout
        << "-------------------------------------------------------------\n";
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

  int battlescore = m.battle();
  logger::get(logtype::logINFO) << "Part 1: " << battlescore << "\n";

  return 0;
}