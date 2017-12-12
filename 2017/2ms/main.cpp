#include<iostream>
#include<stdlib.h>
#include<stdio.h>
#include<vector>
#include <algorithm>  
#include <climits>
#include <sstream>
#include <iterator>

int toint(const char& c) {
    return c - '0';
}

int find_min(const std::vector<int>& l) {
    int m = INT_MAX;
    for (auto v: l)
        if (v<m) m = v;
    return m;
}

int find_max(const std::vector<int>& l) {
    int m = INT_MIN;
    for (auto v: l)
        if (v>m) m = v;
    return m;
}

int main() {
    char ch;
    int size=0;
    int sum = 0;
    std::vector<int> list;

    std::string line;
    std::vector< std::vector<int> > all_integers;
    while ( getline( std::cin, line ) ) {
        std::istringstream is( line );
        all_integers.push_back( 
                std::vector<int>( std::istream_iterator<int>(is),
                                std::istream_iterator<int>() ) );
    }    

    for (auto list : all_integers) {
        int minimum = find_min(list);
        int maximum = find_max(list);
        sum += maximum - minimum;
    }

    std::cout << "Part 1: " << sum << std::endl;

    sum = 0;

    for (auto list : all_integers) {
        int s = list.size();
        for (int i=0; i < s; i++) {
            for (int j = 0; j < i; j++) {
                if ( list[i] % list[j] == 0) sum += list[i]/list[j];
                if ( list[j] % list[i] == 0) sum += list[j]/list[i];
            }
        }
    }    

    std::cout << "Part 2: " << sum << std::endl;

}