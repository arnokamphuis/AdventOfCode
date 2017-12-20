#include <iostream>
#include <string>
#include <vector>

typedef std::vector<std::vector<char> > maze_type;

class maze {
protected:
    int step_counter;
    int cx, cy, dx, dy;
    maze_type m;
public:

    maze(){}

    int getsteps() { return step_counter; }

    void addline(std::string line) {
        std::vector<char> l;
        for (auto c: line) {
            l.push_back(c);
        }
        m.push_back(l);
    }

    void init() {
        cx = 0;
        for (auto c: m[0]) {
            if (c=='|') break;
            ++cx; 
        }
        cy = 0;
        dx = 0;
        dy = 1;
        step_counter=0;
    }

    void print() {
        for (auto l: m) { for (auto c: l) std::cout << c; std::cout << std::endl; }
    }

    void determine_direction() {
        if ( dx==0 ) {
            if ((cx-1)>=0) if (m[cy][cx-1]!=' ')          { dx=-1; dy=0; }
            if ((cx+1)<m[0].size()) if (m[cy][cx+1]!=' ') { dx= 1; dy=0; }
        } else {
            if ((cy-1)>=0) if (m[cy-1][cx]!=' ')       { dx=0; dy=-1; }
            if ((cy+1)<m.size()) if (m[cy+1][cx]!=' ') { dx=0; dy= 1; }
        }
        cx+=dx; cy+=dy;
        ++step_counter;
    }

    std::vector<char> walk() {
        std::vector<char> res;
        while (m[cy][cx]!=' ') {
            while ( (m[cy][cx]!=' ') and (m[cy][cx]!='+') ) {
                char c = m[cy][cx];
                //std::cout << c;
                if ( (c!='|') and (c!='-')) res.push_back(c);
                cx+=dx;
                cy+=dy;
                ++step_counter;
                //std::cout << "(" << cx << "," << cy << ")";
            }
            if (m[cy][cx]==' ') break;
            determine_direction();
        }
        return res;
    }
};

int main() {

    maze* themaze = new maze();

    std::string line;
    while (getline(std::cin,line)) themaze->addline(line);

    themaze->init();

    //themaze->print();

    std::vector<char> res = themaze->walk();

    std::cout << "Part 1: ";
    for (auto c: res) std::cout << c;
    std::cout << std::endl;

    std::cout << "Part 2: " << themaze->getsteps() << std::endl;
}