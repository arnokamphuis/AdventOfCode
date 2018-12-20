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

void print(const std::map<std::pair<int, int>, int> &distances,
           const std::map<std::pair<int, int>, std::pair<int, int>> &path) {
  int minx = std::numeric_limits<int>::max();
  int miny = std::numeric_limits<int>::max();
  int maxx = std::numeric_limits<int>::min();
  int maxy = std::numeric_limits<int>::min();

  for (auto d : distances) {
    if (minx > d.first.first)
      minx = d.first.first;
    if (maxx < d.first.first)
      maxx = d.first.first;
    if (miny > d.first.second)
      miny = d.first.second;
    if (maxy < d.first.second)
      maxy = d.first.second;
  }

  int w = maxx - minx;
  int h = maxy - miny;

  std::vector<char> map;
  int size = (w + 1) * (h + 1);
  map.resize(size, '#');

  for (auto p : path) {
    std::pair<int, int> pos = p.first;
    std::pair<int, int> prev = p.second;

    std::pair<int, int> delta;
    delta.first = pos.first - prev.first;
    delta.second = pos.second - prev.second;

    char c;
    if (delta.first == 1)
      c = '<';
    if (delta.first == -1)
      c = '>';
    if (delta.second == 1)
      c = '^';
    if (delta.second == -1)
      c = 'v';

    int x = pos.first - minx;
    int y = pos.second - miny;

    int index = x + y * w;
    map[index] = c;
  }

  map[-minx + (-miny * w)] = 'X';

  for (int y = 0; y < h; ++y) {
    for (int x = 0; x < w; ++x) {
      int index = x + y * w;
      std::cout << map[index];
    }
    std::cout << std::endl;
  }

  std::cout << w << "\t" << h << std::endl;
}

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

  std::map<std::pair<int, int>, std::pair<int, int>> path;

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
        if (distances[pos] > (distances[prev] + 1)) {
          path[pos] = prev;
        }
        distances[pos] = std::min(distances[pos], distances[prev] + 1);
      } else {
        distances[pos] = distances[prev] + 1;
        path[pos] = prev;
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

  // print(distances, path);

  logger::get(logtype::logINFO) << "Part 1: " << max << std::endl;
  logger::get(logtype::logINFO) << "Part 2: " << count1000plus << std::endl;

  return 0;
}