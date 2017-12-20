#include <iostream>
#include <string>
#include <map>
#include <set>
#include <utility>
#include <climits>
#include <vector>

int reduce_molecules(std::string medicine, const std::pair<std::string,std::string>& replacement, int steps, std::set<std::pair<std::string,int>>& molecules, const std::string& target) {
    int p=-1;

    std::string newmedicine;
    //std::cout << "============================================================" << std::endl;
    //std::cout << "Reducing molecule: " << medicine << std::endl;
    //std::cout << medicine <<",";
    while ((p = medicine.find(replacement.first,p+1))!=std::string::npos) {
        int s = replacement.first.size();
        newmedicine = medicine;
        newmedicine.replace(p,s,replacement.second);
        molecules.insert( std::make_pair(newmedicine, steps+1) );
        if (newmedicine.compare(target)==0) return steps+1;
    } 
    //std::cout << "============================================================" << std::endl;
    return -1;       
}

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
                //replacements.push_back(std::make_pair(line.substr(0,p),line.substr(p+4)));
                replacements.push_back(std::make_pair(line.substr(p+4),line.substr(0,p)));
            } else {
                medicine = line;
            }
        }
    }

    int totalsteps=-1;
    std::string target = "e";
//    std::string target = medicine;
    std::set<std::pair<std::string,int>> molecules;
    std::set<std::pair<std::string,int>>::iterator molit, nextmol;
    int minmollen = INT_MAX;
    molecules.insert( std::make_pair(medicine,0) );
    while (totalsteps<0) {

        minmollen = INT_MAX;
        for (molit=molecules.begin(); molit!=molecules.end(); ++molit) {
            if (molit->first.size() < minmollen) {
                minmollen = molit->first.size();
                nextmol = molit;
            }
        }

        std::string mol = nextmol->first;
        int steps = nextmol->second;

        molecules.erase(nextmol);

        for (auto repl: replacements) {
            totalsteps = reduce_molecules(mol,repl,steps,molecules, target);
            if (totalsteps>0) break;
        }
    }

    std::cout << "Part 2: " << totalsteps << std::endl;

}