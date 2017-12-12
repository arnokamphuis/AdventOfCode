#include <iostream>
#include <sstream>
#include <string>
#include <map>
#include <algorithm>
#include <utility>

int main() {
    std::string line;
    getline(std::cin, line);

    std::map< std::pair<int,int>, int > presents_delivered;
    std::vector< std::pair<int,int> > cp;

    int deliverers = 2;
    int cd = 0;

    for (int i=0;i<deliverers;++i) {
        cp.push_back(std::make_pair(0,0));
        if (i==0) presents_delivered[cp[i]]+=1;
    }

    for (auto ch: line) {
        cd = (cd+1) % deliverers; // go to next deliverer
        switch (ch) {
            case '>': {
                cp[cd].first+=1;
            }
            break;
            case '^': {
                cp[cd].second+=1;
            }
            break;
            case '<': {
                cp[cd].first-=1;
            }
            break;
            case 'v': {
                cp[cd].second-=1;
            }
            break;
        }
        presents_delivered[cp[cd]]+=1;
        //std::cout << cp[cd].first << " " << cp[cd].second << std::endl;
    }

    std::cout << "Part 1: " << presents_delivered.size() << std::endl;
}