#include "logger.h"

#include <iostream>

std::ostream& logger::get(logtype lt) {
    if (lt == logtype::logERROR) return std::cerr;
    else return std::cout;
}