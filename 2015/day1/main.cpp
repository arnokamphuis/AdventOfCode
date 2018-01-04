#include <string>
#include <iostream>
#include <sstream>
#include "logger.h"

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

    logger::get(logtype::logINFO) << "Part 1: " << floor << '\n';
    logger::get(logtype::logINFO) << "Part 2: " << position << '\n';

    return 0;
}