#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

std::map<char, std::pair<int, int>> delta;
std::vector<std::pair<int, int>> positions;

int main() {

  delta['N'] = std::make_pair(0, -1);
  delta['E'] = std::make_pair(+1, 0);
  delta['S'] = std::make_pair(0, +1);
  delta['W'] = std::make_pair(-1, 0);

  std::string line;
  getline(std::cin, line);

  char c;
  std::stringstream ss(line);

  std::pair<int, int> pos = std::make_pair(0, 0);
  std::pair<int, int> prev = std::make_pair(0, 0);

  // contains the shortest distance to the start
  std::map<std::pair<int, int>, int> distances;

  while (ss >> c) {

    if (c == '^') {
    } else if (c == '$')
      break;

    else if (c == '(') {
      positions.push_back(pos);

    } else if (c == ')') {
      positions.erase(positions.end() - 1);

    } else if (c == '|') {
      pos = *(positions.end() - 1);

    } else {
      std::pair<int, int> d = delta[c];

      pos.first += d.first;
      pos.second += d.second;

      // apparently you can come back to a previously visited room :-( I forgot
      // this earlier
      if (distances.find(pos) != distances.end()) {
        distances[pos] = std::min(distances[pos], distances[prev] + 1);
      } else {
        distances[pos] = distances[prev] + 1;
      }
    }
    prev = pos;
  }

  int count1000plus = 0;
  int max = std::numeric_limits<int>::min();
  for (auto d : distances) {
    if (d.second >= 1000)
      ++count1000plus;
    if (d.second > max)
      max = d.second;
  }

  logger::get(logtype::logINFO) << "Part 1: " << max << std::endl;
  logger::get(logtype::logINFO) << "Part 2: " << count1000plus << std::endl;

  return 0;
}