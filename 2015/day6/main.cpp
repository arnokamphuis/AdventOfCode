#include <iostream>
#include <sstream>
#include <vector>
#include <map>
#include <climits>

typedef int lighttype;
//typedef bool lighttype;

void toggle(int x1, int y1, int x2, int y2, std::vector<std::vector<lighttype> >& lights) {
    for (int i=x1; i<=x2; ++i) for (int j=y1; j<=y2; ++j) { 
        //lights[i][j] = !lights[i][j];
        lights[i][j]+=2;
    }
}

void turn_on(int x1, int y1, int x2, int y2, std::vector<std::vector<lighttype> >& lights) {
    for (int i=x1; i<=x2; ++i) for (int j=y1; j<=y2; ++j) {
        //lights[i][j] = true;
        lights[i][j]+=1;
    }
}

void turn_off(int x1, int y1, int x2, int y2, std::vector<std::vector<lighttype> >& lights) {
    for (int i=x1; i<=x2; ++i) for (int j=y1; j<=y2; ++j) {
        //lights[i][j] = false;
        lights[i][j]-=1;
        if (lights[i][j]<0) lights[i][j]=0;
    }
}

int main() {

    std::vector< std::vector<lighttype> > lights;
    for (int i=0;i<1000;++i) {
        std::vector<lighttype> v;
        for (int j=0;j<1000;++j) v.push_back((lighttype)0);
        lights.push_back(v);
    }

    std::string line;
    while (getline(std::cin,line)) {
        std::istringstream ss(line);

        std::string cmd;
        ss >> cmd;
        if (cmd.compare("turn")==0)
            ss >> cmd;
        
        int x1, x2, y1, y2;
        std::string tmp1, tmp2, tmp3;
        ss >> tmp1 >> tmp2 >> tmp3;

        std::istringstream cs1(tmp1);
        std::istringstream cs2(tmp3);
        std::string each;
        getline(cs1,each,','); x1 = std::stoi(each);
        getline(cs1,each,','); y1 = std::stoi(each);
        getline(cs2,each,','); x2 = std::stoi(each);
        getline(cs2,each,','); y2 = std::stoi(each);

        if (cmd.compare("toggle")==0) toggle(x1,y1,x2,y2,lights);
        if (cmd.compare("on")==0)     turn_on(x1,y1,x2,y2,lights);
        if (cmd.compare("off")==0)    turn_off(x1,y1,x2,y2,lights);
    }

    int counter = 0;
    for (auto l1: lights) {
        for (auto l2: l1) {
            //if (l2) ++counter;
            counter += l2;
        }
    }

    //std::cout << "Part 1: " << counter << std::endl;
    std::cout << "Part 2: " << counter << std::endl;
}