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

std::string createstr(const std::vector<int> &vec) {
  std::string str;
  for (auto c : vec) {
    str += '0' + c;
  }
  return str;
}

std::pair<int, int> dividescore(int s) {
  std::pair<int, int> res;
  res.first = s / 10;
  res.second = s % 10;

  // std::cout << "\ndivide: " << s << "\t" << res.first << " " << res.second
  //           << "\n";
  return res;
}

int main() {
  std::vector<int> scores;

  int pos1 = 0;
  int pos2 = 1;

  auto start1 = std::chrono::high_resolution_clock::now();

  scores.push_back(3);
  scores.push_back(7);
  scores.push_back(1);
  scores.push_back(0);

  int recipes = 260321;
  int produced = 2;
  std::vector<int> nextten;
  while ((scores.size() < recipes) || (nextten.size() < 10)) {
    int r = scores.size();
    std::pair<int, int> ds = dividescore(scores[pos1] + scores[pos2]);
    if (ds.first != 0) {
      scores.push_back(ds.first);
      if (r >= recipes)
        nextten.push_back(ds.first);
    }
    r = scores.size();
    scores.push_back(ds.second);
    if (r >= recipes)
      nextten.push_back(ds.second);
    pos1 += scores[pos1] + 1;
    pos2 += scores[pos2] + 1;
    pos1 = pos1 % scores.size();
    pos2 = pos2 % scores.size();
  }
  auto end1 = std::chrono::high_resolution_clock::now();

  std::string target = createstr(nextten);
  logger::get(logtype::logINFO)
      << "Part 1: " << target << " in "
      << std::chrono::duration_cast<std::chrono::milliseconds>(end1 - start1)
             .count()
      << " ms\n";
  target = "260321";

  scores.clear();
  pos1 = 0;
  pos2 = 1;

  auto start2 = std::chrono::high_resolution_clock::now();
  scores.push_back(3);
  scores.push_back(7);
  scores.push_back(1);
  scores.push_back(0);

  std::string last;
  for (auto t : target)
    last += " ";

  while (true) {
    int r = scores.size();
    std::pair<int, int> ds = dividescore(scores[pos1] + scores[pos2]);
    if (ds.first != 0) {
      scores.push_back(ds.first);
      last += '0' + ds.first;
      last = last.substr(1, last.length());
      if (target.compare(last) == 0)
        break;
    }
    r = scores.size();
    scores.push_back(ds.second);
    last += '0' + ds.second;
    last = last.substr(1, last.length());
    if (target.compare(last) == 0)
      break;

    pos1 += scores[pos1] + 1;
    pos2 += scores[pos2] + 1;
    pos1 = pos1 % scores.size();
    pos2 = pos2 % scores.size();
  }
  auto end2 = std::chrono::high_resolution_clock::now();

  logger::get(logtype::logINFO)
      << "Part 2: " << scores.size() - target.length() << " in "
      << std::chrono::duration_cast<std::chrono::milliseconds>(end2 - start2)
             .count()
      << " ms\n";

  return 0;
}