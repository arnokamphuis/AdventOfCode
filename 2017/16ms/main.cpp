#include "logger.h"
#include <algorithm>
#include <iostream>
#include <vector>

void do_move(std::string &formation, std::string cmd) {
  switch (cmd[0]) {
  case 's': {
    for (int i = 0; i < std::stoi(cmd.substr(1)); ++i) {
      char c = formation.back();
      formation.pop_back();
      formation.insert(formation.begin(), c);
    }
  } break;

  case 'x': {
    int from, to;
    std::string positions = cmd.substr(1);
    std::size_t slash = positions.find('/');
    from = std::stoi(positions.substr(0, slash));
    to = std::stoi(positions.substr(slash + 1));
    char tmp = formation[from];
    formation[from] = formation[to];
    formation[to] = tmp;
  } break;

  case 'p': {
    int from, to;
    std::string positions = cmd.substr(1);
    std::size_t slash = positions.find('/');
    from = formation.find(positions.substr(0, slash));
    to = formation.find(positions.substr(slash + 1));
    char tmp = formation[from];
    formation[from] = formation[to];
    formation[to] = tmp;
  } break;

  default: {
    logger::get(logtype::logERROR) << "This should not happen!" << '\n';
  } break;
  }
}

void reset(std::string &formation) {
  for (int i = 0; i < formation.size(); ++i)
    formation.at(i) = 'a' + i;
}

int main() {
  int size = 16;

  std::string formation;
  formation.resize(size, 'a');
  reset(formation);

  std::vector<std::string> thedance;
  std::vector<std::string> previous_formations;

  std::string cmd;
  while (getline(std::cin, cmd, ','))
    thedance.push_back(cmd);

  int cycle_length = -1;
  int cycle_start = -1;

  std::vector<std::string>::iterator p;
  int maxruns = 1000;

  previous_formations.push_back(formation);
  for (int i = 0; i < maxruns; ++i) {
    for (auto dm : thedance) {
      do_move(formation, dm);
    }
    if (i == 0)
      std::cout << "Part 1: " << formation << std::endl;

    if ((p = std::find(previous_formations.begin(), previous_formations.end(),
                       formation)) == previous_formations.end()) {
      previous_formations.push_back(formation);
    } else { // cycle detected
      cycle_start = p - previous_formations.begin();
      cycle_length = previous_formations.size() - cycle_start;
      break;
    }
  }

  if (cycle_length > 0) {
    int runstogo = (maxruns - cycle_start) % cycle_length;
    for (int i = 0; i < runstogo; ++i)
      for (auto dm : thedance)
        do_move(formation, dm);
  }

  std::cout << "Part 2: " << formation << std::endl;
}