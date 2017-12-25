#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <utility>
#include <climits>
#include <algorithm> 

class tape {
    public:
    std::vector<int> position;
    std::vector<int>::iterator cp;

    tape() { 
        position.push_back(0);
        cp = position.begin(); 
    }

    void move(int direction) {
        if (direction<0) {
            if (cp==position.begin()) { position.insert(position.begin(),0); cp = position.begin(); }
            else --cp;
        } else {
            ++cp;
            if (cp==position.end()) {
                position.push_back(0);
                cp = position.end()-1;
            }
        }
    }

    int value() { 
        return *cp; 
    }

    void set(int v) {
        *cp = v;
    }
    
    int checksum() {
        int csum = 0;
        for (auto p: position) csum += p;
        return csum;
    }
};

class state {
    public:
    std::string name;
    std::map<int,int> write_value;
    std::map<int,int> move_to;
    std::map<int,std::string> next_state;

    std::string step(tape* t) {
        int v = t->value();
        int wv = write_value[v];
        int d  = move_to[v];
        std::string ns = next_state[v];

        t->set(wv);
        t->move(d);
        return ns;
    }
};



int main() {

    std::string current_state = "A";
    int checksum_counter = -1;
    std::map<std::string, state*> states;

    tape* t = new tape;

    // read from file

    std::string line, tmp, steps;
    getline(std::cin, line); // begin state
    std::istringstream ss(line);
    ss >> tmp >> tmp >> tmp >> current_state; 
    current_state.erase(current_state.end()-1);

    getline(std::cin, line); // number of runs before checksum
    std::istringstream ss_checksum(line);
    ss_checksum >> tmp >> tmp >> tmp >> tmp >> tmp >> steps;
    checksum_counter = std::stoi(steps);

    // all states
    bool reading_state = false;
    std::string read_state = "";
    while (getline(std::cin, line)) {
        if (line.size()>0) {
            if (line.find("In state ")!=std::string::npos) { // begin of state
                reading_state = true;
                read_state = std::string("") + line[9]; 
                states[read_state] = new state();

                for (int q=0; q<2; ++q) {
                    getline(std::cin, line);
                    int s = std::stoi(std::string("")+line[26]);

                    getline(std::cin, line);
                    states[read_state]->write_value[s] = std::stoi(std::string("")+line[22]);

                    getline(std::cin, line);
                    if (line.find("left")!=std::string::npos) {
                        states[read_state]->move_to[s] = -1; 
                    } else {
                        states[read_state]->move_to[s] = 1;
                    }

                    getline(std::cin, line);
                    states[read_state]->next_state[s] = std::string("")+line[26];
                }
            }
        }
    }

    // end read from file

    std::cout << "going to perform " << checksum_counter << " steps " << std::endl;
    while (checksum_counter>0) {
        current_state = states[current_state]->step(t);
        --checksum_counter;
    }

    std::cout << "Part 1: " << t->checksum() << std::endl;
    return 0;
}