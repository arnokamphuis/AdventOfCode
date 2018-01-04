#include <iostream>
#include <sstream>
#include <string>
#include <map>
#include <vector>
#include <climits>

void generate_combination(int size, int sum, std::vector<int> combination, std::vector<std::vector<int> >& allcombs) {
    if (size-combination.size()==0) {
        allcombs.push_back(combination);
        return;
    }
    for (int s=sum; s>=0; --s) {
        std::vector<int> newcomb = combination;
        newcomb.push_back(s);
        generate_combination(size,sum-s,newcomb,allcombs);
    }
}

class property {
public: 
    std::map<std::string,int> values;  

    int score(std::map<std::string,int> ingredients) {
        int s = 0;
        for (auto ing: ingredients)
            s += values[ing.first] * ing.second;
        if (s<0) s=0;
        return s;
    }
};

int main() {
 
    std::vector<std::vector<int> > ac;
    std::vector<int> c;
 
    std::map<std::string, property*> properties;
    std::map<std::string, int> ingredients;

    properties["capacity"] = new property;
    properties["durability"] = new property;
    properties["flavor"] = new property;
    properties["texture"] = new property;
    properties["calories"] = new property;

    std::string line;
    while (getline(std::cin, line)) {
        std::istringstream ss(line);
        std::string name;
        std::string rest;
        std::string tmp;
        std::string propname;
        int propval;
        getline(ss, name, ':');
        ingredients[name] = 0;
        getline(ss, rest, ':');
        std::istringstream sin(rest);
        while (getline(sin, tmp, ',')) {
            while (tmp[0]==' ') tmp.erase(tmp.begin());

            int space = tmp.find(" ");
            propname = tmp.substr(0,space);
            propval  = std::stoi(tmp.substr(space+1));
            properties[propname]->values[name] = propval;
        }
    }

    ingredients.begin()->second = 100;

    int maxscore = INT_MIN;
    generate_combination(ingredients.size(), 100, c, ac);
    for (auto comb: ac) {
        auto ig = ingredients.begin();
        for (auto cv = comb.begin() ; cv != comb.end(); ++cv, ++ig) {
            ig->second= *cv;
        }

        if (properties["calories"]->score(ingredients) == 500) {
            int score = 1;
            for (auto p: properties) {
                if (p.first.compare("calories")!=0) {
                    int is = p.second->score(ingredients);
                    score *= is;
                }
            }
            if (score > maxscore) maxscore = score;
        }
    }

    logger::get(logtype::logINFO) << "Part 2: " << maxscore << std::endl;
}