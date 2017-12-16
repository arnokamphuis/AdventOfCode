#include <iostream>
#include <sstream>
#include <string>
#include <map>
#include <climits>

class reindeer {
protected:
    int ctime, stars, cdist;
    int speed, duration, resttime;
public:
    reindeer(int s, int d, int r) : speed(s), duration(d), resttime(r), ctime(0), stars(0) {}

    int calculate_distance(int time) {
        int intervaltime = duration+resttime;
        int cycles = time / intervaltime;
        int rest   = time % intervaltime;
        int movingtime = cycles * duration;
        if (rest<duration) movingtime+=rest; else movingtime+=duration;
        return movingtime * speed;
    }

    int step() {
        ++ctime;
        return cdist = calculate_distance(ctime);
    }

    void addstar() { ++stars; }
    int  getstars() { return stars; } 

    int getdistance() { return cdist; } 
};

int simulate(int time, std::map<std::string,reindeer*> reindeers ) {
    for (int t=0;t<time;++t) {
        int maxdist = INT_MIN;
        for (auto r: reindeers) {
            int d = r.second->step();
            if (d>maxdist) maxdist=d;
        }

        for (auto r: reindeers)
            if (r.second->getdistance()==maxdist) r.second->addstar();
    }

    int s, maxstars = INT_MIN;
    for (auto r: reindeers) 
        if ( (s = r.second->getstars() ) > maxstars ) maxstars = s;
    return maxstars;
}

int main() {
    std::map<std::string,reindeer*> reindeers;
    std::string line, name, temp;
    int s, d, r;
    while (getline(std::cin,line)) {
        std::istringstream ss(line);
        ss >> name >> temp >> temp >> s >> temp >> temp >> d >> temp >> temp >> temp >> temp >> temp >> temp >> r;
        reindeers[name] = new reindeer(s,d,r);
    }

    int t = 2503;
    int maxdist = INT_MIN;
    for (auto r: reindeers) {
        int d = r.second->calculate_distance(t);
        if (d>maxdist) maxdist=d;
    }

    std::cout << "Part 1: " << maxdist << std::endl;
    std::cout << "Part 2: " << simulate(t, reindeers) << std::endl;
    ;

}