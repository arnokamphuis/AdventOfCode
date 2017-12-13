#include <iostream>
#include <sstream>
#include <map>
#include <vector>
#include <climits>

int severity(std::map<int,int>& scanners, int delay, bool& caught) {
    int danger = 0;
    caught = false;
    for (auto s: scanners) {
        int t = s.first+delay;
        int m = 2*(s.second-1);
        if ( (t % m) == 0) {
            danger += s.first*s.second;
            caught=true;
        }
    }
    return danger;
}

int main() {
    std::map<int,int> scanners;
    std::string line;

    while (getline(std::cin,line)) {
        std::istringstream ss(line);
        std::vector<int> v;
        for(std::string each; getline(ss,each,':');v.push_back(std::stoi(each)));
        scanners[v[0]]=v[1];
    }

    bool caught = true;
    int sev = INT_MAX;
    
    std::cout << "Part 1: " << severity(scanners,0,caught) << std::endl;

    int delay=0;
    while (true) {
        sev = severity(scanners, delay, caught);
        if (!caught) break;
        ++delay;
    }

    std::cout << "Part 2: " << delay << std::endl;
}