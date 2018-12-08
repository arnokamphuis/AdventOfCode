#include "logger.h"
#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

enum instructiontype { HLF = 1, TPL = 2, INC = 3, JMP = 4, JIE = 5, JIO = 6 };

class instruction {
public:
  instructiontype type;
  int reg;
  int value;
};

class program {
private:
  int64_t reg[2];
  std::vector<instruction> instructions;

public:
  program(int a, int b) {
    reg[0] = a;
    reg[1] = b;
  }

  int64_t get(int r) { return reg[r]; }

  void add(std::string inst) {
    std::string type_str = inst.substr(0, 3);
    instruction i;
    if (type_str.compare("hlf") == 0) {
      i.type = instructiontype::HLF;
      i.reg = (inst.substr(4, 1).compare("a") == 0 ? 0 : 1);
    }
    if (type_str.compare("tpl") == 0) {
      i.type = instructiontype::TPL;
      i.reg = (inst.substr(4, 1).compare("a") == 0 ? 0 : 1);
    }
    if (type_str.compare("inc") == 0) {
      i.type = instructiontype::INC;
      i.reg = (inst.substr(4, 1).compare("a") == 0 ? 0 : 1);
    }
    if (type_str.compare("jmp") == 0) {
      i.type = instructiontype::JMP;
      i.value = std::atoi(inst.substr(4, inst.length()).c_str());
    }
    if (type_str.compare("jie") == 0) {
      i.type = instructiontype::JIE;
      i.reg = (inst.substr(4, 1).compare("a") == 0 ? 0 : 1);
      i.value = std::atoi(inst.substr(6, inst.length()).c_str());
    }
    if (type_str.compare("jio") == 0) {
      i.type = instructiontype::JIO;
      i.reg = (inst.substr(4, 1).compare("a") == 0 ? 0 : 1);
      i.value = std::atoi(inst.substr(6, inst.length()).c_str());
    }
    instructions.push_back(i);
  }

  void execute() {
    int pc = 0;
    bool done = false;
    while (!done) {
      instruction i = instructions[pc];

      switch (i.type) {
      case instructiontype::HLF:
        reg[i.reg] /= 2;
        ++pc;
        break;
      case instructiontype::TPL:
        reg[i.reg] *= 3;
        ++pc;
        break;
      case instructiontype::INC:
        reg[i.reg] += 1;
        ++pc;
        break;
      case instructiontype::JMP:
        pc += i.value;
        break;
      case instructiontype::JIE:
        if (reg[i.reg] % 2 == 0)
          pc += i.value;
        else
          ++pc;
        break;
      case instructiontype::JIO:
        if (reg[i.reg] == 1)
          pc += i.value;
        else
          ++pc;
        break;
      }
      done = (pc < 0) || (pc >= instructions.size());
    }
  }
};

int main() {
  std::string line;
  program p1(0, 0);
  program p2(1, 0);

  while (getline(std::cin, line)) {
    p1.add(line);
    p2.add(line);
  }

  p1.execute();
  p2.execute();

  logger::get(logtype::logINFO) << "Part 1: " << p1.get(1) << "\n";
  logger::get(logtype::logINFO) << "Part 2: " << p2.get(1) << "\n";

  return 0;
}