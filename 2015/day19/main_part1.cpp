#include <iostream>
#include <string>
#include <map>
#include <set>
#include <utility>
#include <vector>


void generate_molecules(std::string medicine, const std::pair<std::string,std::string>& replacement, std::set<std::string>& molecules) {
    int p=-1;
    std::string newmedicine;
    while ((p = medicine.find(replacement.first,p+1))!=std::string::npos) {
        std::cout << "p: " << p << std::endl;
        int s = replacement.first.size();
        newmedicine = medicine;
        newmedicine.replace(p,s,replacement.second);
        molecules.insert(newmedicine);
    }
}

int main() {

    std::string line;
    std::vector< std::pair<std::string,std::string> > replacements;
    std::string medicine;

    int replace_line = 1;
    while (getline(std::cin,line)) {
        if (line.length()==0) replace_line = 0;
        else {
            if (replace_line==1) {
                std::size_t p = line.find(" => ");
                replacements.push_back(std::make_pair(line.substr(0,p),line.substr(p+4)));
            } else {
                medicine = line;
            }
        }
    }

    std::set<std::string> molecules;
    for (auto repl: replacements) {
        std::cout << "Working on " << repl.first << " to " << repl.second << std::endl;
        generate_molecules(medicine, repl, molecules);
    }

    std::cout << "Part 1: " << molecules.size() << std::endl;

}