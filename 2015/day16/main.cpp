#include <iostream>
#include <sstream>
#include <string>
#include <map>
#include <vector>
#include <climits>
#include "logger.h"

class aunt {
public:
    int number;
    std::map<std::string,int> possessions;

    explicit aunt(std::string description) {
        std::string line = description;
        for (int i=0;i<4;i++) line.erase(line.begin());
        int colon = line.find(":");
        number = std::stoi(line.substr(0,colon));
        line = line.substr(colon+2);

        std::istringstream ss(line);
        std::string poss;
        while (getline(ss,poss,',')) {
            colon = poss.find(":");
            std::string possname = poss.substr(0,colon);
            while (possname[0]==' ') possname.erase(possname.begin());
            int posvalue = std::stoi(poss.substr(colon+2));
            possessions[possname] = posvalue;
        }
    }

    bool check(std::map<std::string,int>& list, std::map<std::string,int>& comp) {
        for(auto p: possessions) {
            if (comp[p.first]==0) {
                if (list[p.first] != p.second) return false;
            } else if (comp[p.first]==1) { // less than
                if (list[p.first] >= p.second) return false;                
            } else if (comp[p.first]==-1) { // greater than
                if (list[p.first] <= p.second) return false;                                
            }
        }
        return true;
    }
};

int main() {
    std::map<std::string,int> list;
    list["children"] = 3;
    list["cats"] = 7;
    list["samoyeds"] = 2;
    list["pomeranians"] = 3;
    list["akitas"] = 0;
    list["vizslas"] = 0;
    list["goldfish"] = 5;
    list["trees"] = 3;
    list["cars"] = 2;
    list["perfumes"] = 1;

    std::map<std::string,int> comp;
    comp["children"] = 0;
    comp["cats"] = 1; //gt
    comp["samoyeds"] = 0;
    comp["pomeranians"] = -1;  //lt
    comp["akitas"] = 0;
    comp["vizslas"] = 0;
    comp["goldfish"] = -1; //lr
    comp["trees"] = 1; // gt
    comp["cars"] = 0;
    comp["perfumes"] = 0;

    std::map<int,aunt*> aunts;
    std::string line;
    while(getline(std::cin,line)) {
        aunt* sue = new aunt(line);
        aunts[sue->number] = sue;

        if (sue->check(list,comp))
            logger::get(logtype::logINFO) << "Answer: " << sue->number << std::endl;
    }
}