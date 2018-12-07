#include "logger.h"
#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

int number_of_workers = 5;
int duraction_delta = 60;

class task {
public:
  char id;
  bool done;
  int starttime;
  int duration;

  task(char n) : id(n), done(false) {
    duration = (int)(n - 65) + duraction_delta;
    starttime = -1;
  }

  std::vector<task *> next;
  std::vector<task *> prev;

  bool ready() {
    bool r = true;
    for (auto p : prev) {
      if (!p->done)
        r = false;
    }
    return r;
  }

  void start(int t) { starttime = t; }
  bool update(int t) {
    done = (t >= (starttime + duration));
    return done;
  }
};

void print_candidates(const std::map<char, task *> &candidates) {
  for (auto c : candidates) {
    std::cout << c.first << ">";
  }
  std::cout << "\n";
}

void initialize(int step, std::vector<std::pair<char, bool>> &workers,
                std::map<char, task *> &tasks,
                std::map<char, task *> &candidates) {

  task *ct = nullptr;
  for (auto &w : workers) {
    if (!w.second) {
      ct = nullptr;
      for (auto pct : candidates) {
        if (pct.second->ready()) {
          ct = pct.second;
          break;
        }
      }
      if (ct != nullptr) {
        w.first = ct->id;
        w.second = true;
        ct->start(step);
        candidates.erase(ct->id);
      }
    }
  }
}

void do_step(int step, std::vector<std::pair<char, bool>> &workers,
             std::map<char, task *> &tasks, std::map<char, task *> &candidates,
             std::string &order) {
  for (auto &w : workers) {
    if (w.second) {
      task *t = tasks[w.first];
      if (t->update(step)) {
        order += w.first;
        w.second = false;
        w.first = ' ';

        for (auto n : t->next) {
          if (!(n->done)) {
            candidates[n->id] = n;
          }
        }
      }
    }
  }

  initialize(step + 1, workers, tasks, candidates);
}

bool all_done(std::map<char, task *> tasks) {
  bool done = true;
  for (auto t : tasks)
    if (!(t.second->done))
      done = false;
  return done;
}

void process_steps(std::map<char, task *> &candidates, std::string &order) {
  task *ct = nullptr;
  for (auto pct : candidates) {
    if (pct.second->ready()) {
      ct = pct.second;
      break;
    }
  }
  ct->done = true;
  order += ct->id;
  candidates.erase(ct->id);

  if (ct->next.size() == 0)
    return;

  for (auto nc : ct->next) {
    if (!(nc->done)) {
      candidates[nc->id] = nc;
    }
  }

  if (candidates.size() == 0)
    return;

  process_steps(candidates, order);
}

int main() {
  std::map<char, task *> tasks;
  std::map<char, task *> candidates;

  std::string line;
  while (getline(std::cin, line)) {
    char current = line[5];
    char next = line[36];
    task *current_task = nullptr;
    task *next_task = nullptr;

    if (tasks.find(current) != tasks.end())
      current_task = tasks[current];
    else {
      current_task = new task(current);
      tasks[current] = current_task;
    }

    if (tasks.find(next) != tasks.end())
      next_task = tasks[next];
    else {
      next_task = new task(next);
      tasks[next] = next_task;
    }

    current_task->next.push_back(next_task);
    next_task->prev.push_back(current_task);
  }

  for (auto t : tasks) {
    if (t.second->prev.size() == 0)
      candidates[t.first] = t.second;
  }

  bool part1 = false;
  if (part1) {
    std::string order = "";
    process_steps(candidates, order);

    logger::get(logtype::logINFO) << "Part 1: " << order << '\n';
  } else {
    int step = 0;
    std::vector<std::pair<char, bool>> workers;

    for (int i = 0; i < number_of_workers; ++i)
      workers.push_back(std::make_pair(' ', false));

    std::string order = "";
    initialize(step, workers, tasks, candidates);
    while (!all_done(tasks)) {
      do_step(step, workers, tasks, candidates, order);
      ++step;
    }

    logger::get(logtype::logINFO)
        << "Part 2: " << step << "\t" << order << '\n';
  }

  return 0;
}