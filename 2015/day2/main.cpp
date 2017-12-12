#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <algorithm>

int main() {
    std::string line;

    int counter = 0;
    int totalsize = 0;
    int totallength = 0;
    while (getline(std::cin, line)) {
        std::istringstream ss(line);
        std::vector<int> dimensions;
        for (std::string each; getline(ss,each,'x'); dimensions.push_back(std::stoi(each)));

        std::sort(dimensions.begin(),dimensions.end());
        ++counter;

        int s1 = dimensions[0];
        int s2 = dimensions[1];
        int s3 = dimensions[2];
        int size = 2 * ( s1*s2 + s2*s3 + s1*s3 ) + s1*s2;
        totalsize += size;
        
        int ribbonlength = 2 * ( s1+s2 ) + s1*s2*s3;
        totallength += ribbonlength;
    }

    std::cout << "Part 1: " << totalsize << std::endl;
    std::cout << "Part 2: " << totallength << std::endl;
    
}