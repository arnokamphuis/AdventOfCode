#include <iostream>
#include <sstream>
#include <vector>
#include <map>
#include <set>
#include <algorithm>
#include <climits>  
#include <string>
#include <utility>

typedef std::vector< std::pair< std::string, std::pair<char,std::string> > > instruction_type;

bool move(instruction_type::iterator& ci, instruction_type& instructions, int steps) {
    if (steps==0) return true;
    else if (steps<0) {
        if (ci==instructions.begin()) return false;
        else move(--ci,instructions,steps+1);
    } else {
        ++ci;
        if (ci==instructions.end()) return false;
        else move(ci,instructions,steps-1);        
    }
}

int perform_instructions(instruction_type& instructions, 
    int* registers, std::map< char, int > reg_index) {

        int sound_played = -1;
        instruction_type::iterator ci = instructions.begin();

        std::string instr, val_str;
        char reg;
        int val;

        while (true) {
            instr   = ci->first;
            reg     = ci->second.first;
            val_str = ci->second.second;
            bool jumped = false;

            if ( !((instr.compare("snd")==0) or (instr.compare("rcv")==0)) ) {
                int tmp = (val_str[0]-'a');
                std::cout << "--------------------|" << val_str[0] << "| " <<  tmp << std::endl;
                if (  (0<=tmp) && (tmp<=26) ) val = registers[reg_index[val_str[0]]];
                else val = std::stoi(val_str);
            }

            int* regist = &(registers[reg_index[reg]]);
            int inst_val = *regist;

            std::cout << "Executing instruction: " << instr << " " << reg << " " << val << " (" << inst_val <<  ")" << std::endl;

            if (instr.compare("set")==0) *regist = val;
            if (instr.compare("add")==0) *regist += val;
            if (instr.compare("mul")==0) *regist *= val;
            if (instr.compare("mod")==0) *regist = *regist % val;

            if (instr.compare("snd")==0) { sound_played = val;
                std::cout << "PLAY" << val << std::endl;
            }

            if (instr.compare("rcv")==0)  {
                if (val!=0) return sound_played;
            } // TODO

            if (instr.compare("jgz")==0)  {
                int steps = val;
                if (inst_val>0) {
                    std::cout << "JUMP" << std::endl;
                    if (!move(ci, instructions, steps )) {
                        return -1; 
                    } else { jumped = true; }
                }
            }

            if (!jumped) ++ci;

            std::cout << "=======================================" << std::endl;
            for (auto r: reg_index) {
                std::cout << r.first << "\t" << registers[r.second] << std::endl;
            }
            std::cout << "=======================================" << std::endl;
        }        
    }


int main() {

    std::set< char > registers_found;
    std::map< char, int > reg_index;
    instruction_type instructions;
    std::string line;
    while (getline(std::cin,line)) {
        std::istringstream ss(line);
        std::string instr, reg, val;
        ss >> instr >> reg >> val;
        while(val[0]==' ') val.erase(val.begin());
        instructions.push_back( std::make_pair(instr, std::make_pair(reg[0],val)) );
        registers_found.insert(reg[0]);
    }
    int regsize = registers_found.size();
    int *registers = new int[regsize];
    int c=0;
    for (auto r: registers_found) { 
        reg_index[r] = c;
        registers[c++]=0;
    }


    std::cout << "Part 1: " << perform_instructions(instructions, registers, reg_index) << std::endl;
}