#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <algorithm> 

int main() {  
    
    int count = 0;
    int lc = 0;

    while (true) {
        lc++;
        std::cout << "Line " << lc <<std::endl;
        bool error = false;
        std::vector<std::string> line;
        std::string t;
        getline(std::cin, t);

        std::istringstream iss(t);
        std::string word;
        while(iss >> word) {
            std::cout << word << std::endl;
            if ( std::find(line.begin(), line.end(), word) != line.end() ) {
                std::cout << "Not found: " << word << std::endl;
            } else {
                error = true;
                break;
            }
            line.push_back(word);
        }
        if (!error) count++;
        if(std::cin.eof())
            break;
    }

    std::cout << "Part 1: " << count << std::endl;
}