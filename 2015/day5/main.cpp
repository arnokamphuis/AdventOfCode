#include <string>
#include <iostream>
#include <algorithm>
#include <vector>

bool checkdoubleskip(std::string str) {
    bool correct = false;
    for (int i=0; i<str.size()-2;i++)
        if (str[i]==str[i+2]) correct = true;
    return correct;
}

bool checkpairtwice(std::string str) {
    bool correct = false;
    std::string pair;
    std::vector<int> pos;

    for (int i=0; i<str.size()-1; i++) {
        pair = str.substr(i,2);
        pos.clear();
        for (int j=0; j<str.size()-1; j++) {
            if (pair.compare(str.substr(j,2))==0) {
                pos.push_back(j);
            }
        }

        /*
        std::cout << str << "----------------------------------" << std::endl;
        for (auto p : pos) std::cout << p << std::endl;
        std::cout << "----------------------------------" << std::endl;
        */

        if (pos.size()>1) {
            for (int j=0; j<pos.size()-1; ++j) {
                if ( abs(pos[j]-pos[j+1]) > 1 )
                    correct = true;
            }
        }
    }
    return correct;
}

bool checkdouble(std::string str) {
    bool correct = false;
    for (int i=0; i<str.size()-1;i++)
        if (str[i]==str[i+1]) correct = true;
    return correct;
}

int checkvowel(std::string str, char c) {
    return std::count(str.begin(), str.end(), c);
}
bool checkvowels(std::string str) {
    return ( 
        checkvowel(str,'a') + 
        checkvowel(str,'e') + 
        checkvowel(str,'o') + 
        checkvowel(str,'i') + 
        checkvowel(str,'u') ) >= 3; 
}

bool checkbadstrings(std::string str) {
    std::string bad1 = "ab"; 
    std::string bad2 = "cd"; 
    std::string bad3 = "pq"; 
    std::string bad4 = "xy"; 

    return (  
        (str.find(bad1) == std::string::npos) &&
        (str.find(bad2) == std::string::npos) &&
        (str.find(bad3) == std::string::npos) &&
        (str.find(bad4) == std::string::npos));
}

int main() {
    int nice = 0;
    int bad  = 0;
    std::string line;

    while (getline(std::cin, line)) {
        //if ( ! ( checkvowels(line) && checkbadstrings(line) && checkdouble(line) ) ) {
        if ( ! ( checkdoubleskip(line) && checkpairtwice(line) ) ) {
            ++bad;
        } else {
            ++nice;
        }
    }
    std::cout << "Part 2: " << nice << std::endl;
}