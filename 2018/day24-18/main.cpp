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

enum damagetype {
  RADIATION = 0,
  BLUDGENING = 1,
  FIRE = 2,
  COLD = 3,
  SLASHING = 4,
  ERROR = -1
};
enum faction { IMMUNESYSTEM, INFECTION, NONE };
enum defensetype { WEAK, IMMUNE };

class group {
  int id;
  faction side;
  int size;
  int hitpoints;
  int damage;
  damagetype dt;
  int initiative;

  std::map<defensetype, std::vector<damagetype>> defenses;

public:
  group(int nr, faction f, int s, int hp, int d, damagetype t, int i)
      : id(nr), side(f), size(s), hitpoints(hp), damage(d), dt(t),
        initiative(i) {}

  group(const group &g) {
    id = g.id;
    side = g.side;
    size = g.size;
    hitpoints = g.hitpoints;
    damage = g.damage;
    dt = g.dt;
    initiative = g.initiative;

    for (auto d : g.defenses) {
      for (auto dt : d.second) {
        defenses[d.first].push_back(dt);
      }
    }
    // std::cout << "Group copy constructor" << std::endl;
  }

  friend std::ostream &operator<<(std::ostream &os, const group *g) {
    os << "Group " << g->id << std::endl;
    os << "  faction: " << g->side << std::endl;
    os << "  size: " << g->size << std::endl;
    os << "  hp: " << g->hitpoints << std::endl;
    os << "  damagetype: " << g->dt << std::endl;
    os << "  damage: " << g->damage << std::endl;
    os << "  initiative: " << g->initiative << std::endl;
    /*
        for (auto d : g->defenses) {
          os << "     " << d.first << std::endl;
          for (auto dt : d.second)
            os << "       " << dt << std::endl;
        }*/
    return os;
  }

  bool stillalive() const { return size > 0; }

  const faction &getfaction() const { return side; }
  const faction getopposition() const {
    if (side == IMMUNESYSTEM)
      return INFECTION;
    else
      return IMMUNESYSTEM;
  }
  const damagetype &getdamagetype() const { return dt; }
  const int &getdamage() const { return damage; }
  const int &getsize() const { return size; }
  int effectivepower() const { return size * damage; }
  int getinitiative() const { return initiative; }
  const int &getid() const { return id; }

  void addboost(int boost) {
    damage += boost;
    // std::cout << "Damage is : " << damage << std::endl;
  }

  void attack(group *target) { target->dodamage(dt, effectivepower()); }

  void dodamage(damagetype dt, int value) {
    int pd = calculatepotentialdamage(dt, value);
    int unitseffected = std::min(pd / hitpoints, size);
    size -= unitseffected;
    // std::cout << "Group " << id << " received " << pd << " damage and lost "
    //           << unitseffected << " units" << std::endl;
  }

  void adddefensetype(defensetype dt, damagetype t) {
    defenses[dt].push_back(t);
  }

  int calculatepotentialdamage(damagetype attacktype, int value) {
    int pd = value;
    if (std::find(defenses[IMMUNE].begin(), defenses[IMMUNE].end(),
                  attacktype) != defenses[IMMUNE].end())
      pd = 0;
    else if (std::find(defenses[WEAK].begin(), defenses[WEAK].end(),
                       attacktype) != defenses[WEAK].end())
      pd *= 2;

    return pd;
  }
};

class armies {
  std::vector<group *> groups;
  std::map<faction, std::vector<group *>> factions;
  faction winning;

  std::map<group *, group *> targets;

  void cleartargets() { targets.clear(); }

  void removegroup(group *g) {
    cleartargets();
    faction f = g->getfaction();
    factions[f].erase(std::remove(factions[f].begin(), factions[f].end(), g),
                      factions[f].end());
    groups.erase(std::remove(groups.begin(), groups.end(), g), groups.end());
    delete g;
  }

  void cleanupgroups() {
    std::vector<group *> gtbr;
    std::for_each(groups.begin(), groups.end(), [&gtbr](group *g) {
      if (!g->stillalive())
        gtbr.push_back(g);
    });

    for (auto g : gtbr) {
      removegroup(g);
    }
  }

  bool factionwon() {
    for (auto f : factions) {
      // std::cout << "Faction " << f.first << " has " << f.second.size()
      //           << " groups remaining. " << std::endl;
      if (f.second.size() == 0) {
        if (f.first == IMMUNESYSTEM) {
          winning = INFECTION;
        } else {
          winning = IMMUNESYSTEM;
        }

        return true;
      }
    }
    return false;
  }

  void targetselection() {
    cleartargets();
    std::sort(groups.begin(), groups.end(),
              [](const group *g1, const group *g2) {
                int ep1 = g1->effectivepower();
                int ep2 = g2->effectivepower();
                if (ep1 > ep2)
                  return true;
                if (ep1 == ep2)
                  return g1->getinitiative() > g2->getinitiative();
                return false;
              });
    // std::cout << " TS(): groups sorted." << std::endl;

    std::vector<group *> chosen;
    for (auto g : groups) {
      if (g->stillalive()) {

        // std::cout << " TS(): handling group " << g->getid() << std::endl;

        int d = g->effectivepower();
        damagetype dt = g->getdamagetype();

        // std::cout << " TS(): sorting opposition" << std::endl;
        std::sort(factions[g->getopposition()].begin(),
                  factions[g->getopposition()].end(),
                  [d, dt](group *g1, group *g2) {
                    int pd1 = g1->calculatepotentialdamage(dt, d);
                    int pd2 = g2->calculatepotentialdamage(dt, d);
                    int ep1 = g1->effectivepower();
                    int ep2 = g2->effectivepower();
                    int in1 = g1->getinitiative();
                    int in2 = g2->getinitiative();

                    if (pd1 > pd2)
                      return true;
                    else if ((pd1 == pd2) && (ep1 > ep2))
                      return true;
                    else if ((pd1 == pd2) && (ep1 == ep2) && (in1 > in2))
                      return true;
                    return false;
                  });

        // std::cout << " TS(): finding opposition" << std::endl;
        group *pt = nullptr;
        for (auto ag : factions[g->getopposition()]) {
          if (ag->stillalive() && (ag->calculatepotentialdamage(dt, d) > 0)) {
            if (std::find(chosen.begin(), chosen.end(), ag) == chosen.end()) {
              pt = ag;
              chosen.push_back(ag);
              break;
            }
          }
        }

        // std::cout << " TS(): found potential opposition" << std::endl;
        if (pt != nullptr) {
          targets[g] = pt;
          // std::cout << "Group " << g->getid() << " attacks " << pt->getid()
          //           << std::endl;
        }
      }
    }
  }

  void attack() {
    std::sort(groups.begin(), groups.end(), [](group *g1, group *g2) {
      return (g1->getinitiative() > g2->getinitiative());
    });

    for (auto ag : groups) {
      if (ag->stillalive()) {
        // std::cout << "Attack by " << ag->getid() << std::endl;
        if (targets.find(ag) != targets.end()) {
          group *tg = targets[ag];
          // std::cout << "   against " << tg->getid() << std::endl;
          if (tg->stillalive())
            ag->attack(tg);
        }
      }
    }
  }

public:
  armies() : winning(NONE) {}

  armies(const armies &copy) : winning(NONE) {
    for (auto g : copy.groups) {
      group *ng = new group(*g);
      groups.push_back(ng);
      factions[ng->getfaction()].push_back(ng);
    }
  }

  int armysize() const { return groups.size(); }

  void addgroup(group *g) {
    groups.push_back(g);
    factions[g->getfaction()].push_back(g);
  }

  bool battle() {
    // std::cout << "=====================================================\n";
    int begin_as = unitsremaining();
    // std::cout << "Going to select targets" << std::endl;
    targetselection();
    // std::cout << "Going to attack targets" << std::endl;
    attack();
    // std::cout << "Going to clean up groups" << std::endl;
    cleanupgroups();
    // std::cout << "After battle there are " << groups.size()
    //           << " groups active. " << std::endl;

    // for (auto g : groups) {
    //   std::cout << "----------------\n";
    //   std::cout << g << std::endl;
    //   std::cout << "----------------\n";
    // }

    // std::cout << "=====================================================\n";
    int end_as = unitsremaining();

    if (begin_as == end_as)
      return false;

    return !factionwon();
  }

  const faction &winningfaction() const { return winning; }

  void addboost(int boost) {
    for (auto g : factions[IMMUNESYSTEM]) {
      // std::cout << "adding boost " << boost << std::endl;
      g->addboost(boost);
    }
  }

  int unitsremaining() const {
    return std::accumulate(
        groups.begin(), groups.end(), 0,
        [](const int &s, const group *g) { return s + g->getsize(); });
  }
};

std::vector<std::string> split(const char *str, char c = ' ') {
  std::vector<std::string> result;

  do {
    const char *begin = str;

    while (*str != c && *str)
      str++;

    result.push_back(std::string(begin, str));
  } while (0 != *str++);

  return result;
}

std::vector<std::string> subs(std::string line, std::string tokens) {
  char c1 = tokens[0];
  char c2 = tokens[1];
  auto pos1 = line.find(c1);
  auto pos2 = line.find(c2);
  std::vector<std::string> result;
  result.push_back(line.substr(0, pos1));
  result.push_back(line.substr(pos1 + 1, pos2 - pos1 - 1));
  result.push_back(line.substr(pos2 + 1, line.length()));
  return result;
}

damagetype strtodt(std::string s) {
  if (s.compare("radiation") == 0)
    return RADIATION;
  if (s.compare("bludgeoning") == 0)
    return BLUDGENING;
  if (s.compare("fire") == 0)
    return FIRE;
  if (s.compare("cold") == 0)
    return COLD;
  if (s.compare("slashing") == 0)
    return SLASHING;

  return ERROR;
}

std::string removescc(std::string s) {
  auto pos = s.find(';');
  if (pos != std::string::npos)
    return s.substr(0, pos);
  pos = s.find(',');
  if (pos != std::string::npos)
    return s.substr(0, pos);
  return s;
}

int main() {
  armies sickness;

  faction currentfaction;
  std::string line;
  while (getline(std::cin, line)) {
    if (line.find("Immune System:") != std::string::npos) {
      currentfaction = IMMUNESYSTEM;
      continue;
    }
    if (line.find("Infection:") != std::string::npos) {
      currentfaction = INFECTION;
      continue;
    }

    if (line.find("units each with") != std::string::npos) {
      std::vector<std::string> parts;

      if (line.find('(') == std::string::npos) {
        auto hppos = line.find("hit points") + 10;
        parts.push_back(line.substr(0, hppos));
        parts.push_back("");
        parts.push_back(line.substr(hppos, line.length()));
      } else {
        parts = subs(line, "()");
      }

      std::vector<std::string> tokens1 = split(parts[0].c_str(), ' ');
      int size = std::atoi(tokens1[0].c_str());
      int hitpoints = std::atoi(tokens1[4].c_str());

      std::vector<std::string> tokens3 = split(parts[2].c_str(), ' ');
      int damage = std::atoi(tokens3[6].c_str());
      std::string typedamage = tokens3[7];
      int initiative = std::atoi(tokens3[11].c_str());

      damagetype dt = strtodt(typedamage);
      if (dt == ERROR)
        std::cerr << "ERRRORRRR!!!!" << std::endl;

      group *g = new group(sickness.armysize() + 1, currentfaction, size,
                           hitpoints, damage, dt, initiative);

      if (parts[1].size() > 0) {
        std::vector<std::string> defenses = split(parts[1].c_str(), ' ');
        defensetype deft;
        for (auto d : defenses) {
          if (d.compare("weak") == 0)
            deft = WEAK;
          else if (d.compare("immune") == 0)
            deft = IMMUNE;
          else if (d.compare("to") == 0) {
          } else {
            dt = strtodt(removescc(d));
            g->adddefensetype(deft, dt);
          }
        }
      }

      sickness.addgroup(g);
      // std::cout << g << std::endl;
    }
  }

  armies startup(sickness);

  while (sickness.battle()) {
  }

  logger::get(logtype::logINFO)
      << "Part 1: " << sickness.unitsremaining() << std::endl;

  int minboost = 0;
  int maxboost = 1;

  while (true) {

    armies notsick(startup);
    notsick.addboost(maxboost);

    while (notsick.battle()) {
    }

    if (notsick.winningfaction() == IMMUNESYSTEM)
      break;

    maxboost *= 2;
  }

  while ((maxboost - minboost) != 1) {
    int middle = (maxboost + minboost) / 2;

    armies notsick(startup);
    notsick.addboost(middle);
    while (notsick.battle()) {
    }
    if (notsick.winningfaction() == IMMUNESYSTEM) {
      maxboost = middle;
    } else {
      minboost = middle;
    }
  }

  armies notsick(startup);
  notsick.addboost(maxboost);
  while (notsick.battle()) {
  }
  logger::get(logtype::logINFO)
      << "Part 2: " << notsick.unitsremaining() << std::endl;

  return 0;
}