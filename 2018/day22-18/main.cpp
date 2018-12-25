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

enum cavetype { ROCKY = 0, WET = 1, NARROW = 2 };
enum tool { CLIMBING = 0, TORCH = 1, NEITHER = 2, TOOLNOTFOUND = -1 };

using position = std::pair<int, int>;
using postool = std::pair<position, tool>;
using maptype = std::map<postool, int>;
using mappair = std::pair<postool, int>;

struct comparewithheuristic {
  int tx;
  int ty;
  comparewithheuristic(int x, int y) : tx(x), ty(y) {}
  bool operator()(const mappair &left, const mappair &right) const {
    auto posleft = left.first.first;
    auto posright = right.first.first;
    int dleft = std::abs(posleft.first - tx) + std::abs(posleft.first - ty);
    int dright = std::abs(posright.first - tx) + std::abs(posright.first - ty);
    return (left.second + dleft) < (right.second + dright);
  }
};

struct cmpmappair {
  bool operator()(const mappair &left, const mappair &right) const {
    return left.second < right.second;
  }
};
class cave {
  std::map<std::pair<int, int>, uint64_t> geoindex;
  std::map<std::pair<int, int>, cavetype> types;

  maptype map;
  maptype front;

  postool getsmallest() {
    mappair min =
        *min_element(front.begin(), front.end(), comparewithheuristic(0, 0));
    return min.first;
  }

  tool remainingtool(cavetype ct, tool t) {
    if (canuse(ct, CLIMBING) && (t != CLIMBING))
      return CLIMBING;
    if (canuse(ct, NEITHER) && (t != NEITHER))
      return NEITHER;
    if (canuse(ct, TORCH) && (t != TORCH))
      return TORCH;
    return TOOLNOTFOUND;
  }

  bool canuse(cavetype ct, tool t) {
    if ((ct == ROCKY) && (t == CLIMBING))
      return true;
    if ((ct == ROCKY) && (t == TORCH))
      return true;

    if ((ct == WET) && (t == CLIMBING))
      return true;
    if ((ct == WET) && (t == NEITHER))
      return true;

    if ((ct == NARROW) && (t == TORCH))
      return true;
    if ((ct == NARROW) && (t == NEITHER))
      return true;

    return false;
  }

  int findpath() {

    int maxx = 0;
    int maxy = 0;

    std::vector<position> directions = {
        std::make_pair(0, -1), std::make_pair(+1, 0), std::make_pair(0, +1),
        std::make_pair(-1, 0)};

    postool tt;
    tt.first = target;
    tt.second = TORCH;

    auto cmp = [](const mappair &left, const mappair &right) {
      return left.second > right.second;
    };
    std::priority_queue<mappair, std::vector<mappair>, decltype(cmp)> pq(cmp);

    postool zero;
    zero.first.first = 0;
    zero.first.second = 0;
    zero.second = TORCH;
    map[zero] = 0;

    mappair current;
    current.first = zero;
    current.second = 0;
    pq.push(current);

    while ((map.find(tt) == map.end()) && (pq.size() != 0)) {
      current = pq.top();
      pq.pop();

      if (current.first.first.first > maxx)
        maxx = current.first.first.first;
      if (current.first.first.second > maxy)
        maxy = current.first.first.second;

      mappair next = current;
      postool &nextpt = next.first;
      position &nextpos = nextpt.first;
      tool &nexttool = nextpt.second;

      nexttool =
          remainingtool(gettype(nextpos.first, nextpos.second), nexttool);

      if (map.find(nextpt) != map.end()) { // already visited
        if (map[nextpt] > (map[current.first] + 7)) {
          map[nextpt] = map[current.first] + 7;
          next.second = map[nextpt];
          pq.push(next);
        }
      } else {
        map[nextpt] = map[current.first] + 7;
        next.second = map[nextpt];
        pq.push(next);
      }

      for (auto d : directions) {
        next = current;
        postool &nextpt = next.first;
        position &nextpos = nextpt.first;
        tool &nexttool = nextpt.second;

        nextpos.first += d.first;
        nextpos.second += d.second;

        if ((nextpos.first >= 0) && (nextpos.second >= 0)) {
          // still in cave
          if (canuse(gettype(nextpos.first, nextpos.second), nexttool)) {
            // allowed to use
            if (map.find(nextpt) != map.end()) { // already visited
              if (map[nextpt] > (map[current.first] + 1)) {
                map[nextpt] = map[current.first] + 1;
                next.second = map[nextpt];
                pq.push(next);
              }
            } else {
              map[nextpt] = map[current.first] + 1;
              next.second = map[nextpt];
              pq.push(next);
            }
          }
        }
      }
    }
    // std::cout << maxx << "," << maxy << std::endl;
    return map[tt];
  }

  std::pair<int, int> target;
  int depth;

public:
  cave(int tx, int ty, int d) {
    target.first = tx;
    target.second = ty;
    depth = d;
  }

  int minimumtraveltime() { return findpath(); }

  void print(int lx, int ly) {
    cavetype ct;
    for (int y = 0; y <= ly; ++y) {
      for (int x = 0; x <= lx; ++x) {
        ct = gettype(x, y);
        char c;
        if ((x == 0) && (y == 0))
          c = 'M';
        else if ((x == target.first) && (y == target.second))
          c = 'T';
        else if (ct == ROCKY)
          c = '.';
        else if (ct == WET)
          c = '=';
        else if (ct == NARROW)
          c = '|';
        std::cout << c;
      }
      std::cout << "\n";
    }
  }

  uint64_t calculatedanger(int lx, int ly) {
    uint64_t danger = 0;
    for (int x = 0; x <= lx; ++x) {
      for (int y = 0; y <= ly; ++y) {
        danger += risklevel(x, y);
      }
    }
    return danger;
  }

  uint64_t calculateerosionlevel(int x, int y) {
    return (calculategeoindex(x, y) + depth) % 20183;
  }

  int risklevel(int x, int y) {
    cavetype ct = gettype(x, y);
    return (int)ct;
  }

  cavetype gettype(int x, int y) {
    auto pos = std::make_pair(x, y);
    if (types.find(pos) != types.end())
      return types[pos];
    else {
      uint64_t el = calculateerosionlevel(x, y);
      // std::cout << el << "\t" << el % 3 << std::endl;
      cavetype ct = (cavetype)(el % 3);
      types[pos] = ct;
      return ct;
    }
  }

  uint64_t calculategeoindex(int x, int y) {
    uint64_t gi = 0;
    std::pair<int, int> pos = std::make_pair(x, y);
    if (geoindex.find(pos) != geoindex.end())
      return geoindex[pos];
    else {
      if ((x == 0) && (y == 0)) {
        gi = 0;
      } else if ((x == target.first) && (y == target.second)) {
        gi = 0;
      } else if (y == 0) {
        gi = x * 16807;
      } else if (x == 0) {
        gi = y * 48271;
      } else {
        gi = calculateerosionlevel(x - 1, y) * calculateerosionlevel(x, y - 1);
      }

      geoindex[pos] = gi;
      return gi;
    }
  }
};

int main() {

  // real
  int depth = 3339;
  int x = 10;
  int y = 715;

  // real esther
  // int depth = 7305;
  // int x = 13;
  // int y = 734;

  // test
  // int depth = 510;
  // int x = 10;
  // int y = 10;

  cave c(x, y, depth);

  logger::get(logtype::logINFO)
      << "Part 1: " << c.calculatedanger(x, y) << std::endl;
  logger::get(logtype::logINFO)
      << "Part 2: " << c.minimumtraveltime() << std::endl;

  return 0;
}