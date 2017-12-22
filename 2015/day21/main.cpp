#include <iostream>
#include <string>
#include <map>
#include <utility>
#include <climits>

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

class specific_items {
public:
    std::map<std::string, std::pair<int,int> >::iterator weapon;
    std::map<std::string, std::pair<int,int> >::iterator arm;
    std::map<std::string, std::pair<int,int> >::iterator ring1;
    std::map<std::string, std::pair<int,int> >::iterator ring2;    
};

class items {
    public:
    bool wait_armor, wait_ring1, wait_ring2;

    specific_items it;

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

        wait_armor = false;
        wait_ring1 = false;
        wait_ring2 = false;
    }

    int cost() {
        int c = weapon->second.first;
        if (arm!=armor.end()) { c += arm->second.first; }
        if (ring1!=rings.end()) { c += ring1->second.first; }
        if (ring2!=rings.end()) { c += ring2->second.first; }
        return c;
    }

    void calculate(int& damagevalue, int& armorvalue) {
        damagevalue = 0; armorvalue = 0;

        damagevalue = weapon->second.second;

        if (arm!=armor.end())    armorvalue = arm->second.second;
        if (ring1!=rings.end()) { 
            if ( ring1->first.find("damage")!=std::string::npos ) 
                damagevalue += ring1->second.second;
            else 
                armorvalue += ring1->second.second;
        }
        if (ring2!=rings.end()) { 
            if ( ring2->first.find("damage")!=std::string::npos ) 
                damagevalue += ring2->second.second;
            else 
                armorvalue += ring2->second.second;
        }
    }

    bool nextweapon() {
        ++weapon;

        if (weapon==weapons.end()) {
            weapon = weapons.begin();
            return false;
        }
        return true;
    }
    
    bool nextarmor() {
        if (wait_armor and arm==armor.end() ) {
            wait_armor = false;
            return false;
        } else if (arm==armor.end()) {
            arm = armor.begin();
        } else {
            ++arm;
            if ( arm==armor.end() ) wait_armor = true;
        }
        return true;
    }
    
    bool nextring1() {
        if (wait_ring1 and ring1==rings.end() ) {
            wait_ring1 = false;
            return false;
        } else if (ring1==rings.end()) {
            ring1 = rings.begin();
        } else {
            ++ring1;
            if ( ring1==rings.end() ) wait_ring1 = true;
        }
        return true;
    }

    bool nextring2() {
        if (wait_ring2 and ring2==rings.end() ) {
            wait_ring2 = false;
            return false;
        } else if (ring2==rings.end()) {
            ring2 = rings.begin();
        } else {
            ++ring2;
            if ( ring2==rings.end() ) wait_ring2 = true;
        }
        return true;
    }

    bool next() {
        if (!nextweapon())
            if (!nextarmor())
                if (!nextring1())
                    if (!nextring2())
                        return false;
        return true;
    }

    void print(bool current) {
        std::cout <<"=========================" << std::endl;
        if (current) {
            std::cout <<"Cost: " << cost() << std::endl;
            std::cout << "Weapon: " << weapon->first << std::endl;
            if (arm!=armor.end()) std::cout << "Armor: " << arm->first << std::endl;
            if (ring1!=rings.end()) std::cout << "Ring 1: " << ring1->first << std::endl;
            if (ring2!=rings.end()) std::cout << "Ring 2: " << ring2->first << std::endl;
        } else {
            std::cout << "Weapon: " << it.weapon->first << std::endl;
            if (it.arm!=armor.end()) std::cout << "Armor: " << it.arm->first << std::endl;
            if (it.ring1!=rings.end()) std::cout << "Ring 1: " << it.ring1->first << std::endl;
            if (it.ring2!=rings.end()) std::cout << "Ring 2: " << it.ring2->first << std::endl;            
        }
        std::cout <<"=========================" << std::endl;
    }

    void store() {
        it.weapon = weapon;
        it.arm    = arm;
        it.ring1  = ring1;
        it.ring2  = ring2;
    }
};

int main() {
    player* boss = new player("boss");

    boss->hitpoints = 100;
    boss->damage    =   8;
    boss->armor     =   2;

    player* me = new player("player");

    items* inventory = new items;

    int mingold = INT_MAX;
    int mostgold = INT_MIN;

    int teller=0;

    do {
        ++teller;
        inventory->calculate(me->damage, me->armor);

        player* cp = me;
        player* op = boss;

        me->hitpoints   = 100;
        boss->hitpoints = 100;

        while(cp->hitpoints>0) {
            cp->attack(op);

            // switch turns
            player* tmp = cp; cp = op; op = tmp;
        }

        if (op==me)
            if (inventory->cost()<mingold) 
                mingold = inventory->cost();

        if (op==boss) {
            if (inventory->cost()>mostgold) {
                mostgold = inventory->cost();
                inventory->store();
            }
        }

    } while (inventory->next());


    std::cout << "Part 1: " << mingold << std::endl;
    std::cout << "Part 2: " << mostgold << std::endl;
}