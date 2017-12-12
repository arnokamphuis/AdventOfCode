#include <stack>
#include <iostream>
#include <vector>


class group {
public:
    std::string content;
    std::vector<group*> children;

    void print(int indent) {
        for (int i=0; i<indent; i++) std::cout << " ";
        std::cout << "group" << std::endl;
        for (auto c: children) {
            c->print(indent+2);
        }
    }

    int calculate_score(int level) {
        int score = level;
        for (auto c: children) {
            score += c->calculate_score(level+1);
        }
        return score;
    }
};

enum class state : int { group = 1, garbage, ignore };

int main() {
    char c;
    int garbagecounter=0;
    std::stack<state> states;
    std::stack<group*> groups;

    group* root = new group;
    groups.push(root);
    states.push(state::group);

    while ((c = getchar()) !=EOF) {   
        switch(states.top()) {
            case state::ignore:
            {
                states.pop();
                break;
            }
            case state::group:
            {
                switch(c) {
                    case '}': { // stop group
                        states.pop();
                        group* g = groups.top();
                        groups.pop();
                        groups.top()->children.push_back(g);
                        break;
                    }
                    case '{': { // start new group
                        group* ng = new group;
                        groups.push(ng);
                        states.push(state::group);
                        break;
                    }
                    case '<': { // start new garbage
                        states.push(state::garbage);
                        break;
                    }
                    default: {
                        groups.top()->content += c;
                        break;
                    }
                }
                break;
            }
            case state::garbage:
            {
                switch(c) {
                    case '!': {
                        states.push(state::ignore);
                        break; 
                    }                   
                    case '>': { // end garbage
                        states.pop();
                        break;
                    }
                    default: {
                        garbagecounter++;
                        break;
                    }
                }
                break;
            }
        }

    }

    //root->print(0);
    std::cout << "root has score " << root->calculate_score(0) << std::endl;
    std::cout << "Garbage counted: " << garbagecounter << std::endl;
}