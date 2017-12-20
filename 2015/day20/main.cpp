
#include <iostream>
#include <cmath>

int number_of_presents(int housenumber) {
    int presents = 0;
    for (int i = 1; i <= std::sqrt(housenumber); ++i) {
        if (housenumber % i == 0) {
            if (std::sqrt(housenumber)!=i)
                presents += (i+housenumber/i)*10;
            else
                presents += i*10;
        }
    } 
    return presents;
}

int number_of_presents2(int housenumber) {
    int presents = 0;
    for (int i = 1; i <= housenumber; ++i) {
        if (housenumber % i == 0) {
            if (housenumber/i<50) {
                presents += i*11;
            }
        }
    } 
    return presents;
}

int main() {
    int i=1;
    while (true) {
        if (number_of_presents(i)>=36000000) break;
        ++i;
    }
    std::cout << "Part 1: " << i << std::endl;

    while (true) {
        int nop = number_of_presents2(i);
        if (nop>=36000000) break;
        ++i;
    }
    std::cout << "Part 2: " << i << std::endl;
}