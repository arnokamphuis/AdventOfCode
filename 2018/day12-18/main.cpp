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

class life {
  int nul;
  std::string current;

  std::map<std::string, char> rules;

public:
  life(std::string init) : current(init) { nul = 0; }

  void addrule(std::string in, char res) { rules[in] = res; }

  void update() {
    std::string temp = current;

    nul += 4;
    temp = "...." + temp + "....";

    std::string next = temp;

    for (int i = 2; i < temp.size() - 2; ++i) {
      std::string index = temp.substr(i - 2, 5);
      if (rules.find(index) != rules.end()) {
        char n = rules[index];
        next[i] = (n == '#' ? '#' : '.');
      } else {
        next[i] = '.';
      }
    }

    while (next[0] == '.') {
      next.erase(next.begin());
      --nul;
    }
    while (next[next.size() - 1] == '.') {
      next.erase(next.end() - 1);
    }
    current = next;
  }

  int score() {
    int res = 0;
    for (int i = 0; i < current.size(); ++i) {
      res += (current[i] == '#' ? i - nul : 0);
    }
    return res;
  }
};

int main() {
  std::string line;
  getline(std::cin, line);
  life l(line.substr(15, line.length()));

  while (getline(std::cin, line)) {
    size_t arrowpos = line.find('=');
    if (arrowpos != std::string::npos) {
      std::string from = line.substr(0, arrowpos - 1);
      std::string to = line.substr(arrowpos + 3, line.length());
      l.addrule(from, to[0]);
    }
  }

  int score = 0;
  int prevscore = 0;
  std::queue<int> scores;
  for (int t = 0; t < 3000; ++t) {
    l.update();
    score = l.score();
    scores.push(score - prevscore);
    if (t == 19)
      logger::get(logtype::logINFO) << "Part 1: " << score << "\n";
    if (scores.size() > 100)
      scores.pop();

    prevscore = score;
  }

  int total = 0;
  int number = scores.size();
  while (!scores.empty()) {
    total += scores.front();
    scores.pop();
  }
  int delta = total / number;

  int64_t result = (int64_t)delta * ((int64_t)50000000000 - 3000) + score;
  logger::get(logtype::logINFO) << "Part 2: " << result << "\n";

  return 0;
}