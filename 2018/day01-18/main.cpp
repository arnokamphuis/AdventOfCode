#include <algorithm>
#include <iostream>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

#include "logger.h"

int main() {
  int64_t total = 0;
  int64_t in = 0;

  std::vector<int64_t> input;
  std::set<int64_t> frequencies;

  while (!std::cin.eof()) {
    std::cin >> in;
    input.push_back(in);
  }

  // for (auto i : input)
  //   total += i;

  // logger::get(logtype::logINFO) << "Part 1: " << total << "\n";

  int64_t freq = 0;
  frequencies.insert(0);
  int i = 0;
  int size = input.size();

  while (true) {
    in = input[i];
    ++i;
    freq += in;
    auto elem = frequencies.insert(freq);
    if (elem.second == false)
      break;
    if (i == size)
      i = 0;
  }

  logger::get(logtype::logINFO) << "Part 2: " << freq << "\n";

  return 0;
}