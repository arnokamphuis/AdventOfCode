#include <iostream>
#include <sstream>
#include <string>
#include <climits>

int distance(int pos[3]) {
    int delta = 0;
    delta += abs(pos[0]);
    delta += abs(pos[1]);
    delta += abs(pos[2]);
    return delta/2;
}

void update_pos(int pos[3], std::string dir) {
    if (dir.compare("nw")==0) { pos[0] += 0; pos[1] += 1; pos[2] -= 1; }
    if (dir.compare("n")==0)  { pos[0] += 1; pos[1] += 1; pos[2] += 0; }
    if (dir.compare("ne")==0) { pos[0] += 1; pos[1] += 0; pos[2] += 1; }
    if (dir.compare("sw")==0) { pos[0] -= 1; pos[1] += 0; pos[2] -= 1; }
    if (dir.compare("s")==0)  { pos[0] -= 1; pos[1] -= 1; pos[2] += 0; }
    if (dir.compare("se")==0) { pos[0] += 0; pos[1] -= 1; pos[2] += 1; }
}

int main() {
    int d = 0;
    int maxd = INT_MIN;
    int pos[3];
    pos[0] = pos[1] = pos[2] = 0;

    std::string token;
    while(std::getline(std::cin, token, ',')) {
        update_pos(pos,token);
        d = distance(pos);
        if (d>maxd) maxd=d;
    }

    std::cout << "Part 1: " << d << std::endl;
    std::cout << "Part 2: " << maxd << std::endl;

}