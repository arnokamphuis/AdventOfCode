#include <vector>
#include <string>
#include <iostream>
#include <utility>
#include <algorithm>
#include <climits>

typedef std::pair<int,int> element;

inline int otherside(int conn, const element& e) { if (conn==e.first) return e.second; else return e.first; }

bool find_elements(int conn, const std::vector<element>& pipes, std::vector<element>& elements) {
    bool found = false;
    for (auto p: pipes) if ( (p.first==conn) or (p.second==conn) ) { elements.push_back(p); found = true; }
    return found;
}

void find_all_bridges(int conn, const std::vector<element>& pipes, std::vector<element> bridge, std::vector< std::vector<element> >& all_bridges) {
    std::vector<element> possible;

    if (find_elements(conn, pipes, possible)) { // still possible to expand bridge
        for (auto p: possible) {
            std::vector<element> newbridge = bridge;

            newbridge.push_back(p);
            std::vector<element> available = pipes;
            available.erase( std::find(available.begin(), available.end(), p) );

            find_all_bridges(otherside(conn,p), available, newbridge, all_bridges);
        }
    } else {
        all_bridges.push_back(bridge);
    }
}


int main() {
    std::vector<element> pipes;

    std::string line;
    while (getline(std::cin,line)){
        auto p = line.find("/");
        int from = std::stoi(line.substr(0,p));
        int to   = std::stoi(line.substr(p+1));
        pipes.push_back(std::make_pair(from,to));
    }

    std::vector<element> possible;
    std::vector<element> bridge;
    std::vector< std::vector<element> > all_bridges;
    std::vector< std::vector<element> > longest_bridges;

    find_all_bridges(0, pipes, bridge, all_bridges);

    int longest = INT_MIN;
    int maxbridge = INT_MIN;
    for (auto b: all_bridges) {
        int s = b.size();
        int l = 0; for (auto e: b) l+= e.first + e.second;
        if (s > longest) longest = s;

        if (l>maxbridge) maxbridge = l;
    }

    std::cout << "Part 1: " << maxbridge << std::endl;

    for (auto b: all_bridges) if (b.size()==longest) longest_bridges.push_back(b);
    int maxstrength = INT_MIN;
    for (auto b: longest_bridges) {
        int l = 0; for (auto e: b) l+= e.first + e.second;
        if (l>maxstrength) maxstrength = l;
    }

    std::cout << "Part 2: " << maxstrength << std::endl;
}