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
  uint16_t id;
  uint16_t A;
  uint16_t B;
  uint16_t C;
  uint16_t curreg[4];
  uint16_t resreg[4];
  uint16_t tmpreg[4];
  std::map<uint16_t, std::string> opset;

  instruction() {
    for (uint16_t i = 0; i < 4; ++i)
      curreg[i] = 0;
  }

  void execute(std::string line) {
    setinstruction(line);
    std::string opstr = opset[id];
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
    in >> id >> A >> B >> C;
  }

  void loadcurregister(std::string line) {
    std::stringstream in(line);
    for (uint16_t i = 0; i < 4; ++i)
      in >> curreg[i];
  }

  void loadresregister(std::string line) {
    std::stringstream in(line);
    for (uint16_t i = 0; i < 4; ++i)
      in >> resreg[i];
  }

  std::set<std::string> checkokay() {
    std::set<std::string> possibilities;
    preexec();
    if (addr())
      possibilities.insert("addr");

    preexec();
    if (addi())
      possibilities.insert("addi");

    preexec();
    if (mulr())
      possibilities.insert("mulr");

    preexec();
    if (muli())
      possibilities.insert("muli");

    preexec();
    if (banr())
      possibilities.insert("banr");

    preexec();
    if (bani())
      possibilities.insert("bani");

    preexec();
    if (borr())
      possibilities.insert("borr");

    preexec();
    if (bori())
      possibilities.insert("bori");

    preexec();
    if (setr())
      possibilities.insert("setr");

    preexec();
    if (seti())
      possibilities.insert("seti");

    preexec();
    if (gtir())
      possibilities.insert("gtir");

    preexec();
    if (gtri())
      possibilities.insert("gtri");

    preexec();
    if (gtrr())
      possibilities.insert("gtrr");

    preexec();
    if (eqir())
      possibilities.insert("eqir");

    preexec();
    if (eqri())
      possibilities.insert("eqri");

    preexec();
    if (eqrr())
      possibilities.insert("eqrr");

    return possibilities;
  }

  void preexec() {
    for (uint16_t i = 0; i < 4; ++i)
      tmpreg[i] = curreg[i];
  }

  void postexec() {
    for (uint16_t i = 0; i < 4; ++i)
      curreg[i] = tmpreg[i];
  }

  bool checkcorrectexec() {
    for (uint16_t i = 0; i < 4; ++i)
      if (tmpreg[i] != resreg[i])
        return false;
    return true;
  }

  bool addr() {
    tmpreg[C] = curreg[A] + curreg[B];
    return checkcorrectexec();
  }

  bool addi() {
    tmpreg[C] = curreg[A] + B;
    return checkcorrectexec();
  }

  bool mulr() {
    tmpreg[C] = curreg[A] * curreg[B];
    return checkcorrectexec();
  }

  bool muli() {
    tmpreg[C] = curreg[A] * B;
    return checkcorrectexec();
  }

  bool banr() {
    tmpreg[C] = curreg[A] & curreg[B];
    return checkcorrectexec();
  }

  bool bani() {
    tmpreg[C] = curreg[A] & B;
    return checkcorrectexec();
  }

  bool borr() {
    tmpreg[C] = curreg[A] | curreg[B];
    return checkcorrectexec();
  }

  bool bori() {
    tmpreg[C] = curreg[A] | B;
    return checkcorrectexec();
  }

  bool setr() {
    tmpreg[C] = curreg[A];
    return checkcorrectexec();
  }

  bool seti() {
    tmpreg[C] = A;
    return checkcorrectexec();
  }

  bool gtir() {
    tmpreg[C] = (A > curreg[B] ? 1 : 0);
    return checkcorrectexec();
  }

  bool gtri() {
    tmpreg[C] = (curreg[A] > B ? 1 : 0);
    return checkcorrectexec();
  }

  bool gtrr() {
    tmpreg[C] = (curreg[A] > curreg[B] ? 1 : 0);
    return checkcorrectexec();
  }

  bool eqir() {
    tmpreg[C] = (A == curreg[B] ? 1 : 0);
    return checkcorrectexec();
  }

  bool eqri() {
    tmpreg[C] = (curreg[A] == B ? 1 : 0);
    return checkcorrectexec();
  }

  bool eqrr() {
    tmpreg[C] = (curreg[A] == curreg[B] ? 1 : 0);
    return checkcorrectexec();
  }
};

class analyzer {
  std::vector<instruction> instructions;

  std::map<uint16_t, std::vector<std::set<std::string>>> possibilities;
  std::map<std::string, std::set<uint16_t>> reversepossibilities;

public:
  std::map<uint16_t, std::string> forward;
  std::map<std::string, uint16_t> backward;
  void addinstruction(instruction i) { instructions.push_back(i); }

  void constructpossibilities() {
    reversepossibilities["addr"] = {};
    reversepossibilities["addi"] = {};
    reversepossibilities["mulr"] = {};
    reversepossibilities["muli"] = {};
    reversepossibilities["banr"] = {};
    reversepossibilities["bani"] = {};
    reversepossibilities["borr"] = {};
    reversepossibilities["bori"] = {};
    reversepossibilities["setr"] = {};
    reversepossibilities["seti"] = {};
    reversepossibilities["gtri"] = {};
    reversepossibilities["gtir"] = {};
    reversepossibilities["gtrr"] = {};
    reversepossibilities["eqri"] = {};
    reversepossibilities["eqir"] = {};
    reversepossibilities["eqrr"] = {};

    for (auto i : instructions) {
      possibilities[i.id].push_back(i.checkokay());

      if (i.addr())
        reversepossibilities["addr"].insert(i.id);
      if (i.addi())
        reversepossibilities["addi"].insert(i.id);
      if (i.mulr())
        reversepossibilities["mulr"].insert(i.id);
      if (i.muli())
        reversepossibilities["muli"].insert(i.id);
      if (i.banr())
        reversepossibilities["banr"].insert(i.id);
      if (i.bani())
        reversepossibilities["bani"].insert(i.id);
      if (i.borr())
        reversepossibilities["borr"].insert(i.id);
      if (i.bori())
        reversepossibilities["bori"].insert(i.id);
      if (i.setr())
        reversepossibilities["setr"].insert(i.id);
      if (i.seti())
        reversepossibilities["seti"].insert(i.id);
      if (i.gtir())
        reversepossibilities["gtir"].insert(i.id);
      if (i.gtri())
        reversepossibilities["gtri"].insert(i.id);
      if (i.gtrr())
        reversepossibilities["gtrr"].insert(i.id);
      if (i.eqri())
        reversepossibilities["eqri"].insert(i.id);
      if (i.eqir())
        reversepossibilities["eqir"].insert(i.id);
      if (i.eqrr())
        reversepossibilities["eqrr"].insert(i.id);
    }
  }

  std::set<std::string> setintersection(std::set<std::string> a,
                                        std::set<std::string> b) {

    std::set<std::string> res;

    for (auto left : a)
      if (b.find(left) != b.end())
        res.insert(left);
    return res;
  }

  template <class T> void subtract(std::set<T> &s, T v) {
    auto found = s.find(v);
    if (found != s.end()) {
      s.erase(found);
    }
  }

  void determinetranslation() {
    while (reversepossibilities.size() > 0) {
      std::map<std::string, uint16_t> tbr;
      for (auto rp : reversepossibilities) {
        if (rp.second.size() == 1) {
          backward[rp.first] = *(rp.second.begin());
          tbr[rp.first] = *(rp.second.begin());
        }
      }

      for (auto name : tbr)
        reversepossibilities.erase(name.first);

      for (auto name : tbr) {
        for (auto &rp : reversepossibilities) {
          rp.second.erase(name.second);
        }
      }
    }

    for (auto b : backward) {
      forward[b.second] = b.first;
    }
  }
};

int main() {
  std::vector<std::string> inputprog;
  analyzer ana;
  uint16_t count = 0;
  std::string line;
  instruction instr;

  while (getline(std::cin, line)) {
    if (line.find("Before: [") != std::string::npos) {
      auto bb = line.find("[");
      auto be = line.find("]");
      std::string beforereg = line.substr(bb + 1, be - bb - 1);
      while (beforereg.find(',') != std::string::npos)
        beforereg.erase(beforereg.begin() + beforereg.find(','));

      getline(std::cin, line);
      std::string instructionline = line;

      getline(std::cin, line);
      bb = line.find("[");
      be = line.find("]");
      std::string afterreg = line.substr(bb + 1, be - bb - 1);
      while (afterreg.find(',') != std::string::npos)
        afterreg.erase(afterreg.begin() + afterreg.find(','));

      instr.setinstruction(instructionline);
      instr.loadcurregister(beforereg);
      instr.loadresregister(afterreg);
      ana.addinstruction(instr);
      if (instr.checkokay().size() >= 3)
        ++count;
    } else {
      if (line.length() > 0)
        inputprog.push_back(line);
    }
  }

  logger::get(logtype::logINFO) << "Part 1: " << count << "\n";

  instruction prog;
  ana.constructpossibilities();
  ana.determinetranslation();
  prog.opset = ana.forward;

  for (auto progline : inputprog)
    prog.execute(progline);

  logger::get(logtype::logINFO) << "Part 2: " << prog.curreg[0] << "\n";

  return 0;
}