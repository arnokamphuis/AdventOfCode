#include <iostream>
#include <string>
#include <vector>
#include <stack>
#include "logger.h"

enum class state { number, other };

class group {
public:
    group() : isarray(false), level(0), str("      ") {}

    bool isarray;
    int level;
    std::string str;
    std::vector<int> numbers; 
    std::vector<group*> children; 

    void print(int indent) {
        std::string ind = "";
        for (int i=0;i<level*5;++i) ind += " ";
        logger::get(logtype::logINFO) << ind << (isarray?"array":"group") << std::endl;
        logger::get(logtype::logINFO) << ind << "------------------------------------" << std::endl;
        logger::get(logtype::logINFO) << ind << str << std::endl;
        logger::get(logtype::logINFO) << ind << "------------------------------------" << std::endl;
        logger::get(logtype::logINFO) << ind << "found red? : " << (str.find("\"red\"" ) != std::string::npos) << std::endl;
        logger::get(logtype::logINFO) << ind << "children: " << children.size() << std::endl;
        for (auto n: numbers) logger::get(logtype::logINFO) << ind << n << std::endl;
        for (auto g: children) g->print(indent+5);
    }

    int calculate() {
        int sum = 0;
        bool foundred = (str.find("\"red\"")!=std::string::npos);
        if ( isarray || ( (!isarray) && (!foundred) ) ) {
            for (auto n: numbers) sum+=n;
            for (auto g: children) sum+=g->calculate();    
        }
        return sum;
    }
};

int main() {

    group* root = new group;
    std::stack<group*> groups;

    group* current_group = root;
    groups.push(root);

    char c;
    std::string nrstr;
    int number;
    bool positive = true;

    state s = state::other;

    while ((c=getchar())!=EOF) {

        if ( (c=='{') or (c=='[') ) {
            group* ng = new group;
            ng->level = current_group->level+1;
            ng->isarray = (c=='[');
            current_group->children.push_back(ng);
            current_group = ng;
            groups.push(ng);

        } 

        current_group->str += c;

        if (s == state::number) {
            int value = c-'0';
            if ( (0 <= value) && ( value <= 9 ) ) {
                nrstr += std::to_string(value);
            } else {
                number = std::stoi(nrstr);
                if (!positive) number *= -1;
                current_group->numbers.push_back(number);

                nrstr = "";
                positive = true;
                s = state::other;
            }
        } else if (s == state::other) {
            int value = c-'0';
            if ( (0<=value) && (value<=9) ) {
                nrstr = std::to_string(c-'0');
                s = state::number;
            } else if (c=='-') {
                positive = false;
                s = state::number;
                nrstr = "";
            } else  { 
                positive = true;
            }
        }

        if ( (c=='}') || (c==']') ) {
            groups.pop(); current_group = groups.top(); 
        }

    }

    logger::get(logtype::logINFO) << "Part 2: " << root->calculate() << std::endl;
}