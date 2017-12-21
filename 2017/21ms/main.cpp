#include <string>
#include <map>
#include <vector>
#include <iostream>
#include <cmath>

void find_and_replace_all(std::string & data, std::string toSearch, std::string replaceStr)
{
	// Get the first occurrence
	size_t pos = data.find(toSearch);
 
	// Repeat till end is reached
	while( pos != std::string::npos)
	{
		// Replace this occurrence of Sub String
		data.replace(pos, toSearch.size(), replaceStr);
		// Get the next occurrence from the current position
		pos =data.find(toSearch, pos + toSearch.size());
	}
}

class tile {
    int size;
    std::string leds;

    bool check_flippedh(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                if (leds[j*size+fi]!=desc[j*size+i]) return false;
            }
        }
        return true;
    }

    bool check_flippedv(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[fj*size+i]!=desc[j*size+i]) return false;
            }
        }
        return true;
    }

    bool check_normal(const std::string& desc) {
        //std::cout << "tile::check_normal() " << leds << "|" << desc << std::endl;
        for (int i=0; i<size; ++i) {
            for (int j=0; j<size; ++j) {
                if (leds[j*size+i]!=desc[j*size+i]) return false;
            }
        }
        return true;
    }

    bool check_lr(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[fi*size+j]!=desc[j*size+i]) return false;
            }
        }
        return true;        
    }

    bool check_rr(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[i*size+fj]!=desc[j*size+i]) return false;
            }
        }
        return true;        
    }

    bool check_fr(const std::string& desc) {
        for (int i=0; i<size; ++i) {
            int fi = size-1-i;
            for (int j=0; j<size; ++j) {
                int fj = size-1-j;
                if (leds[fj*size+fi]!=desc[j*size+i]) return false;
            }
        }
        return true;        
    }

    bool check(const std::string& desc) {
        std::string d = desc;
        find_and_replace_all(d,"/","");
        if (check_normal(d)) return true;
        if (check_lr(d)) return true;
        if (check_fr(d)) return true;
        if (check_rr(d)) return true;
        //if (check_flippedv(d)) return true;
        if (check_flippedh(d)) return true;
        return false;
    }

    void change_into(const std::string& desc) {
        ++size;
        //std::cout << leds << " change into: " << desc << std::endl;
        update_leds(desc);
    }

    void update_leds(const std::string& desc) {
        leds = "";
        for (auto c: desc) if (c!='/') leds += c;
        //std::cout << "tile::update_leds(): leds " << leds << std::endl;
    }

public:
    tile(int s, const std::string& desc) : size(s) {
        update_leds(desc);
    }

    int get_size() { return size; } 

    void evolve(const std::map<std::string, std::string>& rules) {
        bool changed = false;
        //std::cout << "tile::evolve() " << size << "\t" << leds << std::endl;
        for (auto r: rules) {
            std::string from = r.first;
            find_and_replace_all(from, "/", "");
            int s = std::sqrt(from.size());
            if(s == size) {
                //std::cout << "tile::evolve() : " << r.first << " -> " << r.second << std::endl;
                if (check(r.first)) {
                    change_into(r.second);
                    changed = true;
                    break;
                }
            }
        }
        if (!changed) 
            std::cout << "EEEEEEEEEEEEEEEEEEEEEERRRRRRRRRRRRRRRRRROOOOOOOOOOOOOOOOOOOOOORRRRRRRRRRRRRRRRRRRRRRRR" << std::endl;

    }

    char led(int i, int j) {
        return leds[i+size*j];
    }

};

class grid {
    int size;
    std::vector< std::vector< char > > leds;
    std::vector< std::vector< tile* > > tiles;
    std::map<std::string, std::string> rules;
public:
    grid() {
        size = 3;
        tile* first = new tile(size, ".#./..#/###");

        for (int i=0;i<size;++i) {
            std::vector<char> line;
            for (int j=0;j<size;++j) {
                line.push_back(first->led(j,i));
            }
            leds.push_back(line);
        }
    }

    int get_size() { return size; }

    void add_rule(const std::string& from, const std::string& to) {
        rules[from] = to;
    }

    void delete_tiles() {
        for (auto& tl: tiles) {
            for (auto t: tl) 
                delete t;
            tl.clear();
        }
        tiles.clear();
    }

    void create_tiles() {
        int tile_size = ( (size%2==0) ? 2: 3);
        int tilecount = size/tile_size;
        //std::cout << "create_tiles(): tilecount: " << tilecount << std::endl;
        for (int tc_x=0; tc_x<tilecount; ++tc_x) {
            std::vector< tile* > tile_line;
            for (int tc_y=0; tc_y<tilecount; ++tc_y) {
                std::string tilestring;
                for (int x=0;x<tile_size;++x) {
                    int x_index = tile_size*tc_x+x;
                    for (int y=0;y<tile_size;++y) {
                        int y_index = tile_size*tc_y+y;
                        tilestring += leds[y_index][x_index];
                    }
                    tilestring += '/';
                }
                tilestring.erase(tilestring.end()-1);

                tile* t = new tile(tile_size, tilestring);
                tile_line.push_back(t);
            }
            tiles.push_back(tile_line);
        }
    }

    void update_tiles() {
        //std::cout << "grid::update_tiles()" <<std::endl;
        for (auto tl: tiles)
            for (auto t: tl)
                t->evolve(rules);
    }

    void grid_from_tiles() {
        int ts = tiles[0][0]->get_size();
        size = ts * tiles.size();

        leds.clear();
        for (int x=0; x<size; ++x) {
            std::vector< char > ledline;
            for (int y=0; y<size; ++y) {
                int tx = x/ts;
                int ty = y/ts;

                int ix = x%ts;
                int iy = y%ts;

                ledline.push_back( tiles[ty][tx]->led(iy,ix) );
            } 
            leds.push_back(ledline);
        }
    }

    void update() {
        create_tiles();
        update_tiles();
        grid_from_tiles();
        delete_tiles();
    }

    void print() {
        for (auto lc: leds) {
            for (auto c: lc) 
                std::cout << c;
            std::cout << std::endl;
        }
    }

    int get_turned_on() {
        int count=0, oc=0;
        for (auto lc: leds)
            for (auto c: lc) 
                if (c=='#') ++count; else ++oc;

        if ( (count+oc)!=size*size ) {
            std::cout << "ERROR: Number of leds not correct" << std::endl;
        }
        return count;
    }
};

int main() {

    grid* g = new grid();

    std::string line;
    while (getline(std::cin, line)) {
        std::size_t p = line.find(" => ");
        g->add_rule(line.substr(0,p), line.substr(p+4));
    }

    // std::cout << "==============================================" << std::endl;
    // g->print();
    // std::cout << "==============================================" << std::endl;


    int run1 = 5;
    int run2 = 18;
    for (int i=0; i<run1; ++i) { 
        g->update();
        // std::cout << "==============================================" << std::endl;
        // g->print();
        // std::cout << "==============================================" << std::endl;
    }

    std::cout << "Part 1: " << g->get_turned_on() << std::endl;

    for (int i=0; i<(run2-run1); ++i) { 
        g->update();
        // std::cout << "==============================================" << std::endl;
        // g->print();
        // std::cout << "==============================================" << std::endl;
    }

    std::cout << "Part 2: " << g->get_turned_on() << std::endl;

}