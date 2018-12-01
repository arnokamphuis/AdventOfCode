#include <algorithm>
#include <iostream>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

#include "logger.h"
// #include "basetimer.h"

int main() {
  int64_t in = 0;

  // BaseTimer timer;

  std::vector<int64_t> input;
  std::set<int64_t> frequencies;

  while (!std::cin.eof()) {
    std::cin >> in;
    input.push_back(in);
  }

  int64_t total = 0;
  for (auto i : input)
    total += i;

  logger::get(logtype::logINFO) << "Part 1: " << total << "\n";

  int i = 0;
  int size = input.size();
    int64_t freq = 0;

  // int runs=100;
  // timer.start();
  // for (int run = 0; run<runs; ++run) {
    freq = 0;
    frequencies.clear();
    frequencies.insert(freq);
    while (true) {
      freq += input[i++];
      if (!frequencies.insert(freq).second)
        break;
      i = (i==size?0:i);
    }
  // }
  // timer.stop();
  // std::cout << "timer: " << timer.elapsedMilliSeconds()/(float)runs << "\n";

  logger::get(logtype::logINFO) << "Part 2: " << freq << "\n";

  return 0;
}