#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

#include "logger.h"
// #include "basetimer.h"

std::map<char, int64_t> make_histogram(std::string id) {
  std::map<char, int64_t> res;
  for (auto c : id)
    res[c]++;
  return res;
}

bool check(std::map<char, int64_t> hist, int64_t count) {
  for (auto m : hist)
    if (m.second == count)
      return true;
  return false;
}

int64_t compare(std::string s1, std::string s2) {
  int l = s1.length();
  int64_t c = 0;
  for (int i = 0; i < l; ++i)
    if (s1[i] != s2[i])
      ++c;
  return c;
}

std::string remove_duplicates(std::string s1, std::string s2) {
  int l = s1.length();
  std::string res = "";
  for (int i = 0; i < l; ++i)
    if (s1[i] == s2[i])
      res += s1[i];
  return res;
}

int main() {
  std::vector<std::string> input;

  int64_t count2, count3;
  count2 = 0;
  count3 = 0;
  std::string line;
  while (getline(std::cin, line)) {
    input.push_back(line);
    std::map<char, int64_t> hist = make_histogram(line);
    count2 += (check(hist, 2) ? 1 : 0);
    count3 += (check(hist, 3) ? 1 : 0);
  }
  logger::get(logtype::logINFO) << "Part 1: " << count2 * count3 << "\n";

  std::string c1, c2;
  for (auto s1 : input) {
    for (auto s2 : input) {
      if (s1.compare(s2) != 0) {
        if (compare(s1, s2) == 1) {
          c1 = s1;
          c2 = s2;
        }
      }
    }
  }
  std::string result = remove_duplicates(c1, c2);
  logger::get(logtype::logINFO) << "Part 2: " << result << "\n";
  return 0;
}