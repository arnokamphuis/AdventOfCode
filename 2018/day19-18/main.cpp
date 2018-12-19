#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

class instruction {
public:
  std::string opstr;
  uint16_t A;
  uint16_t B;
  uint16_t C;
  uint16_t ir;
  uint16_t curreg[6];
  uint16_t resreg[6];
  uint16_t tmpreg[6];

  instruction() {
    for (uint16_t i = 0; i < 4; ++i)
      curreg[i] = 0;
  }

  std::string str() {
    return opstr + " " + std::to_string(A) + " " + std::to_string(B) + " " +
           std::to_string(C);
  }

  void execute() {
    preexec();
    if (opstr.compare("addr") == 0)
      addr();
    else if (opstr.compare("addi") == 0)
      addi();
    else if (opstr.compare("mulr") == 0)
      mulr();
    else if (opstr.compare("muli") == 0)
      muli();
    else if (opstr.compare("banr") == 0)
      banr();
    else if (opstr.compare("bani") == 0)
      bani();
    else if (opstr.compare("setr") == 0)
      setr();
    else if (opstr.compare("seti") == 0)
      seti();
    else if (opstr.compare("borr") == 0)
      borr();
    else if (opstr.compare("bori") == 0)
      bori();
    else if (opstr.compare("gtri") == 0)
      gtri();
    else if (opstr.compare("gtir") == 0)
      gtir();
    else if (opstr.compare("gtrr") == 0)
      gtrr();
    else if (opstr.compare("eqri") == 0)
      eqri();
    else if (opstr.compare("eqir") == 0)
      eqir();
    else if (opstr.compare("eqrr") == 0)
      eqrr();

    postexec();
  }

  void setinstruction(std::string line) {
    std::stringstream in(line);
    in >> opstr >> A >> B >> C;
  }

  void loadcurregister(int16_t reg[6]) {
    for (uint16_t i = 0; i < 6; ++i)
      curreg[i] = reg[i];
  }

  void getcurregister(int16_t reg[6]) {
    for (uint16_t i = 0; i < 6; ++i)
      reg[i] = curreg[i];
  }

  void preexec() {
    for (uint16_t i = 0; i < 6; ++i)
      tmpreg[i] = curreg[i];
  }

  void postexec() {
    for (uint16_t i = 0; i < 6; ++i)
      curreg[i] = tmpreg[i];
  }

  void addr() { tmpreg[C] = curreg[A] + curreg[B]; }

  void addi() { tmpreg[C] = curreg[A] + B; }

  void mulr() { tmpreg[C] = curreg[A] * curreg[B]; }

  void muli() { tmpreg[C] = curreg[A] * B; }

  void banr() { tmpreg[C] = curreg[A] & curreg[B]; }

  void bani() { tmpreg[C] = curreg[A] & B; }

  void borr() { tmpreg[C] = curreg[A] | curreg[B]; }

  void bori() { tmpreg[C] = curreg[A] | B; }

  void setr() { tmpreg[C] = curreg[A]; }

  void seti() { tmpreg[C] = A; }

  void gtir() { tmpreg[C] = (A > curreg[B] ? 1 : 0); }

  void gtri() { tmpreg[C] = (curreg[A] > B ? 1 : 0); }

  void gtrr() { tmpreg[C] = (curreg[A] > curreg[B] ? 1 : 0); }

  void eqir() { tmpreg[C] = (A == curreg[B] ? 1 : 0); }

  void eqri() { tmpreg[C] = (curreg[A] == B ? 1 : 0); }

  void eqrr() { tmpreg[C] = (curreg[A] == curreg[B] ? 1 : 0); }
};

class program {
  int16_t reg[6];
  int16_t ir;
  std::vector<instruction> instrs;

  void printreg() {
    std::cout << "[";
    for (int i = 0; i < 6; ++i)
      std::cout << reg[i] << "\t";
    std::cout << "]\n" << std::flush;
  }

public:
  program() {
    for (int i = 0; i < 6; ++i)
      reg[i] = 0;
    ir = 0;
  }

  void reset(int16_t v) {
    reg[0] = v;
    for (int i = 1; i < 6; ++i)
      reg[i] = 0;
  }

  void addinstruction(instruction i) { instrs.push_back(i); }

  void run(int initial_ir, bool print = false) {
    ir = initial_ir;
    while (true) {
      int pc = reg[ir];
      if ((pc < 0) || (pc >= instrs.size()))
        break;
      instruction inst = instrs[pc];
      inst.loadcurregister(reg);
      inst.execute();
      inst.getcurregister(reg);
      if (print) {
        std::cout << "ir: " << pc << "\t";
        std::cout << "|" << inst.str() << "|\t";
        printreg();
      }
      ++reg[ir];
    }
    --reg[ir];
    printreg();
  }

  int16_t getreg0() const { return reg[0]; }
};

int main() {

  program prog;

  std::string line;

  getline(std::cin, line);

  int ir = std::atoi(line.substr(4, 2).c_str());

  while (getline(std::cin, line)) {
    instruction i;
    i.setinstruction(line);
    prog.addinstruction(i);
  }

  prog.run(ir);

  logger::get(logtype::logINFO) << "Part 1: " << prog.getreg0() << "\n";

  prog.reset(1);
  prog.run(ir, true);
  logger::get(logtype::logINFO) << "Part 2: " << prog.getreg0() << "\n";

  return 0;
}