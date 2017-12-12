#include <string>
#include <sstream>
#include <iostream>
#include <vector>
#include <iomanip>


int main() {
    
    std::string line;
    getline(std::cin, line);

    std::vector<int> lengths, orig_lengths;
    for (auto c: line)
        lengths.push_back((int)c);

    lengths.push_back(17);
    lengths.push_back(31);
    lengths.push_back(73);
    lengths.push_back(47);
    lengths.push_back(23);

    orig_lengths = lengths;

    int maximum_round = 64;
    int round_counter = 0;

    int skip_size=0;
    int listlength = 256;

    std::vector<int> list;
    for (int i=0; i<listlength; ++i) list.push_back(i);

    int position = 0;

    for (round_counter = 0; round_counter < maximum_round; ++round_counter) {
        lengths = orig_lengths;
        while (lengths.size()>0) {
            // get the length
            int length = *lengths.begin(); lengths.erase( lengths.begin() ); 

            std::vector<int> rev;
            for (int i=0; i<length; ++i)
                rev.insert( rev.begin(), list[ (position+i) % listlength ] );
            
            for (int i=0; i<length; ++i)
                list[ (position+i) % listlength ] = rev[i];
            
            position += length + skip_size;
            position = position % listlength;

            // increase skip size
            ++skip_size;
        }
    }

    std::vector<int> densehash;
    for (int i=0; i<16; i++) {
        int xorvalue = list[16*i];
        for (int j = 1; j<16; j++)
            xorvalue = xorvalue ^ list[16*i+j];
        densehash.push_back(xorvalue);
    }

    std::stringstream ss;
    for (auto d : densehash) ss << std::setfill('0') << std::setw(2) << std::hex << d;

    std::string hash = ss.str();
    std::cout << "Part 2: " << hash << std::endl;
}