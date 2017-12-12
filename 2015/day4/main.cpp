#include <iostream>
#include <string>
#include <numeric>

#include "md5.h"

int main() {

    std::string line;
    getline(std::cin,line);

    int counter=1;
    std::string msg;

    while(true) {
        msg = line + std::to_string(counter);
        std::string hash = MD5(msg).hexdigest();

        if (hash.find("000000")==0) { 
            std::cout << counter << std::endl;
            break;
        }

        ++counter;
    }
}