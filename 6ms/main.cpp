#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>

bool find_config(std::vector<std::string> confs, std::string conf) {
    return ( std::find(confs.begin(), confs.end(), conf)!=confs.end() );
}

std::string getmemstr(int memory[16], int size) {
    std::string res = "";
    for (int i=0; i<size; i++) {
        res += std::to_string(memory[i]) + std::string("-");
    }
    return res;
}

int find_largest(int memory[16], int size) {
    int j = 0;
    int m = memory[0];
    for (int i=1; i<size; i++) 
        if (memory[i]>m) { j=i; m=memory[i]; }
    return j;
}

void reallocate(int memory[16], int size, int bank) {
    int s = memory[bank];
    memory[bank] = 0;
    while (s>0) {
        bank++;
        memory[bank%size]++;
        s--;
    }
}


int main() {
    std::map<std::string, int> config_time;

    int b;
    int blockcount = 0;
    int memory[16];

    std::vector<std::string> configs;

    while (std::cin >> b) {
        memory[blockcount] = b;
        blockcount++;
    }

    int counter = 0;
    std::string c;
    while (!find_config(configs,getmemstr(memory,blockcount))) {
        c = getmemstr(memory,blockcount);
        config_time[c] = counter;
        configs.push_back(c);
        int l = find_largest(memory, blockcount);
        reallocate(memory, blockcount, l);
        counter++;
    }
    c = getmemstr(memory,blockcount);

    std::cout << "Part 1: " << counter << std::endl;
    std::cout << "Part 2: " << counter-config_time[c] << std::endl;

}