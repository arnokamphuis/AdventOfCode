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
        // find the number of steps to take forward
        int steps_forward = input % circular_buffer.size();
        // take steps forward
        for (int j=0;j<steps_forward;++j) {
            ++current;
            // if move beyond end, start at beginning
            if (current==circular_buffer.end()) current = circular_buffer.begin();
        }
        // insert value and update the current iterator to that position
        current = circular_buffer.insert(current+1, i+1);
    }

    std::cout << "Part 1: " << *(current+1) << std::endl;


    // Part two is too complicated to fully expand. So only track the 0 position and the value after it
    int bufsize = 1; // size of the circular buffer
    int cp = 0;      // current position for insert
    int iv = -1;     // last inserted value after 0
    int np = 0;      // current position of 0

    for (int i=0;i<50000000;++i) {
        int ip = 1+(cp+input)%bufsize;  // find position to insert at
        if (ip==(1+np)) iv = i+1;       // if insert position if directly after 0, remember value
        if (ip<=(0+np)) ++np;           // if insert position was before or at position of 0, move 0 position on
        cp = ip;                        // current position is at the insert position
        ++bufsize;                      // the buffer has increased by 1
    }
    std::cout << "Part 2: " << iv << std::endl;
}