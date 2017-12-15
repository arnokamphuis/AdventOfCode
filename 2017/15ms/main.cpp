#include <iostream>
#include <bitset>
#include <cstdint>

bool compare(uint64_t a, uint64_t b) {
    bool res = true;
     std::bitset<16> bs_a(a);
     std::bitset<16> bs_b(b);
     for (int j=0;j<16;++j)
         if (bs_a[j] != bs_b[j])
             res = false;
     return res;
}

uint64_t generate(uint64_t prev, uint64_t seed, uint64_t multiple) {
    uint64_t temp = prev * seed;
    uint64_t candidate;
    
    do {
        candidate = ((prev * seed) % (uint64_t)2147483647);
        prev = candidate;
    } while ( (candidate % multiple) != 0);

    return candidate;
}

int main() {
    uint64_t val_a, val_b;
    
    // real puzzle values
    uint64_t init_a = 516;
    uint64_t init_b = 190;

    // test puzzle values
    // uint64_t init_a = 65;
    // uint64_t init_b = 8921;

    uint64_t found=0;
    uint64_t counter=0;

    val_a = init_a;
    val_b = init_b;

    while (counter<5000000) {
        val_a = generate(val_a, 16807, 4);
        val_b = generate(val_b, 48271, 8);
        //std::cout << val_a << "\t\t" << val_b << std::endl;
        if (compare(val_a,val_b))
            ++found;
        ++counter;
    }

    std::cout << "Answer: " << found << std::endl;
}