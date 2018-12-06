#include "logger.h"
#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

int capoff = 10000;

void printmap(const std::vector<std::vector<int>> &map, bool translate) {
  for (auto x : map) {
    for (auto y : x)
      if (y == -1)
        std::cout << ".";
      else {
        if (translate) {
          if (y > 25)
            std::cout << (char)(y + 97 - 26);
          else
            std::cout << (char)(y + 65);
        } else
          std::cout << y;
      }
    std::cout << '\n';
  }
}

int find_area(int c, const std::vector<std::vector<int>> &map) {
  int count = 0;
  for (auto x : map) {
    for (auto y : x) {
      if (y == c)
        ++count;
    }
  }
  return count;
}

int find_total_distance(int i, int j,
                        const std::vector<std::pair<int, int>> &positions) {
  int c = 0;
  for (auto p : positions)
    c += std::abs(p.first - i) + std::abs(p.second - j);
  if (c >= capoff)
    c = -1;
  return c;
}

int find_closest(int i, int j,
                 const std::vector<std::pair<int, int>> &positions) {
  int d = 1000000;
  int idx = -1;
  int c = 0;
  for (auto p : positions) {
    int cd = std::abs(p.first - i) + std::abs(p.second - j);
    if (cd < d) {
      idx = c;
      d = cd;
    }
    ++c;
  }

  int duplicates = 0;
  for (auto p : positions) {
    int cd = std::abs(p.first - i) + std::abs(p.second - j);
    if (cd == d)
      ++duplicates;
  }
  if (duplicates > 1)
    idx = -1;
  return idx;
}

int main() {
  std::vector<std::string> input;
  std::vector<std::pair<int, int>> positions;
  int min_x = 10000;
  int min_y = 10000;
  int max_x = -1;
  int max_y = -1;

  std::vector<std::vector<int>> map;
  std::vector<std::vector<int>> distances;

  std::string line;
  while (getline(std::cin, line)) {
    input.push_back(line);
    auto npos = line.find(',');
    int x = std::atoi(line.substr(0, npos).c_str());
    int y = std::atoi(line.substr(npos + 1, line.length()).c_str());
    positions.push_back(std::make_pair(x, y));
    if (x < min_x)
      min_x = x;
    if (y < min_y)
      min_y = y;
    if (x > max_x)
      max_x = x;
    if (y > max_y)
      max_y = y;
  }

  int realsize_x = max_x + min_x;
  int realsize_y = max_y + min_y;

  std::set<int> candidates;
  for (int i = 0; i < positions.size(); ++i)
    candidates.insert(i);

  for (int i = 0; i < realsize_x; ++i) {
    std::vector<int> row;
    std::vector<int> row2;
    for (int j = 0; j < realsize_y; ++j) {
      int c = find_closest(i, j, positions);
      int d = find_total_distance(i, j, positions);
      row.push_back(c);
      row2.push_back(d);
    }
    map.push_back(row);
    distances.push_back(row2);
  }

  // printmap(distances, false);

  std::map<int, int> borders;
  for (int i = 0; i < realsize_x; ++i) {
    ++borders[map[i][0]];
    ++borders[map[i][realsize_y - 1]];
  }
  for (int i = 0; i < realsize_x; ++i) {
    ++borders[map[0][i]];
    ++borders[map[realsize_x - 1][i]];
  }
  --borders[map[0][0]];
  --borders[map[realsize_x - 1][0]];
  --borders[map[0][realsize_y - 1]];
  --borders[map[realsize_x - 1][realsize_y - 1]];

  borders.erase(-1);
  for (auto b : borders) {
    if (b.second > 1) {
      candidates.erase(b.first);
    }
  }

  int max_area = 0;
  int max_c = -1;
  for (auto c : candidates) {
    int area = find_area(c, map);
    if (area > max_area) {
      max_area = area;
      max_c = c;
    }
  }
  logger::get(logtype::logINFO) << "Part 1: " << max_area << '\n';

  int area = 0;
  for (auto x : distances) {
    for (auto y : x) {
      if (y > 0)
        ++area;
    }
  }
  logger::get(logtype::logINFO) << "Part 2: " << area << '\n';
  return 0;
}