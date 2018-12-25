#include "logger.h"
#include <algorithm>
#include <chrono>
#include <iostream>
#include <list>
#include <map>
#include <math.h>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

uint64_t divsum(uint64_t n) {
  // Sum of divisors
  uint64_t result = 0;

  // find all divisors which divides 'num'
  for (uint64_t i = 2; i <= std::sqrt(n); i++) {
    // if 'i' is divisor of 'n'
    if (n % i == 0) {
      // if both divisors are same
      // then add it once else add
      // both
      if (i == (n / i))
        result += i;
      else
        result += (i + n / i);
    }
  }

  // Add 1 and n to result as above loop
  // considers proper divisors greater
  // than 1.
  return (result + n + 1);
}

template <typename T>
std::vector<T> factorize(T num) { // works great for smooth numbers
  std::vector<T> v;
  if (num < 4) {
    v.push_back(num);
    return v;
  }
  T d{2};
  while (num >= d * d) {
    while (num % d == 0) { // remove all repeats of this divisor
      v.push_back(d);
      num /= d;
    }
    ++d;
    if (d * d > num && num > 1) {
      v.push_back(num);
      return v;
    }
  }
  return v;
}

class instruction {
public:
  std::string opstr;
  uint64_t A;
  uint64_t B;
  uint64_t C;
  uint64_t ir;
  uint64_t curreg[6];
  uint64_t resreg[6];
  uint64_t tmpreg[6];

  instruction() {
    for (uint64_t i = 0; i < 4; ++i)
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

  void loadcurregister(uint64_t reg[6]) {
    for (uint64_t i = 0; i < 6; ++i)
      curreg[i] = reg[i];
  }

  void getcurregister(uint64_t reg[6]) {
    for (uint64_t i = 0; i < 6; ++i)
      reg[i] = curreg[i];
  }

  void preexec() {
    for (uint64_t i = 0; i < 6; ++i)
      tmpreg[i] = curreg[i];
  }

  void postexec() {
    for (uint64_t i = 0; i < 6; ++i)
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
  uint64_t reg[6];
  uint64_t ir;
  std::vector<instruction> instrs;
  std::set<uint64_t> seen;

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

  void reset(uint64_t v) {
    reg[0] = v;
    for (int i = 1; i < 6; ++i)
      reg[i] = 0;
  }

  void addinstruction(instruction i) { instrs.push_back(i); }

  void run(int initial_ir, int pc_stop, bool part2 = false,
           bool print = false) {
    ir = initial_ir;
    uint64_t prev;
    while (true) {
      int pc = reg[ir];
      if ((pc < 0) || (pc >= instrs.size()))
        break;
      instruction inst = instrs[pc];
      inst.loadcurregister(reg);
      inst.execute();
      inst.getcurregister(reg);
      if (/*(pc == pc_stop) &&*/ (print)) {
        std::cout << "ir: " << pc << "\t";
        std::cout << "|" << inst.str() << "|\t";
        printreg();
      }
      if (pc == pc_stop) {
        if (!part2)
          break;
        else {
          // std::cout << "reg2: " << getreg2() << " size of seen: " <<
          // seen.size()
          //           << std::endl;
          if (seen.find(getreg2()) == seen.end()) {
            prev = getreg2();
            seen.insert(getreg2());
          } else {
            reg[2] = prev;
            break;
          }
        }
      }
      ++reg[ir];
    }
    --reg[ir];
    // printreg();
  }

  uint64_t getreg0() const { return reg[0]; }
  uint64_t getreg2() const { return reg[2]; }
  uint64_t getreg3() const { return reg[3]; }
  uint64_t getreg4() const { return reg[4]; }
  int getinstructioncount() const { return instrs.size() - 1; }
};

int main() {

  program prog;

  std::string line;

  getline(std::cin, line);

  int ir = std::atoi(line.substr(4, 2).c_str());

  int pc_stop = -1;
  while (getline(std::cin, line)) {
    instruction i;
    i.setinstruction(line);
    prog.addinstruction(i);
    if (line.compare("eqrr 2 0 3") == 0)
      pc_stop = prog.getinstructioncount();
  }
  prog.run(ir, pc_stop, false, false);

  logger::get(logtype::logINFO) << "Part 1: " << prog.getreg2() << "\n";

  prog.reset(0);
  prog.run(ir, pc_stop, true, false);
  logger::get(logtype::logINFO) << "Part 2: " << prog.getreg2() << "\n";

  return 0;
}