#include <iostream>
#include <sstream>
#include <vector>
#include <string>
#include <climits>
#include <algorithm>

int main() {
    std::string line;
    std::vector<std::string> lines;
    int min_d = INT_MAX;
    int i=0, min_p = -1;
    while (getline(std::cin,line)) {
        lines.push_back(line);

        std::size_t p = line.find(", a=<");
        std::string acc_str = line.substr(p+5);
        acc_str.erase(acc_str.end()-1);
        std::istringstream ss(acc_str);
        std::string a; 
        int ax, ay, az;
        getline(ss,a,','); ax = std::abs(std::stoi(a));
        getline(ss,a,','); ay = std::abs(std::stoi(a));
        getline(ss,a,','); az = std::abs(std::stoi(a));

        int d_eventually = ax+ay+az;
        if (d_eventually<min_d) {
            min_d = d_eventually;
            min_p = i;
        }
        ++i;
    }
    std::cout << "Part 1: " << min_p << std::endl;
}