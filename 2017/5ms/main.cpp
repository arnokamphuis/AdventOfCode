#include <iostream>
#include <vector>

int toint(const char &c) { return c - '0'; }

int execute(int *arr, int c, int size, int steps) {
  while (!((c < 0) || (c >= size))) {
    // std::cout << "c: " << c << " - size: " << size << " - steps: " << steps
    // << std::endl;
    int move = arr[c];
    if (move >= 3)
      arr[c]--;
    else
      arr[c]++;
    steps++;
    c += move;
  }
  return steps;
}

int main() {
  int i;
  std::vector<int> list;
  while (std::cin >> i) {
    list.push_back(i);
  }

  int *arr = new int[list.size()];
  int *orig_arr = arr;
  for (auto l : list) {
    *arr = l;
    arr++;
  }

  int steps = execute(orig_arr, 0, list.size(), 0);

  delete[] orig_arr;

  std::cout << "Answer: " << steps << std::endl;
}