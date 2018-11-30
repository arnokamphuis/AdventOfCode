#include <cmath>
#include <iostream>
#include <map>
#include <string>
#include <vector>

int ops[10];
bool complex = true;

void find_and_replace_all(std::string &data, std::string toSearch,
                          std::string replaceStr) {
  // Get the first occurrence
  size_t pos = data.find(toSearch);

  // Repeat till end is reached
  while (pos != std::string::npos) {
    // Replace this occurrence of Sub String
    data.replace(pos, toSearch.size(), replaceStr);
    // Get the next occurrence from the current position
    pos = data.find(toSearch, pos + toSearch.size());
  }
}

class tilestorage {
private:
public:
  int size;
  char storage[4][4];

  tilestorage() { size = -1; }

  explicit tilestorage(const std::string &s) {
    std::string init_s = s;
    find_and_replace_all(init_s, "/", "");
    int ssize = init_s.size();
    size = std::sqrt(ssize);
    int i = 0;
    int x = 0;
    int y = 0;
    for (auto c : init_s) {
      x = i % size;
      y = i / size;
      storage[x][y] = c;
      ++i;
    }
  }

  tilestorage rotate_left() {
    ops[1]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[j][size - 1 - i] = storage[i][j];
    return ts;
  }

  tilestorage rotate_right() {
    ops[2]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[size - 1 - j][i] = storage[i][j];
    return ts;
  }

  tilestorage rotate_full() {
    ops[3]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[size - 1 - j][size - 1 - i] = storage[i][j];
    return ts;
  }

  tilestorage flip() {
    ops[4]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[i][size - 1 - j] = storage[i][j];
    return ts;
  }

  tilestorage flip_vertical() {
    ops[5]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[size - 1 - i][j] = storage[i][j];
    return ts;
  }

  tilestorage flip_rotate_left() {
    ops[6]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[j][i] = storage[i][j];
    return ts;
  }

  tilestorage flip_rotate_right() {
    ops[7]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[size - 1 - j][size - 1 - i] = storage[i][j];
    return ts;
  }

  tilestorage flip_rotate_full() {
    ops[8]++;
    tilestorage ts;
    ts.size = size;
    for (int i = 0; i < size; ++i)
      for (int j = 0; j < size; ++j)
        ts.storage[size - 1 - i][j] = storage[i][j];
    return ts;
  }

  std::string get_string() {
    std::string res = "";
    for (int i = 0; i < size; ++i) {
      for (int j = 0; j < size; ++j)
        res += storage[i][j];
      if (i != (size - 1))
        res += '/';
    }
    return res;
  }
};

class tile {
  int size;
  std::string leds;

  bool check(const std::string &desc) {
    tilestorage ts(desc);
    if (leds.compare(ts.get_string()) == 0)
      return true;

    if (complex) {
      if (leds.compare(ts.rotate_left().get_string()) == 0)
        return true;
      if (leds.compare(ts.rotate_right().get_string()) == 0)
        return true;
      if (leds.compare(ts.rotate_full().get_string()) == 0)
        return true;
      if (leds.compare(ts.flip().get_string()) == 0)
        return true;
      if (leds.compare(ts.flip_rotate_left().get_string()) == 0)
        return true;
      if (leds.compare(ts.flip_rotate_right().get_string()) == 0)
        return true;
      if (leds.compare(ts.flip_rotate_full().get_string()) == 0)
        return true;
      if (leds.compare(ts.flip_vertical().get_string()) == 0)
        return true;
    }
    return false;
  }

  void change_into(const std::string &desc) {
    ++size;
    update_leds(desc);
  }

  void update_leds(const std::string &desc) { leds = desc; }

public:
  tile(int s, const std::string &desc) : size(s) { update_leds(desc); }

  int get_size() { return size; }

  void evolve(const std::map<std::string, std::string> &rules) {
    bool changed = false;
    for (auto r : rules) {
      std::string from = r.first;
      find_and_replace_all(from, "/", "");
      int s = std::sqrt(from.size());
      if ((s == size) && (check(r.first))) {
        change_into(r.second);
        changed = true;
        break;
      }
    }
    if (!changed)
      std::cout << "EEEEEEEEEEEEEEEEEEEEEERRRRRRRRRRRRRRRRRROOOOOOOOOOOOOOOOOOO"
                   "OOORRRRRRRRRRRRRRRRRRRRRRRR"
                << std::endl;
  }

  char led(int i, int j) {
    std::string tmp = leds;
    find_and_replace_all(tmp, "/", "");
    return tmp[i + size * j];
  }
};

class grid {
  int size;
  std::vector<std::vector<char>> leds;
  std::vector<std::vector<tile *>> tiles;
  std::map<std::string, std::string> rules;

public:
  grid() {
    size = 3;
    tile *first = new tile(size, ".#./..#/###");

    for (int i = 0; i < size; ++i) {
      std::vector<char> line;
      for (int j = 0; j < size; ++j) {
        line.push_back(first->led(j, i));
      }
      leds.push_back(line);
    }
  }

  int get_size() { return size; }

  void add_rule(const std::string &from, const std::string &to) {
    rules[from] = to;
  }

  void delete_tiles() {
    for (auto &tl : tiles) {
      for (auto t : tl)
        delete t;
      tl.clear();
    }
    tiles.clear();
  }

  void create_tiles() {
    int tile_size = ((size % 2 == 0) ? 2 : 3);
    int tilecount = size / tile_size;
    for (int tc_x = 0; tc_x < tilecount; ++tc_x) {
      std::vector<tile *> tile_line;
      for (int tc_y = 0; tc_y < tilecount; ++tc_y) {
        std::string tilestring;
        for (int x = 0; x < tile_size; ++x) {
          int x_index = tile_size * tc_x + x;
          for (int y = 0; y < tile_size; ++y) {
            int y_index = tile_size * tc_y + y;
            tilestring += leds[y_index][x_index];
          }
          tilestring += '/';
        }
        tilestring.erase(tilestring.end() - 1);

        tile *t = new tile(tile_size, tilestring);
        tile_line.push_back(t);
      }
      tiles.push_back(tile_line);
    }
  }

  void update_tiles() {
    for (auto tl : tiles)
      for (auto t : tl)
        t->evolve(rules);
  }

  void grid_from_tiles() {
    int ts = tiles[0][0]->get_size();
    size = ts * tiles.size();

    leds.clear();
    for (int x = 0; x < size; ++x) {
      std::vector<char> ledline;
      for (int y = 0; y < size; ++y) {
        int tx = x / ts;
        int ty = y / ts;

        int ix = x % ts;
        int iy = y % ts;

        ledline.push_back(tiles[ty][tx]->led(iy, ix));
      }
      leds.push_back(ledline);
    }
  }

  void update() {
    create_tiles();
    update_tiles();
    grid_from_tiles();
    delete_tiles();
  }

  void print() {
    for (auto lc : leds) {
      for (auto c : lc)
        std::cout << c;
      std::cout << std::endl;
    }
  }

  int get_turned_on() {
    int count = 0;
    int oc = 0;
    for (auto lc : leds)
      for (auto c : lc)
        if (c == '#')
          ++count;
        else
          ++oc;

    if ((count + oc) != size * size) {
      std::cout << "ERROR: Number of leds not correct" << std::endl;
    }
    return count;
  }
};

int main() {
  for (int i = 0; i < 9; ++i)
    ops[i] = 0;

  grid *g = new grid();

  std::string line;
  while (getline(std::cin, line)) {
    std::size_t p = line.find(" => ");
    g->add_rule(line.substr(0, p), line.substr(p + 4));
  }

  int run1 = 5;
  int run2 = 18;
  for (int i = 0; i < run1; ++i) {
    g->update();
  }

  std::cout << "Part 1: " << g->get_turned_on() << std::endl;

  for (int i = 0; i < (run2 - run1); ++i) {
    g->update();
  }

  std::cout << "Part 2: " << g->get_turned_on() << std::endl;

  delete g;

  return 0;
}