#include <iostream>
#include <string>
#include <sstream>
#include <map>
#include <set>
#include <vector>


void loadconnections(std::map< int, std::vector<int> >& connections) {
    
    std::string line;
    while(std::getline(std::cin, line)) {

        std::size_t p = line.find(" <-> ");
        std::string from = line.substr(0,p);
        std::string to   = line.substr(p+5);

        int from_id = std::stoi(from);

        std::istringstream ss(to);
        char split_char = ',';
        std::vector<std::string> tokens;
        for (std::string each; std::getline(ss,each, split_char); tokens.push_back(each));

        for (auto t: tokens)
            connections[from_id].push_back( std::stoi(t) );
    }
    
}

std::set<int> getcomponent(int startid, std::map< int, std::vector<int> >& connections) {
    std::set< int > component;
    std::set<int> open, visited;

    open.insert(startid);

    while (open.size()>0) {
        int cid = *(open.begin());
        if ( visited.count(cid)==0 ) {
            visited.insert(cid);
            for ( auto child: connections[cid] ) {
                if (visited.count(child)==0) {                    
                    open.insert(child);
                }
            }
            if (component.count(cid)==0) component.insert(cid);
            open.erase( open.find(cid) );
        }
    }
    return component;
}

int main() {

    std::map< int, std::vector<int> > connections;
    std::map< int, std::vector<int> > stillopen;
    std::set< int > component;

    loadconnections(connections);
    stillopen = connections;

    int component_count=1;
    component = getcomponent(0, connections);
    std::cout << "Part 1: " << component.size() << std::endl;

    for (auto c: component) stillopen.erase(stillopen.find(c));

    while (stillopen.size()>0) {
        int startid = stillopen.begin()->first;
        component = getcomponent(startid, stillopen);
        component_count++;
        for (auto c: component) stillopen.erase(stillopen.find(c));
    }
    std::cout << "Part 2: " << component_count << std::endl;

}