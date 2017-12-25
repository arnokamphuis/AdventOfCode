#include <string>
#include <map>
#include <vector>
#include <iostream>
#include <cmath>

int ops[10];

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



class tilestorage {
    private:
    public:
        int size;
        char storage[3][3];

        tilestorage() { size = -1; }

        tilestorage(const std::string& s) {
            std::string init_s = s;
            //std::cout << "tilestorage(): " << s << std::endl;
            find_and_replace_all(init_s,"/","");
            int ssize = init_s.size();
            size = std::sqrt(ssize);
            int i = 0, x, y;
            for (auto c: init_s) {  
                x = i % size;
                y = i / size;
                storage[x][y] = c;
                ++i;
            }
        }

        tilestorage rotate_left() {
            ops[1]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[j][size-1-i] = storage[i][j];
            return ts;
        }
        
        tilestorage rotate_right() {
            ops[2]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[size-1-j][i] = storage[i][j];
            return ts;            
        }

        tilestorage rotate_full() {
            ops[3]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[size-1-j][size-1-i] = storage[i][j];
            return ts;            
        }

        tilestorage flip() {
            ops[4]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[i][size-1-j] = storage[i][j];
            return ts;            
        }

        tilestorage flip_vertical() {
            ops[5]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[size-1-i][j] = storage[i][j];
            return ts;            
        }

        tilestorage flip_rotate_left() {
            ops[6]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[j][i] = storage[i][j];
            return ts;
        }
        
        tilestorage flip_rotate_right() {
            ops[7]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[size-1-j][size-1-i] = storage[i][j];
            return ts;
        }
        
        tilestorage flip_rotate_full() {
            ops[8]++;
            tilestorage ts;
            ts.size = size;
            for (int i=0; i<size; ++i) for (int j=0; j<size; ++j) ts.storage[size-1-i][j] = storage[i][j];
            return ts;
        }

        std::string get_string() {
            std::string res = "";
            for (int i=0; i<size; ++i) {
                for (int j=0; j<size; ++j)
                    res += storage[i][j];
                if (i!=(size-1))
                    res += '/';
            }
            //std::cout << "get_string(): " << res << std::endl;
            return res;
        }
};


int main() {

    std::map<std::string, std::string> rules;
    std::string line;
    while (getline(std::cin, line)) {
        std::size_t p = line.find(" => ");
        tilestorage ts(line.substr(0,p));

        rules[ts.get_string()] = line.substr(p+4);
        rules[ts.rotate_left().get_string()] = line.substr(p+4);
        rules[ts.rotate_right().get_string()] = line.substr(p+4);
        rules[ts.rotate_full().get_string()] = line.substr(p+4);
        rules[ts.flip().get_string()] = line.substr(p+4);
        rules[ts.flip_rotate_left().get_string()] = line.substr(p+4);
        rules[ts.flip_rotate_right().get_string()] = line.substr(p+4);
        rules[ts.flip_rotate_full().get_string()] = line.substr(p+4);
        rules[ts.flip_vertical().get_string()] = line.substr(p+4);
    }

    for (auto r: rules) {
        std::cout << r.first << " => " << r.second << std::endl;
    }
}