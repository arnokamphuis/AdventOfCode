#include <iostream>
#include <string>
#include <map>
#include <utility>

class player {
    public:
    std::string name;
    int hitpoints;
    int damage;
    int armor;

    player(const std::string& n) : name(n) {
        hitpoints = 100;
        damage = 0;
        armor = 0;
    }

    void attack(player* other) {
        int battlepoints = 0;

        int d = damage - other->armor;
        if (d<1) d = 1;

        other->hitpoints -= d;        
    }
};

class items {
    public:
    std::map<std::string, std::pair<int,int> > weapons;
    std::map<std::string, std::pair<int,int> > armor;
    std::map<std::string, std::pair<int,int> > rings;

    std::map<std::string, std::pair<int,int> >::iterator weapon;
    std::map<std::string, std::pair<int,int> >::iterator arm;
    std::map<std::string, std::pair<int,int> >::iterator ring1;
    std::map<std::string, std::pair<int,int> >::iterator ring2;

    items() {
        weapons["dagger"]     = std::make_pair(8,4);
        weapons["shortswood"] = std::make_pair(10,5);
        weapons["warhammer"]  = std::make_pair(25,6);
        weapons["longsword"]  = std::make_pair(40,7);
        weapons["greataxe"]   = std::make_pair(74,8);

        armor["leather"]    = std::make_pair(13,1);
        armor["chainmail"]  = std::make_pair(31,2);
        armor["splintmail"] = std::make_pair(53,3);
        armor["bandedmail"] = std::make_pair(75,4);
        armor["platemail"]  = std::make_pair(102,5);

        rings["damage1"] = std::make_pair(25,1);
        rings["damage2"] = std::make_pair(50,2);
        rings["damage3"] = std::make_pair(100,3);
        rings["defense1"] = std::make_pair(20,1);
        rings["defense2"] = std::make_pair(40,2);
        rings["defense3"] = std::make_pair(80,3);

        weapon = weapons.begin();
        arm = armor.end();
        ring1 = rings.end();
        ring2 = rings.end();
    }

    int cost() {
        int c = weapon->second.first;
        if (arm!=armor.end()) { c += arm->second.first; }
        if (ring1!=rings.end()) { c += ring1->second.first; }
        if (ring2!=rings.end()) { c += ring2->second.first; }
        return c;
    }

    void calculate(int& damagevalue, int& armorvalue) {
        damagevalue = weapon->second.second;
        if (arm!=armor.end()) armorvalue = arm->second.second; else armorvalue = 0;
        if (ring1!=rings.end()) {
            if (ring1->first.find("damage")!=std::string::npos) damagevalue += ring1->second.second;
            else armorvalue += ring1->second.second;
        }
        if (ring2!=rings.end()) {
            if (ring2->first.find("damage")!=std::string::npos) damagevalue += ring2->second.second;
            else armorvalue += ring2->second.second;
        }
    }

    bool next() {
        ++weapon;

        if (weapon==weapons.end()) {
            weapon = weapons.begin();

            if (arm==armor.end()) arm = armor.begin();
            else {
                ++arm;

                if (arm==armor.end()) return false;
            }
        }
        return true;
    }
};

int main() {
    player* boss = new player("boss");

    boss->hitpoints = 100;
    boss->damage    =   8;
    boss->armor     =   2;

    player* me = new player("player");

    items* inventory = new items;

    do {
        inventory->calculate(me->damage, me->armor);

        player* cp = me;
        player* op = boss;

        while(cp->hitpoints>0) {
            cp->attack(op);

            // switch turns
            player* tmp = cp; cp = op; op = tmp;
        }

        std::cout << op->name << " wins at " << inventory->cost() << " gold" << std::endl;
    } while (inventory->next());

}