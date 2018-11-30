#include <algorithm>
#include <climits>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

typedef struct {
  std::string name;
  int size;
  int orig_size;
  std::vector<std::string> children;
} node;

int main() {

  std::vector<node> childless;
  std::vector<node> parents;

  for (std::string line; std::getline(std::cin, line);) {
    std::size_t found = line.find(" -> ");
    if (found != std::string::npos) {
      node n;
      std::string nodename = line.substr(0, found);

      std::size_t f = nodename.find(" ");
      std::size_t ho = nodename.find("(");
      std::size_t hs = nodename.find(")");
      n.name = nodename.substr(0, f);
      n.size = std::stoi(nodename.substr(ho + 1, hs - ho));
      n.orig_size = n.size;

      std::string childs = line.substr(found + 4);

      std::string chld;
      std::size_t cp = childs.find(",");
      while (cp != std::string::npos) {
        chld = childs.substr(0, cp);
        n.children.push_back(chld);
        childs = childs.substr(cp + 2);
        cp = childs.find(",");
      }
      n.children.push_back(childs);

      std::stringstream ss(childs);
      std::string token;
      parents.push_back(n);
    } else { // not found ==> childless
      node n;
      std::size_t f = line.find(" ");
      std::size_t ho = line.find("(");
      std::size_t hs = line.find(")");
      n.name = line.substr(0, f);
      n.size = std::stoi(line.substr(ho + 1, hs - ho));
      n.orig_size = n.size;
      childless.push_back(n);
    }
  }

  while (parents.size() > 0) {

    // checking the weights
    for (auto &p : parents) {
      std::vector<std::string>::iterator n_iter = p.children.begin();
      std::map<int, int> sizes;
      int minsize = INT_MAX;
      int maxsize = INT_MIN;
      while (n_iter != p.children.end()) {
        for (auto c : childless) {
          if (c.name.compare(*n_iter) == 0) {
            if (c.size > maxsize)
              maxsize = c.size;
            if (c.size < minsize)
              minsize = c.size;
            sizes[c.size]++;
          }
        }
        ++n_iter;
      }
      int wrong_size, wrong_orig_size;
      bool correct = true;
      if (sizes.size() > 1) {
        for (auto s : sizes) {
          if (s.second == 1) {
            n_iter = p.children.begin();
            while (n_iter != p.children.end()) {
              for (auto c : childless) {
                if (c.name.compare(*n_iter) == 0) {
                  if (c.size == s.first) {
                    wrong_size = c.size;
                    wrong_orig_size = c.orig_size;
                    correct = false;
                    std::cout << "Found node with error: " << c.name
                              << " with weight " << c.size << std::endl;
                  }
                }
              }
              ++n_iter;
            }
          }
        }
      }
      if (!correct) {
        for (auto s : sizes) {
          if (s.first != wrong_size) {
            int delta = s.first - wrong_size;
            std::cout << "Needs to be adjusted by " << delta
                      << " - should be: " << wrong_orig_size + delta
                      << std::endl;
            return 0;
          }
        }
      }
    }

    // reforming/collapsing the tree
    for (auto &p : parents) {
      // std::cout << "  --> checking parent: " << p.name << std::endl;
      for (auto &c : childless) {
        // std::cout << "checking child: " << c.name << std::endl;

        std::vector<std::string>::iterator n_iter = p.children.begin();
        while (n_iter != p.children.end()) {
          if (n_iter->compare(c.name) == 0) {
            // std::cout << "removing" << std::endl;
            p.size += c.size;
            c.size = 0;
            n_iter = p.children.erase(n_iter);
          } else
            ++n_iter;
        }
        // std::cout << "parent " << p.name << " has " << p.children.size() << "
        // children" << std::endl;
      }
    }

    std::vector<node>::iterator c_iter = childless.begin();
    while (c_iter != childless.end()) {
      if (c_iter->size == 0) {
        c_iter = childless.erase(c_iter);
      } else {
        ++c_iter;
      }
    }
    std::vector<node>::iterator p_iter = parents.begin();
    while (p_iter != parents.end()) {
      // std::cout << p_iter->name << " has size " << p_iter->children.size() <<
      // std::endl;
      if (p_iter->children.size() == 0) {
        // std::cout << "Moving " << p_iter->name << " to childless." <<
        // std::endl;
        childless.push_back(*p_iter);
        p_iter = parents.erase(p_iter);
      } else {
        ++p_iter;
      }
    }
  }

  // std::cout << childless.size() << std::endl;
  // std::cout << parents.size() << std::endl;
  std::cout << "Name of root: " << childless[0].name << " with size "
            << childless[0].size << std::endl;
}