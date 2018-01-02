#include <string>
#include <sstream>
#include <iostream>
#include <vector>
#include <iomanip>
#include "knot.h"

int main() {
    
    std::string line;
    getline(std::cin, line);

    std::string hash = knot_hash(line);

    std::cout << "Part 2: " << hash << std::endl;
}