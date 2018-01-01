#include <iostream>
#include <sstream>
#include <vector>
#include <map>
#include <set>
#include <string>
#include <climits>
#include <algorithm>
#include <utility>
#include <cmath>

class particle {
public:
    int x[3];
    int v[3];
    int a[3];
    int id;

    int cur[3];

    particle(std::string line, int i) : id(i) {        
        std::size_t p_p = line.find("p=<");
        std::string p_str = line.substr(p_p+3);
        p_str.erase(p_str.end()-1);
        std::istringstream ss_p(p_str);
        std::string p_val; 
        getline(ss_p,p_val,','); x[0] = std::stoi(p_val);
        getline(ss_p,p_val,','); x[1] = std::stoi(p_val);
        getline(ss_p,p_val,','); x[2] = std::stoi(p_val);        

        std::size_t p_v = line.find(", v=<");
        std::string v_str = line.substr(p_v+5);
        v_str.erase(v_str.end()-1);
        std::istringstream ss_v(v_str);
        std::string v_v; 
        getline(ss_v,v_v,','); v[0] = std::stoi(v_v);
        getline(ss_v,v_v,','); v[1] = std::stoi(v_v);
        getline(ss_v,v_v,','); v[2] = std::stoi(v_v);        

        std::size_t p_a = line.find(", a=<");
        std::string a_str = line.substr(p_a+5);
        a_str.erase(a_str.end()-1);
        std::istringstream ss_a(a_str);
        std::string a_v; 
        getline(ss_a,a_v,','); a[0] = std::stoi(a_v);
        getline(ss_a,a_v,','); a[1] = std::stoi(a_v);
        getline(ss_a,a_v,','); a[2] = std::stoi(a_v);

        cur[0] = x[0];
        cur[1] = x[1];
        cur[2] = x[2];
    }

    // doesn't work yet
    int calculate_collision_time(particle* other) {
        int ca[3], cb[3], cc[3], D[3];
        int t1[3], t2[3], t[3];

        bool col[3];
        for (int i=0;i<3;++i) {
            col[i] = false;
            ca[i] = 1*(a[i]-other->a[i]);
            cb[i] = 2*(v[i]-other->v[i]);
            cc[i] = 2*(x[i]-other->x[i]);
            if ( (ca[i]==0) and (cb[i]==0) and (cc[i]==0) ) {
                col[i] = true;
                //std::cout << "(col still in " << i << ")\t";
            } else {

                D[i] = cb[i]*cb[i] - 4*ca[i]*cc[i];

                //std::cout << "(D: " << D[i] << ")" ; 
                if (D[i]==0) {
                    if (ca[i]!=0) {
                        t1[i]  = -cb[i]/(2*ca[i]); 
                        if (t1[i]>0) col[i] = true; else t1[i] = INT_MAX;
                    } else {
                        //std::cout << "linear";
                        if (cb[i]!=0) {
                            if (cc[i]%cb[i]==0) {
                                t1[i] = std::abs(-1.0*cc[i]/cb[i]);
                                t2[i] = INT_MAX;
                                col[i] = true;
                            } else {
                                t1[i] = INT_MAX;
                                t2[i] = INT_MAX;
                                col[i] = false;
                            }
                        } else {
                            col[i] = false;
                        }
                    }

                    t2[i] = INT_MAX;

                    //std::cout << "(t1:" << t1[i] << ")";

                    t[i] = t1[i];
                } else if (D[i]>0) {
                    //std::cout << "(ca:" << ca[i] << ",cb:" << cb[i] << ",cc:" << cc[i] << ")";
                    if (ca[i]!=0) {
                        t1[i] = (-cb[i]-sqrt(D[i]))/(2*ca[i]); 
                        if (t1[i]>0) col[i] = true; else t1[i] = INT_MAX;

                        t2[i] = (-cb[i]+sqrt(D[i]))/(2*ca[i]); 
                        if (t2[i]>0) col[i] = true; else t2[i] = INT_MAX;
                    } else { 
                        //std::cout << "linear";
                        if (cb[i]!=0) {
                            if (cc[i]%cb[i]==0) {
                                t1[i] = std::abs(-1.0*cc[i]/cb[i]);
                                t2[i] = INT_MAX;
                                col[i] = true;
                            } else {
                                t1[i] = INT_MAX;
                                t2[i] = INT_MAX;
                                col[i] = false;
                            }
                        } else {
                            col[i] = false;
                        }
                    }

                    //std::cout << "(col:" << col[i] << ",t1:" << t1[i] << ",t2:" << t2[i] << ")" ;
                    if (col[i])
                        t[i] = std::min(t1[i],t2[i]);
                    else
                        t[i] = INT_MAX;
                } else {
                    t1[i] = INT_MAX;
                    t2[i] = INT_MAX;
                    t[i] = std::min(t1[i],t2[i]);
                    col[i] = false;
                }
            }
        }

        float mint = INT_MAX;
        if (col[0] and col[1] and col[2]) {
            for (int i=0;i<3;++i)
                if (col[i]) if (t[i]<mint) mint=t[i];

            std::cout << "COLLISION at " << mint << " between " << id << " and " << other->id << std::endl;
        } else {
            mint = -1;
            //std::cout << "NO COLLISION " << std::endl;
        }

        return mint;
        //std::cout << t1[0] << "," << t1[1] << "," << t1[2] << "," << t2[0] << "," << t2[1] << "," <<t2[2] << std::endl;
    }

    int getmax() { return std::abs(a[0]) + std::abs(a[1]) + std::abs(a[2]);}

    void dostep() {
        for (int i=0; i<3; ++i) v[i] += a[i];
        for (int i=0; i<3; ++i) x[i] += v[i];
    }

    bool checkcollision(particle* other) {
        return ( (x[0]==other->x[0]) and (x[1]==other->x[1]) and (x[2]==other->x[2]) );
    }
};

int main() {
    std::string line;
    std::vector<particle*> particles;
    int min_d = INT_MAX;
    int i=0, j, min_p = -1;
    while (getline(std::cin,line)) {
        particle* p = new particle(line,i);

        int d_eventually = p->getmax();
        if (d_eventually<min_d) {
            min_d = d_eventually;
            min_p = i;
        }

        particles.push_back(p);
        ++i;
    }
    std::cout << "Part 1: " << min_p << std::endl;

    std::cout << "Number of particles: " << particles.size() << std::endl;

    std::vector<particle*>::iterator p1it, p2it;

    std::map<int, std::set<std::pair<particle*,particle*> > > collisions;
    std::vector<particle*> still_living = particles;

    for (int t=0; t<100000; ++t) {
        if (t%1000==0) std::cout << ".";
        for (auto p1: still_living) p1->dostep();
        
        std::set<particle*> toberemoved;
        for (auto p1: still_living) for (auto p2: still_living) if (p1!=p2) if (p1->checkcollision(p2)) {
            toberemoved.insert(p1);
            toberemoved.insert(p2);
        }

        for (auto p: toberemoved) {
            still_living.erase(std::find(still_living.begin(), still_living.end(), p));
        }
    }

    // i=0;
    // j=0;
    // for (p1it=particles.begin(); p1it!=particles.end(); ++p1it, ++i) {
    //     j=i+1;
    //     for (p2it=p1it+1; p2it!=particles.end(); ++p2it, ++j) {
    //         int t = (*p1it)->calculate_collision_time((*p2it));
    //         if (t>0) {
    //             collisions[t].insert(std::make_pair(*p1it,*p2it));
    //             collisions[t].insert(std::make_pair(*p2it,*p1it));
    //         }
    //     }            
    // }

    // for (auto col: collisions) {
    //     std::cout << "Resolving time: " << col.first << std::endl;
    //     std::set<particle*> toberemoved;
    //     for (auto pair: col.second) {
    //         particle* p1 = pair.first;
    //         particle* p2 = pair.second;

    //         auto p1f = std::find(still_living.begin(), still_living.end(), p1);
    //         auto p2f = std::find(still_living.begin(), still_living.end(), p2);

    //         if ( p1f!=still_living.end() and p2f!=still_living.end() ) {
    //             toberemoved.insert(p1);
    //             toberemoved.insert(p2);
    //         }
    //     }

    //     std::cout << "Found " << toberemoved.size() << std::endl;
    //     for (auto p: toberemoved) {
    //         //std::cout << "COLLISION removing the particles" << std::endl;
    //         still_living.erase(std::find(still_living.begin(), still_living.end(), p));
    //     }
    // }

    std::cout << "Part 2: " << still_living.size() << std::endl;

}