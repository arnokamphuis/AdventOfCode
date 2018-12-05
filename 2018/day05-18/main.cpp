#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vector>
#include <wchar.h>

//#include "basetimer.h"
#include "logger.h"

void print(std::vector<int> in) {
  for (auto i : in)
    std::cout << (char)i;
  std::cout << '\n';
}

std::vector<int> collapse(std::vector<int> input) {
  for (int i = 0; i < input.size() - 1; ++i) {
    int diff = abs(input[i] - input[i + 1]);

    if (diff == 32) {
      input.erase(input.begin() + i);
      input.erase(input.begin() + i);
      i = i - 2;
      if (i < 0)
        i = -1;
    }
  }
  return input;
}

std::vector<int> react(int c, std::vector<int> input) {
  for (int i = 0; i < input.size() - 1; ++i) {
    if ((input[i] == c) || (input[i] == (c + 32))) {
      input.erase(input.begin() + i);
      --i;
    }
  }
  return input;
}

int main() {

  std::vector<int> input;
  std::vector<int> orig;

  int ch = 0;
  while ((ch = getchar()) != EOF)
    input.push_back(ch);

  logger::get(logtype::logINFO) << "Part 1: " << collapse(input).size() << '\n';

  int min_size = 100000;
  int index = 64;
  for (int a = 65; a < 91; ++a) {
    int s = collapse(react(a, input)).size();
    if (s < min_size) {
      min_size = s;
      index = a;
    }
  }

  logger::get(logtype::logINFO) << "Part 2: " << min_size << '\n';

  return 0;
}