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

using pos = std::vector<int>;

std::vector<std::string> split(const char *str, char c = ' ') {
  std::vector<std::string> result;

  do {
    const char *begin = str;

    while (*str != c && *str)
      str++;

    result.push_back(std::string(begin, str));
  } while (0 != *str++);

  return result;
}

int distance(pos p1, pos p2) {
  return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1]) + abs(p1[2] - p2[2]) +
         abs(p1[3] - p2[3]);
}

void print_constellations(std::set<std::set<pos>> &constellations) {
  for (auto con : constellations) {
    std::cout << "{";
    for (auto e : con) {
      std::cout << "(";
      for (auto v : e)
        std::cout << v << ",";
      std::cout << "),";
    }
    std::cout << "}\n";
  }
}

std::set<std::set<pos>>
merge_constellations(const std::set<std::set<pos>> &current, int dist = 3) {
  std::set<std::set<pos>> newset;

  for (auto s1 : current) {
    std::set<std::set<pos>> tobemerged;
    tobemerged.insert(s1);
    for (auto s2 : current) {
      if (s1 != s2) {
        bool merge = false;
        for (auto e1 : s1) {
          for (auto e2 : s2) {
            if (distance(e1, e2) <= dist) {
              // merge two sets into new set
              tobemerged.insert(s2);
            }
          }
        }
      }
    }
    if (tobemerged.size() > 1) {
      std::set<pos> ns;
      for (auto tbm : tobemerged)
        for (auto e : tbm)
          ns.insert(e);
      newset.insert(ns);
    } else {
      newset.insert(s1);
    }
  }

  // print_constellations(newset);

  return newset;
}

std::set<std::set<pos>>
find_constellations(const std::set<std::set<pos>> &current,
                    std::vector<pos> all, int dist = 3) {
  std::set<std::set<pos>> newset;

  for (auto cs : current) {
    std::set<pos> ns;
    for (auto v : all) {
      for (auto e : cs) {
        ns.insert(e);
        if (distance(v, e) <= dist) {
          ns.insert(v);
        }
      }
    }
    newset.insert(ns);
  }
  return newset;
}

int main() {

  std::vector<pos> positions;
  std::set<std::set<pos>> constellations;

  std::string line;
  while (getline(std::cin, line)) {
    std::vector<std::string> splits = split(line.c_str(), ',');
    pos pos4d = {std::atoi(splits[0].c_str()), std::atoi(splits[1].c_str()),
                 std::atoi(splits[2].c_str()), std::atoi(splits[3].c_str())};

    positions.push_back(pos4d);
    constellations.insert({pos4d});
  }

  // print_constellations(constellations);

  constellations = find_constellations(constellations, positions, 3);
  int postsize = -1;
  int presize = constellations.size();
  while (presize != postsize) {
    presize = constellations.size();
    constellations = merge_constellations(constellations, 3);
    postsize = constellations.size();
  }

  logger::get(logtype::logINFO)
      << "Part 1: " << constellations.size() << std::endl;

  return 0;
}