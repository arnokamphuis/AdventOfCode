#include <string>
#include <map>
#include <iostream>

class tile {
    int size;
    std::string leds;

    bool check_flipped(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                if (leds[j*size+fi]!=desc[j*size+i]) return false;
            }
        }
        return true;
    }

    bool check_normal(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            for (int j=0; j<size; ++j) {
                if (leds[j*size+i]!=desc[j*size+i]) return false;
            }
        }
        return true;
    }

    bool check_lr(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[fi*size+j]!=desc[j*size+i]) return false;
            }
        }
        return true;        
    }

    bool check_rr(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[i*size+fj]!=desc[j*size+i]) return false;
            }
        }
        return true;        
    }

    bool check_fr(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[fj*size+fi]!=desc[j*size+i]) return false;
            }
        }
        return true;        
    }

    bool check(const std::string& desc) {
        if (check_normal(desc)) return true;
        if (check_lr(desc)) return true;
        if (check_fr(desc)) return true;
        if (check_rr(desc)) return true;
        if (check_flipped(desc)) return true;
        return false;
    }

    void change_into(const std::string& desc) {
        ++size;
        leds = desc;
    }

    void update_leds(const std::string& desc) {
        leds = "";
        for (auto c: desc) if (c!='/') leds += c;
    }

public:
    tile(int s, const std::string& desc) : size(s) {
        update_leds(desc);
    }

    void evolve(const std::map<std::string, std::string>& rules) {
        for (auto r: rules)
            if(r.first.size() == size)
                if (check(r.first))
                    change_into(r.second);
    }

    char led(int i, int j) {
        return leds[i+size*j];
    }

};


int main() {
    tile* first = new tile(3, ".#./..#/###");

    for (int i=0;i<3;++i) {
        for (int j=0;j<3;++j) {
            std::cout << first->led(i,j);
        }
        std::cout << std::endl;
    }
}