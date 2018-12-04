#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

//#include "basetimer.h"
#include "logger.h"

class guard {
  int64_t sleeping[60];
  int64_t lasttimefellasleep;

public:
  int64_t id;
  guard() {
    for (int i = 0; i < 60; ++i)
      sleeping[i] = 0;
    lasttimefellasleep = -1;
  }

  void fallasleep(int64_t t) { lasttimefellasleep = t; }
  void wakeup(int64_t t) {
    for (int i = lasttimefellasleep; i < t; ++i)
      ++sleeping[i];
  }

  int64_t sleepingminutes() {
    int64_t total = 0;
    for (auto s : sleeping)
      total += s;
    return total;
  }

  int maxsleepingminute() {
    int64_t max = -1;
    int index = -1;
    for (int i = 0; i < 60; ++i) {
      if (sleeping[i] > max) {
        max = sleeping[i];
        index = i;
      }
    }
    return index;
  }

  int64_t sleeptime(int64_t t) { return sleeping[t]; }
};

int main() {
  std::vector<std::string> input;
  std::map<int64_t, guard> guards;

  // Part 1
  std::string line;
  while (getline(std::cin, line)) {
    input.push_back(line);
  }

  std::map<std::string, std::string> actions;
  for (auto in : input) {
    auto firstbrace = in.find('[');
    auto lastbrace = in.find(']');

    std::string datetime = in.substr(firstbrace + 1, lastbrace - 1);

    std::string action = in.substr(lastbrace + 2, in.length());

    actions[datetime] = action;
  }

  int lastguardid = -1;
  for (auto a : actions) {
    if (a.second[0] == 'G') {
      // guard starts shift
      auto hastag = a.second.find('#');
      auto guardid = a.second.substr(hastag + 1, a.second.length());
      auto space = guardid.find(' ');
      guardid = guardid.substr(0, space);
      lastguardid = std::atoi(guardid.c_str());
      guards[lastguardid].id = lastguardid;
    } else {
      auto secpos = a.first.find(':');
      auto t = std::atoi(a.first.substr(secpos + 1, a.first.length()).c_str());

      if (a.second[0] == 'f')
        guards[lastguardid].fallasleep(t);
      else if (a.second[0] == 'w')
        guards[lastguardid].wakeup(t);
    }
  }

  int64_t maxsleep = -1;
  int64_t gid = -1;
  for (auto g : guards) {
    int64_t sleeptime = g.second.sleepingminutes();
    if (sleeptime > maxsleep) {
      maxsleep = sleeptime;
      gid = g.first;
    }
  }
  logger::get(logtype::logINFO)
      << "Part 1: " << guards[gid].maxsleepingminute() * gid << "\n";

  maxsleep = -1;
  gid = -1;
  for (auto g : guards) {
    int64_t maxminute = g.second.maxsleepingminute();
    int64_t sleeptime = g.second.sleeptime(maxminute);
    if (sleeptime > maxsleep) {
      maxsleep = sleeptime;
      gid = g.first;
    }
  }
  logger::get(logtype::logINFO)
      << "Part 2: " << guards[gid].maxsleepingminute() * gid << "\n";

  return 0;
}