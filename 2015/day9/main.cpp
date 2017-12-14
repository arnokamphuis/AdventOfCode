#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <set>
#include <utility>
#include <algorithm>
#include <climits>

int main() {
    std::set<std::string> places;

    std::map< std::pair<std::string, std::string>, int> distances;
    std::string line;
    while (getline(std::cin,line)) {
        std::istringstream ss(line);
        std::string from, to, tmp;
        int distance;
        ss >> from >> tmp >> to >> tmp >> distance;

        std::pair<std::string, std::string> route1 = std::make_pair(from,to);
        std::pair<std::string, std::string> route2 = std::make_pair(to,from);
        distances[route1] = distance;
        distances[route2] = distance;

        places.insert(from);
        places.insert(to);
    }

    std::vector<std::string> ordered_places;
    for (auto p: places) ordered_places.push_back(p);

    int shortest = INT_MAX;
    int longest = INT_MIN;
    do {
        int d=0;
        std::string prev_place = "";
        for (auto p: ordered_places) {
            if (prev_place.size()!=0) {
                std::pair<std::string, std::string> route = make_pair(prev_place,p);
                d += distances[route];
            }
            prev_place = p;
        }
        if (d<shortest) shortest = d;
        if (d>longest) longest = d;
    } while(std::next_permutation(ordered_places.begin(), ordered_places.end()));

    std::cout << "Part 1: " << shortest << std::endl;
    std::cout << "Part 2: " << longest << std::endl;
}