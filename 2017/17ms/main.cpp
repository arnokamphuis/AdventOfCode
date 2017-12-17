#include <string>
#include <sstream>
#include <iostream>
#include <vector>
#include <iomanip>
#include <bitset>
#include <algorithm>
#include <queue>
#include <utility>

int main() {

    int input = 366; // real 366, test 3

    std::vector<int> circular_buffer;

    circular_buffer.push_back(0);
    auto current = circular_buffer.begin();

    for (int i=0;i<2017;++i) {
        //std::cout << "going to insert: " << (i+1) << " with step " << input << std::endl;
        int steps_forward = input % circular_buffer.size();
        //std::cout << "step forward: " << steps_forward << std::endl;
        for (int j=0;j<steps_forward;++j) {
            ++current;
            if (current==circular_buffer.end()) current = circular_buffer.begin();
        }
        //std::cout << "current value at that position: " << *current << std::endl;
        current = circular_buffer.insert(current+1, i+1);

        //std::cout << "-------- buffer = " ;
        //for (auto cb: circular_buffer) std::cout << cb << " "; std::cout << std::endl;
    }

    std::cout << "Part 1: " << *(current+1) << std::endl;

    int bufsize = 1;
    int cp = 0;
    int iv = -1;
    int np = 0;
    for (int i=0;i<50000000;++i) {
        int ip = 1+(cp+input)%bufsize;
        if (ip==(1+np)) iv = i+1;
        if (ip<=(0+np)) ++np;
        cp = ip;
        ++bufsize;
    }
    std::cout << "Part 2: " << iv << std::endl;
}