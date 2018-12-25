#include "logger.h"
#include <algorithm>
#include <chrono>
#include <fstream>
#include <iostream>
#include <iterator>
#include <list>
#include <map>
#include <math.h>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

struct Nanobot {
  int64_t x;
  int64_t y;
  int64_t z;
  int64_t r;
};

bool covers(const Nanobot &nanobot, const int64_t &x, const int64_t &y,
            const int64_t &z, const int64_t &padding) {
  return (std::abs(x - nanobot.x) + std::abs(y - nanobot.y) +
              std::abs(z - nanobot.z) <=
          nanobot.r + padding);
}

struct Point {
  int64_t x;
  int64_t y;
  int64_t z;
  Point(const int64_t &xi, const int64_t &yi, const int64_t &zi)
      : x(xi), y(yi), z(zi) {}
  bool operator<(const Point &p) const {
    return x < p.x ? true
                   : (x > p.x ? false
                              : (y < p.y ? true : (y > p.y ? false : z < p.z)));
  }

  bool operator==(const Point &p) const {
    return x == p.x && y == p.y && z == p.z;
  }
};

bool covers(const Nanobot &nanobot, const Point &p, const int64_t &padding) {
  return covers(nanobot, p.x, p.y, p.z, padding);
}

int main() {
  std::vector<Nanobot> nanobots;

  std::string line;
  while (getline(std::cin, line)) {
    Nanobot nb;

    auto ls = line.find('<');
    auto rs = line.find('>');
    std::string posstr = line.substr(ls + 1, rs - ls - 1);

    auto xp = posstr.find(',');
    auto yp = posstr.rfind(',');

    nb.x = std::atoi(posstr.substr(0, xp).c_str());
    nb.y = std::atoi(posstr.substr(xp + 1, yp - xp - 1).c_str());
    nb.z = std::atoi(posstr.substr(yp + 1, posstr.length()).c_str());

    auto ra = line.find(", r=");
    std::string radstr = line.substr(ra + 4, line.length());
    nb.r = std::atoi(radstr.c_str());

    nanobots.push_back(nb);
  }

  std::sort(nanobots.begin(), nanobots.end(),
            [](const Nanobot &n0, const Nanobot &n1) { return n0.r < n1.r; });

  auto &nanobot = nanobots.back();

  int coverage =
      std::count_if(nanobots.begin(), nanobots.end(), [&](const Nanobot &n) {
        return covers(nanobot, n.x, n.y, n.z, 0);
      });
  logger::get(logtype::logINFO) << "Part 1: " << coverage << std::endl;

  int64_t x_min = 0;
  int64_t y_min = 0;
  int64_t z_min = 0;
  int64_t x_max = 0;
  int64_t y_max = 0;
  int64_t z_max = 0;
  for (auto &n : nanobots) {
    x_min = std::min(x_min, n.x - n.r);
    x_max = std::max(x_max, n.x + n.r + 1);
    y_min = std::min(y_min, n.y - n.r);
    y_max = std::max(y_max, n.y + n.r + 1);
    z_min = std::min(z_min, n.z - n.r);
    z_max = std::max(z_max, n.z + n.r + 1);
  }

  int64_t deltax = x_max - x_min;
  int64_t deltay = y_max - y_min;
  int64_t deltaz = z_max - z_min;

  int64_t scale = (int64_t(1) << int64_t(
                       (std::log(deltax + deltay + deltaz) / std::log(2)) + 1));

  x_min = (x_min / scale) * scale;
  x_max = (x_max / scale + 1) * scale;
  y_min = (y_min / scale) * scale;
  y_max = (y_max / scale + 1) * scale;
  z_min = (z_min / scale) * scale;
  z_max = (z_max / scale + 1) * scale;

  size_t nx = (x_max - x_min) / scale;
  size_t ny = (y_max - y_min) / scale;
  size_t nz = (z_max - z_min) / scale;

  std::vector<Point> points;
  for (size_t dx = 0; dx < nx; ++dx)
    for (size_t dy = 0; dy < ny; ++dy)
      for (size_t dz = 0; dz < nz; ++dz) {
        points.push_back(
            Point(x_min + dx * scale, y_min + dy * scale, z_min + dz * scale));
      }

  while (true) {
    int max_bots = 0;
    std::vector<Point> new_points;
    for (auto &point : points) {
      int num_bots = std::count_if(
          nanobots.begin(), nanobots.end(),
          [&](const Nanobot &n) { return covers(n, point, scale); });

      if (num_bots != 0 && num_bots == max_bots) {
        new_points.push_back(point);
      }
      if (num_bots > max_bots) {
        max_bots = num_bots;
        new_points.clear();
        new_points.push_back(point);
      }
    }

    if (scale == 0) {
      std::swap(points, new_points);
      break;
    }
    points.clear();
    scale /= 2;
    if (scale == 0) {
      std::swap(points, new_points);
    } else {
      for (auto &point : new_points) {
        for (int64_t dx = -scale; dx <= scale; dx += scale)
          for (int64_t dy = -scale; dy <= scale; dy += scale)
            for (int64_t dz = -scale; dz <= scale; dz += scale)
              points.push_back(Point(point.x + dx, point.y + dy, point.z + dz));
      }
      std::sort(points.begin(), points.end());
      points.erase(std::unique(points.begin(), points.end()), points.end());
    }
  }

  int64_t min_distance = std::numeric_limits<int64_t>::max();
  for (auto &point : points)
    min_distance =
        std::min(min_distance,
                 std::abs(point.x) + std::abs(point.y) + std::abs(point.z));

  logger::get(logtype::logINFO) << "Part 2: " << min_distance << std::endl;
}