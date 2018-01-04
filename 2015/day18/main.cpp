#include <iostream>
#include <string>
#include <vector>

bool error_lights = true;

void do_error_lights(std::vector<std::vector<bool>> &lights) {
  if (error_lights) {
    int size = lights.size();
    lights[0][0] = true;
    lights[size - 1][0] = true;
    lights[0][size - 1] = true;
    lights[size - 1][size - 1] = true;
  }
}

void update(std::vector<std::vector<bool>> &lights,
            const std::vector<std::vector<bool>> &prev_lights) {
  int size = lights.size();

  for (int x = 0; x < size; ++x) {
    for (int y = 0; y < size; ++y) {

      int nl_on = 0;
      for (int tx = -1; tx < 2; ++tx) {
        for (int ty = -1; ty < 2; ++ty) {
          bool leftcorrect = (0 <= x + tx);
          bool rightcorrect = ((x + tx) < size);
          bool topcorrect = (0 <= (y + ty));
          bool bottomcorrect = ((y + ty) < size);
          bool notcenter = !((tx == 0) && (ty == 0));

          if (notcenter && leftcorrect && rightcorrect && topcorrect &&
              bottomcorrect) {
            if (prev_lights[x + tx][y + ty])
              ++nl_on;
          }
        }
      }

      bool prev_lightsXY = prev_lights[x][y];

      if (prev_lightsXY && !(nl_on == 2 || nl_on == 3))
        lights[x][y] = false;
      if (!prev_lightsXY && nl_on == 3)
        lights[x][y] = true;
    }
  }
}

void print(const std::vector<std::vector<bool>> &lights) {
  for (auto ll : lights) {
    for (auto l : ll)
      if (l)
        std::cout << "#";
      else
        std::cout << ".";
    std::cout << std::endl;
  }
}

int main() {
  std::vector<std::vector<bool>> lights, prev_lights;
  std::string line;
  int x, y;
  x = 0;
  y = 0;
  while (getline(std::cin, line)) {
    std::vector<bool> lightline;
    for (x = 0; x < line.size(); ++x)
      lightline.push_back((line[x] == '#'));
    ++y;
    lights.push_back(lightline);
  }

  int size = lights.size();
  if (error_lights) {
    lights[0][0] = true;
    lights[size - 1][0] = true;
    lights[0][size - 1] = true;
    lights[size - 1][size - 1] = true;
  }

  prev_lights = lights;

  // print(lights);
  // std::cout <<"=======================================" << std::endl;

  for (int step = 0; step < 100; ++step) {
    update(lights, prev_lights);
    do_error_lights(lights);
    prev_lights = lights;
    // print(lights);
    // std::cout <<"=======================================" << std::endl;
  }

  int c = 0;
  for (auto ll : lights)
    for (auto l : ll)
      if (l)
        ++c;

  std::cout << "Part 1: " << c << std::endl;
}