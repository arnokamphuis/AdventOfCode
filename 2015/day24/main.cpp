#include "logger.h"
#include <algorithm>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

class packing {
  int packs;
  std::vector<int> packages;

public:
  packing(int p) : packs(p) {}

  void addpackage(int p) { packages.push_back(p); }

  int64_t run() {
    std::sort(packages.begin(), packages.end(), std::greater<int>());

    int sum = std::accumulate(packages.begin(), packages.end(), 0);
    int package_weight = sum / packs;

    std::map<int, std::set<std::vector<int>, std::greater<>>> sizesets;
    std::set<std::vector<int>, std::greater<>> sets;

    subset_sum(sets, packages, package_weight);

    for (auto s : sets)
      sizesets[s.size()].insert(s);

    int smallest = std::numeric_limits<int>::max();
    int largest = std::numeric_limits<int>::min();

    for (auto ss : sizesets) {
      if (smallest > ss.first)
        smallest = ss.first;
      if (largest < ss.first)
        largest = ss.first;
    }

    return min_entropy(sizesets[smallest]);
  }

  int64_t entropy(std::vector<int> set) {
    int64_t ent = 1;
    for (auto s : set)
      ent *= (int64_t)s;
    return ent;
  }

  int64_t min_entropy(std::set<std::vector<int>, std::greater<>> sets) {
    int64_t minimum = std::numeric_limits<int64_t>::max();
    for (auto set : sets) {
      int64_t ent = entropy(set);
      if (ent < minimum)
        minimum = ent;
    }
    return minimum;
  }
  void subset_sum(std::set<std::vector<int>, std::greater<>> &sets,
                  std::vector<int> numbers, int target,
                  std::vector<int> partial = {}) {
    int s = std::accumulate(partial.begin(), partial.end(), 0);

    if (s == target) {
      sets.insert(partial);
    }
    if (s >= target) {
      return;
    }

    for (int i = 0; i < numbers.size(); ++i) {
      int n = numbers[i];
      std::vector<int> next_partial = partial;
      next_partial.push_back(n);

      std::vector<int> remaining;
      for (int j = i + 1; j < numbers.size(); ++j) {
        remaining.push_back(numbers[j]);
      }
      subset_sum(sets, remaining, target, next_partial);
    }
  }
};

void print_set(std::vector<int> set) {
  std::cout << "(";
  for (auto s : set)
    std::cout << s << ",";
  std::cout << ")";
}

int main() {
  int n;
  packing p1(3);
  packing p2(4);
  while (std::cin >> n) {
    p1.addpackage(n);
    p2.addpackage(n);
  }
  logger::get(logtype::logINFO) << "Part 1: " << p1.run() << std::endl;
  logger::get(logtype::logINFO) << "Part 2: " << p2.run() << std::endl;
  return 0;
}