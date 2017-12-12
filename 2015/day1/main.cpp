#include <string>
#include <iostream>
#include <sstream>

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

    std::cout << "Part 1: " << floor << std::endl;
    std::cout << "Part 2: " << position << std::endl;
}