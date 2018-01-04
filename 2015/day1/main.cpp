#include <string>
#include <iostream>
#include <sstream>

#include "logger.h"

logger log(std::cout, std::cerr);

int main() {
    int position = -1;
    std::string line;
    getline(std::cin, line);

    int count=0;
    int floor = 0;
    for (char c: line) {
        ++count;
        if (c == '(') ++floor;
        if (c == ')') --floor;
        if ( (floor<0) && (position<0) ) position=count;
    }

    log.log("Part 1: "); log.log(floor); log.log('\n');
    log.log("Part 2: "); log.log(position); log.log('\n');

    return 0;
}