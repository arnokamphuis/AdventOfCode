#include <iostream>
#include <bitset>
#include <cstdint>

bool compare(uint_fast32_t a, uint_fast32_t b) {
    bool res = true;
    std::bitset<16> bs_a(a);
    std::bitset<16> bs_b(b);
    for (int j=0;j<16;++j)
        if (bs_a[j] != bs_b[j])
            res = false;
    return res;
}

uint_fast32_t generate(uint_fast32_t prev, uint_fast32_t seed) {
    return ((prev * seed) % 2147483647);
}

int main() {
    uint_fast32_t val_a, val_b;
    
    uint_fast32_t init_a = 65;
    uint_fast32_t init_b = 8921;

    uint_fast32_t found=0;
    uint_fast32_t counter=0;

    val_a = init_a;
    val_b = init_b;

    while (counter<5) {
        val_a = generate(val_a, 16807);
        val_b = generate(val_b, 48271);
        std::cout << val_a << "\t" << val_b << std::endl;
        if (compare(val_a,val_b))
            ++found;
        ++counter;
    }

    std::cout << "Part 1: " << found << std::endl;
}