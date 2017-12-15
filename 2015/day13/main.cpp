#include <vector>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <map>
#include <set>
#include <string>
#include <utility>
#include <climits>

int main() {
    std::string line;
    std::set<std::string> guests;
    std::map< std::pair<std::string,std::string>, int> happinessgains;

    while (getline(std::cin,line)) {
        std::string from, to, temp, gain;
        int value;
        std::istringstream ss(line);
        ss >> from >> temp >> gain >> value >> temp >> temp >> temp >> temp >> temp >> temp >> to;
        to = to.substr(0,to.size()-1);

        std::pair<std::string, std::string> neighbours = make_pair(from,to);
        if (gain.compare("lose")==0) value *= -1;

        happinessgains[neighbours] = value;
        guests.insert(from);
        guests.insert(to);
    }

    for (auto g: guests) {
        happinessgains[make_pair("arno",g)] = 0;
        happinessgains[make_pair(g,"arno")] = 0;
    }
    guests.insert("arno");

    std::vector<std::string> seating;
    for (auto g: guests) {
        seating.push_back(g);
    }

    int maxhappiness = INT_MIN;
    int size = seating.size();
    do {
        int happiness = 0;
        for (int i=0; i<size; ++i) {
            std::string from = seating[i];
            std::string to = seating[(i+1)%size];
            happiness += happinessgains[make_pair(from,to)];
            happiness += happinessgains[make_pair(to,from)];
        }
        if (happiness > maxhappiness) maxhappiness = happiness;
    } while(std::next_permutation(seating.begin(), seating.end()));
    
    std::cout << "Part 2: " << maxhappiness << std::endl;
}