#include <string>
#include <iostream>
#include <sstream>
#include <map>
#include <climits>

enum class comp : int { lt = 1, let, gt, get, eq, neq };

class condition {
public:
    std::string reg;
    comp        compare;
    int         value;
    
    condition(const std::string& condstr) {
        std::istringstream ss(condstr);
        std::string strcomp;
        ss >> reg;
        ss >> strcomp;
        if (strcomp.compare("<")==0) this->compare = comp::lt;
        if (strcomp.compare(">")==0) this->compare = comp::gt;
        if (strcomp.compare("<=")==0) this->compare = comp::let;
        if (strcomp.compare(">=")==0) this->compare = comp::get;
        if (strcomp.compare("==")==0) this->compare = comp::eq;
        if (strcomp.compare("!=")==0) this->compare = comp::neq;

        ss >> value;
    }

    bool can_execute(std::map<std::string,int>& regs) {
        int regvalue = regs[reg];

        switch(this->compare) {
            case comp::lt:  if (regvalue <  value) { return true;} break;
            case comp::gt:  if (regvalue >  value) { return true;} break;
            case comp::let: if (regvalue <= value) { return true;} break;
            case comp::get: if (regvalue >= value) { return true;} break;
            case comp::eq:  if (regvalue == value) { return true;} break;
            case comp::neq: if (regvalue != value) { return true;} break;
            default: return false;
        }
        return false;
    }


};

int find_max_regs(const std::map<std::string,int>& regs) {
    int maxreg = INT_MIN;
    for (auto r : regs) {
        if (r.second > maxreg) {
            maxreg = r.second;
        }
    }
    return maxreg;
}

int main() {

    int max = INT_MIN;
    std::map<std::string,int> registers;

    for (std::string line; std::getline(std::cin, line); ) {
        std::string reg;
        std::string instruction;
        int delta;
        std::string condstr;
        std::string temp;

        std::istringstream ss(line);
        ss >> reg;
        ss >> instruction;
        ss >> delta;
        
        ss >> temp;

        getline(ss,condstr);

        condition c(condstr);
        
        if (c.can_execute(registers)) {
            if (instruction.compare("inc")==0) 
                registers[reg] += delta;
            else
                registers[reg] -= delta;
        }

        int m = find_max_regs(registers);
        if (m>max) max = m;
    }
    
    int maxval = INT_MIN;
    std::string maxreg;
    for (auto reg: registers) {
        if (maxval<reg.second) {
            maxval = reg.second;
            maxreg = reg.first;
        }
    }

    std::cout << "Maximum register is " << maxreg << " with value " << maxval << std::endl;
    std::cout << "Maximum ever held was: " << max << std::endl;
}