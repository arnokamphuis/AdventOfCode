#include <iostream>
#include <map>
#include <math.h> 

int find_ring(int v) {
    int r=1;
    while (true) {
        if (v <= r*r)
            break;
        r+=2;
    }
    return r/2;
}

std::pair<int,int> get_coord(int cell) {
    int x, y;

    int ring = find_ring(cell);
    int ring_prev = ring-1;

    int w = 2 * ring_prev + 1;
    int total_prev = w*w;

    int delta = cell - total_prev;
    w = 2 * ring + 1;

    if (delta<=w-1) { 
        x = 0;
    } else if (delta<2*w-1) {
        x = 1;
    } else if (delta<3*w-2) {
        x = 2;
    } else {
        x = 3;
    }
    y = delta-1-(x*(w-1));

    int posx, posy;

    if (x==0) {
        posx = ring;
        posy = y - ring + 1;
    } else if (x==1) {
        posx = ring - y - 1;
        posy = ring;
    } else if (x==2) {
        posx = -ring;
        posy = ring - y - 1;
    } else {
        posx = y - ring + 1;
        posy = -ring;
    }

    std::pair<int, int> p;
    p.first = posx;
    p.second = posy;
    return p;
}

int main() {
    int bin;
    std::cin >> bin;

    float value = bin;

    std::pair<int, int> p = get_coord(value);
    int mandist = abs(p.first) + abs(p.second);
    std::cout << "Part 1: " << mandist <<std::endl;

    int i = 2;
    std::map< std::pair<int,int>, int > values;

    std::pair<int,int> ip = get_coord(1);
    values[ip] = 1;
    int answer = -1;
    while (true) {
        int sum=0;
        std::pair<int, int> cp = get_coord(i);

        std::pair<int, int> tl; tl.first = cp.first-1; tl.second = cp.second+1;
        std::pair<int, int>  l;  l.first = cp.first-1;  l.second = cp.second;
        std::pair<int, int> bl; bl.first = cp.first-1; bl.second = cp.second-1;

        std::pair<int, int> tr; tr.first = cp.first+1; tr.second = cp.second+1;
        std::pair<int, int>  r;  r.first = cp.first+1;  r.second = cp.second;
        std::pair<int, int> br; br.first = cp.first+1; br.second = cp.second-1;

        std::pair<int, int> t;   t.first = cp.first;    t.second = cp.second+1;
        std::pair<int, int> b;   b.first = cp.first;    b.second = cp.second-1;

        if (values.find(tl)!=values.end()) { sum+= values.find(tl)->second; }
        if (values.find(t )!=values.end()) { sum+= values.find(t )->second; }
        if (values.find(tr)!=values.end()) { sum+= values.find(tr)->second; }
        if (values.find(bl)!=values.end()) { sum+= values.find(bl)->second; }
        if (values.find(b )!=values.end()) { sum+= values.find(b )->second; }
        if (values.find(br)!=values.end()) { sum+= values.find(br)->second; }
        if (values.find( l)!=values.end()) { sum+= values.find( l)->second; }
        if (values.find( r)!=values.end()) { sum+= values.find( r)->second; }

        values[cp] = sum;

        if (sum > value) {answer = sum; break; }

        i++;
    }
    std::cout << "Part 2: " << answer << std::endl;


}