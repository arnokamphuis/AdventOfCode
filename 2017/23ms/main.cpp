#include <iostream>
#include <sstream>
#include <vector>
#include <map>
#include <queue>
#include <set>
#include <algorithm>
#include <climits>  
#include <string>
#include <utility>

typedef std::vector<std::string> instruction_type;
typedef int64_t memory_type;

class program {
    public:
    memory_type id;
    int send_counter, mul_counter, counter;
    
    instruction_type instructions;
    instruction_type::iterator ci;
    std::queue<memory_type> messages;
    memory_type registers[8];
    std::map<char,int> reg_index;
    program* other;
    bool waiting;

    program(memory_type pn) : id(pn), send_counter(0), mul_counter(0), counter(0) {
    }

    void print_registers() {
        for (auto r: reg_index) {
            std::cout << "|" << r.first << "=" << registers[r.second] << "|";
        }
        std::cout << std::endl;
    }

    void init() {
        waiting = false;
        ci = instructions.begin();
        int count=0;
        for (auto& ri: reg_index) {
            ri.second = count;
            registers[ri.second] = 0;
            ++count;
        }
        registers[reg_index['a']] = id;
    }

    void add_instruction(std::string inst) {
        instructions.push_back(inst);
        std::istringstream ss(inst);
        std::string tmp, reg_str;
        ss >> tmp >> reg_str >> tmp;
        while (reg_str[0]==' ') reg_str.erase(reg_str.begin());
        int tmp_a = reg_str[0]-'a';
        int s = reg_index.size();
        if ( (0 <= tmp_a) and (tmp_a <=26) ) reg_index[reg_str[0]]=s;
    }

    bool jump(int steps) {
        if (steps == 0) return true;
        if (steps < 0) {
            if (ci!=instructions.begin()) {
                --ci;
                return jump(steps+1);
            } else return false;
        } else {
            ++ci;
            if (ci!=instructions.end()) {
                return jump(steps-1);
            } else return false;
        }
    }

    bool execute_instruction() {
        std::string current_instruction = *ci;

        ++counter;

        int ln = (int) (ci-instructions.begin());

        bool jumped = false;
        waiting = false;

        std::string inst, param0, param1;
        std::istringstream ss(current_instruction);
        ss >> inst >> param0 >> param1;

        while (param0[0]==' ') param0.erase(param0.begin());
        while (param1[0]==' ') param1.erase(param1.begin());

        int reg_value, value;
        char reg;

        //std::cout <<"INSTRUCTION: |" << inst << "|" << std::endl;

        if (  ! ( (inst.compare("jnz")==0) ) ) {
            reg = param0[0];
            reg_value = registers[reg_index[reg]];
            int tmp_a = param1[0]-'a';
            if ( (0 <= tmp_a) and (tmp_a <=26) ) value = registers[reg_index[param1[0]]];
            else value = std::stoi(param1);

            if (inst.compare("set")==0)   registers[reg_index[reg]] = value;
            if (inst.compare("mul")==0) { registers[reg_index[reg]] *= value; ++mul_counter; }
            if (inst.compare("sub")==0)   registers[reg_index[reg]] -= value;
        } else if (inst.compare("jnz")==0) {

            int check_value;
            int tmp_a = param0[0]-'a';
            if ( (0 <= tmp_a) and (tmp_a <=26) ) check_value = registers[reg_index[param0[0]]];
            else check_value = std::stoi(param0);

            tmp_a = param1[0]-'a';
            if ( (0 <= tmp_a) and (tmp_a <=26) ) value = registers[reg_index[param1[0]]];
            else value = std::stoi(param1);

            //std::cout << "  do i jump? " << check_value << "\t steps to jump: " << value << std::endl;
            if (check_value != 0) {
                if (jump(value)) {
                    jumped = true;
                } else {
                    waiting = false;
                    return false;
                }
            }
            //std::cout << "  done jumping" << std::endl;
        }
        
        //print_registers();
        //std::cout << "============================================================" << std::endl;

        if (!jumped) ++ci;
        return true;
    }

    void send(memory_type value) { 
        //std::cout << "SEND: " << id << ":" << value << std::endl;
        other->messages.push(value); 
        ++send_counter; 
    }
    bool can_continue() { return messages.size()>0; } 
    bool is_waiting() { return waiting; } 
};

int main() {
    program part1(0), part2(1);

    std::string line;
    while (getline(std::cin,line)) {
        part1.add_instruction(line);
        part2.add_instruction(line);
    }

    part1.init();
    part2.init();

    while (part1.execute_instruction()) {}
    std::cout << "Part 1: " << part1.mul_counter << std::endl;

    while (part2.execute_instruction()) {}
    std::cout << "Part 2: " << part2.registers[part2.reg_index['h']] << std::endl;
}