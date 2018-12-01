#include <algorithm>
#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

#include "logger.h"

int main() {
  int total = 0;
  int in = 0;

  std::vector<int> input;
  std::vector<int> frequencies;

  while (!std::cin.eof()) {
    std::cin >> in;
    input.push_back(in);
  }

  for (auto i : input)
    total += i;

  logger::get(logtype::logINFO) << "Part 1: " << total << "\n";

  int freq = 0;
  int i = 0;
  int size = input.size();
  while (true) {
    in = input[i];
    ++i;
    freq += in;
    if (frequencies.size() > 0) {
      auto found = std::find(frequencies.begin(), frequencies.end(), freq);
      if (freq == *found)
        break;
    }
    frequencies.push_back(freq);
    if (i == size)
      i = 0;
  }

  logger::get(logtype::logINFO) << "Part 2: " << freq << "\n";

  return 0;
}