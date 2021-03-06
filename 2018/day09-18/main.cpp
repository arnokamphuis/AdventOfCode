#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

class marblecircle {
  std::list<uint64_t>::iterator current;
  std::list<uint64_t> marbles;

public:
  void print() {
    auto c = marbles.begin();
    while (c != marbles.end()) {
      std::cout << (c == current ? "(" : "") << *c << (c == current ? ")" : "")
                << " ";
      ++c;
    }
    std::cout << "\n";
  }

  uint64_t addmarble(uint64_t m) {
    uint64_t score = 0;
    if ((m == 0) || (m == 1)) {
      marbles.push_back(m);
      current = marbles.end();
      current--;
    } else if (m % 23 == 0) {
      for (int i = 0; i < 7; ++i) {
        if (current == marbles.begin())
          current = marbles.end();
        --current;
      }
      score += m + (*current);
      current = marbles.erase(current);
    } else {
      for (int i = 0; i < 2; ++i) {
        if (current == marbles.end())
          current = marbles.begin();
        current++;
      }
      current = marbles.insert(current, m);
    }

    if (marbles.size() == marbles.max_size())
      std::cout << "EEERORROR" << std::endl;
    return score;
  }
};

class game {
  uint64_t numberofmarbles;
  std::vector<uint64_t> playerscore;
  marblecircle circle;

  void print() {
    std::cout << "Scores: ";
    for (auto ps : playerscore)
      std::cout << ps << ",";
    std::cout << "\n";
  }

public:
  game(uint64_t players, uint64_t maxmarble) : numberofmarbles(maxmarble) {
    for (uint64_t i = 0; i < players; ++i)
      playerscore.push_back(0);
  }

  uint64_t run() {
    uint64_t player = 0;
    circle.addmarble(0);
    for (uint64_t turn = 1; turn <= numberofmarbles; ++turn) {
      playerscore[player] += circle.addmarble(turn);
      player = (player + 1) % playerscore.size();
    }

    uint64_t maxscore = 0;
    for (auto ps : playerscore)
      if (ps > maxscore)
        maxscore = ps;
    return maxscore;
  }
};

int main() {
  std::string line;
  uint64_t players = 0;
  uint64_t marbles = 0;
  getline(std::cin, line);
  auto firstspace = line.find(' ');
  auto worth = line.find("worth");
  auto points = line.find("points");
  players = std::atoi(line.substr(0, firstspace).c_str());
  marbles = std::atoi(line.substr(worth + 6, points - worth - 6).c_str());

  game g1(players, marbles);

  auto start1 = std::chrono::high_resolution_clock::now();
  int64_t r1 = g1.run();
  auto end1 = std::chrono::high_resolution_clock::now();
  logger::get(logtype::logINFO)
      << "Part 1: " << r1 << " in "
      << std::chrono::duration_cast<std::chrono::milliseconds>(end1 - start1)
             .count()
      << " ms" << '\n';

  game g2(players, 100 * marbles);
  auto start2 = std::chrono::high_resolution_clock::now();
  int64_t r2 = g2.run();
  auto end2 = std::chrono::high_resolution_clock::now();
  logger::get(logtype::logINFO)
      << "Part 2: " << r2 << " in "
      << std::chrono::duration_cast<std::chrono::milliseconds>(end2 - start2)
             .count()
      << " ms" << '\n';
  return 0;
}