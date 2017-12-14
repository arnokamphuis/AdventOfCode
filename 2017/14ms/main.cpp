#include <string>
#include <sstream>
#include <iostream>
#include <vector>
#include <iomanip>
#include <bitset>
#include <algorithm>
#include <queue>
#include <utility>

std::string knot_hash(std::string line) {
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
    return ss.str();    
}

int label_cc(const std::vector< std::vector<int> >& usage, std::vector< std::vector<int> >& cc) {
    std::queue< std::pair<int, int> > q;
    int currentlabel = 1;
    int cx, cy;

    cx=cy=0;

    while(true) {
        //std::cout << "Currently working on " << cx << " " << cy << std::endl;
        if ( (usage[cx][cy]==1) && (cc[cx][cy]==0) ) {  //forground, not labeled
            //std::cout << "   ==> foreground, not labeled" << std::endl;
            cc[cx][cy] = currentlabel;
            q.push(std::make_pair(cx,cy));

            // pop it until empty
            while (q.size()>0) {
                std::pair<int,int> coord = q.front(); q.pop();

                //std::cout << "      ==> popped: " << coord.first << " " << coord.second << std::endl;

                int nx, ny;
                // top neighbour
                nx = coord.first; ny = coord.second-1;
                if ( (nx>=0) && (nx<usage.size()) && (ny>=0) && (ny<usage.size()) ) {
                    //std::cout << "             ----------> t " << nx <<  " " << ny <<  " " << usage[nx][ny] <<" " << cc[nx][ny] << std::endl;
                    if ( (usage[nx][ny]==1) && (cc[nx][ny]==0) )  { //foreground not labeled
                        cc[nx][ny] = currentlabel;
                        q.push(std::make_pair(nx,ny));
                    }
                }
                // left neighbour
                nx = coord.first-1; ny = coord.second;
                if ( (nx>=0) && (nx<usage.size()) && (ny>=0) && (ny<usage.size()) ) {
                    //std::cout << "             ----------> l " << nx <<  " " << ny <<  " " << usage[nx][ny] <<" " << cc[nx][ny] << std::endl;
                    if ( (usage[nx][ny]==1) && (cc[nx][ny]==0) )  { //foreground not labeled
                        cc[nx][ny] = currentlabel;
                        q.push(std::make_pair(nx,ny));
                    }
                }
                // right neighbour
                nx = coord.first+1; ny = coord.second;
                if ( (nx>=0) && (nx<usage.size()) && (ny>=0) && (ny<usage.size()) ) {
                    //std::cout << "             ----------> r " << nx <<  " " << ny <<  " " << usage[nx][ny] <<" " << cc[nx][ny] << std::endl;
                    if ( (usage[nx][ny]==1) && (cc[nx][ny]==0) )  { //foreground not labeled
                        cc[nx][ny] = currentlabel;
                        q.push(std::make_pair(nx,ny));
                    }
                }
                // bottom neighbour
                nx = coord.first; ny = coord.second+1;
                if ( (nx>=0) && (nx<usage.size()) && (ny>=0) && (ny<usage.size()) ) {
                    //std::cout << "             ----------> b " << nx <<  " " << ny <<  " " << usage[nx][ny] <<" " << cc[nx][ny] << std::endl;
                    if ( (usage[nx][ny]==1) && (cc[nx][ny]==0) )  { //foreground not labeled
                        cc[nx][ny] = currentlabel;
                        q.push(std::make_pair(nx,ny));
                    }
                }
            }
            ++currentlabel;
        }
        ++cy;
        if (cy==usage.size()) {
            cy=0;
            ++cx;
            if (cx==usage.size()) break; // end of all pixels reached
        }
    }

    return currentlabel-1;
}

std::string hash_to_bits(std::string hash) {
    std::string res = "";

    for (auto c: hash) {
        std::stringstream ss;
        ss << std::hex << c;
        unsigned int uc;
        ss >> uc;
        std::bitset<32> bits(uc);

        res += std::to_string((int)bits[3]);
        res += std::to_string((int)bits[2]);
        res += std::to_string((int)bits[1]);
        res += std::to_string((int)bits[0]);
    }
    return res;
}

int main() {

    std::string input;
    std::cin >> input;

    std::vector< std::vector<int> > usage;

    int counter = 0;
    for (int row=0;row<128;++row) {
        std::string key = input + "-" +  std::to_string(row);
        std::string hash = knot_hash(key);
        std::string bits = hash_to_bits(hash);

        std::vector<int> col;
        for(auto b: bits)
            col.push_back(b=='1');
        usage.push_back(col);

        counter += std::count(bits.begin(), bits.end(), '1');
    }
    std::cout << "Part 1: " << counter << std::endl;

    std::vector< std::vector<int> > cc = usage;
    for (auto& r:cc) for (auto& c:r) c=0; 

    std::cout << "Part 2: " << label_cc(usage,cc) << std::endl;
    return 0;
    std::cout << "---------------------------------------------------------------------------------------------------------------------" << std::endl;
    for (auto& r:usage) { for (auto& c:r) std::cout << c << "\t"; std::cout << std::endl; }; 
    std::cout << "---------------------------------------------------------------------------------------------------------------------" << std::endl;
    for (auto& r:cc) { for (auto& c:r) std::cout << c << "\t"; std::cout << std::endl; }; 
    std::cout << "---------------------------------------------------------------------------------------------------------------------" << std::endl;
    
}