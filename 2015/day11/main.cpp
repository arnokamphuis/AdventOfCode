#include <iostream>
#include <string>
#include "logger.h"

std::string inc(const std::string& str) {
    std::string output = str;
    int ci=str.size()-1;
    while(true) {
        char c = output.at(ci);

        char nc = c+1;
        if ( (nc-'z')==1 ) {
            output.at(ci) = 'a';
            --ci;
        } else {
            output.at(ci) = nc;
            break;
        }
    }
    return output;
}

bool check_increasing(std::string str) {
    for (int i=2;i<8;++i)
        if ( ((str[i] - str[i-1])==1) && ((str[i-1] - str[i-2])==1) ) return true;
    return false;
}

bool check_vowels(const std::string& str) {
    if ( ! ( (str.find('o')!=std::string::npos) || 
        (str.find('i')!=std::string::npos) || 
        (str.find('l')!=std::string::npos) ) ) {
            return true;
        }
    return false;
}

bool check_doubles(const std::string& str) {
    int double_counter = 0;
    for (int i=2;i<8;++i)
        if ( (str[i] - str[i-1])==0 ) { ++double_counter; ++i; }
    return (double_counter==2);
}

bool check(const std::string& str) {
    return check_doubles(str) && check_increasing(str) && check_vowels(str);
}

int main() {
    std::string input = "hepxcrrq";
    std::string output = input;

    for (int i=0;i<2;++i) {
        while(!check(output))
            output = inc(output);

        logger::get(logtype::logINFO) << "Part " << (i+1) << ": " << output << '\n';
        output = inc(output);
    }

    return 0;
}