#include "logger.h"
#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

class node {
  std::vector<node *> children;
  std::vector<int> payloads;
  int value;

public:
  node() : value(-1) {}

  void parse_from_cin() {
    int n_children = 0;
    int n_payloads = 0;
    std::cin >> n_children >> n_payloads;

    for (int c = 0; c < n_children; ++c) {
      node *newnode = new node;
      newnode->parse_from_cin();
      children.push_back(newnode);
    }

    for (int p = 0; p < n_payloads; ++p) {
      int payload = 0;
      std::cin >> payload;
      payloads.push_back(payload);
    }
  }

  int addpayloads() {
    int total = 0;
    for (auto p : payloads)
      total += p;
    for (auto c : children)
      total += c->addpayloads();
    return total;
  }

  int calculate_value() {
    if (value > -1)
      return value;

    value = 0;

    if (children.size() > 0) {

      for (auto p : payloads) {
        if ((p - 1) < children.size()) {
          value += children[p - 1]->calculate_value();
        }
      }

    } else {
      for (auto p : payloads)
        value += p;
    }

    return value;
  }
};

int main() {
  node *root = new node();
  root->parse_from_cin();
  logger::get(logtype::logINFO) << "Part 1: " << root->addpayloads() << '\n';
  logger::get(logtype::logINFO)
      << "Part 2: " << root->calculate_value() << '\n';
  return 0;
}