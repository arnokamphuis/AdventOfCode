#include <vector>
#include <iostream>
#include <sstream>
#include <string>
#include <map>
#include <algorithm>

class chann {
public:
    bool initialized;
    bool isvalue;
    unsigned short int value;
    std::string l_str, r_str;
    chann* l;
    chann* r;
    std::string op, name;

    bool is_number(const std::string& s) {
        return !s.empty() && std::find_if(s.begin(), 
            s.end(), [](char c) { return !std::isdigit(c); }) == s.end();
    }
    
    chann(std::string n, std::string v) {
        name = n;
        initialized=false;
        if (is_number(v)) { 
            isvalue = true; 
            initialized = true; 
            value = std::stoi(v);
            l = nullptr;
            r = nullptr; 
        } else {
            isvalue = false;
            op = "";
             
            size_t pos = v.find("NOT");
            if (pos != std::string::npos) { 
                l_str = ""; 
                l = nullptr;
                r_str = v.substr(pos+4);
                if (is_number(r_str)) {
                    r = new chann(r_str, r_str);
                    initialized = true;
                } else {
                    r = nullptr;
                }
                op = "NOT";
            }

            pos = v.find("AND");
            if (pos != std::string::npos) { 
                l_str = v.substr(0,pos-1);
                if (is_number(l_str)) l = new chann(l_str, l_str); else l = nullptr;
                r_str = v.substr(pos+4);
                if (is_number(r_str)) r = new chann(r_str, r_str); else r = nullptr;
                op = "AND";
            }

            pos = v.find("OR");
            if (pos != std::string::npos) { 
                l_str = v.substr(0,pos-1);
                if (is_number(l_str)) l = new chann(l_str, l_str); else l = nullptr;
                r_str = v.substr(pos+3);
                if (is_number(r_str)) r = new chann(r_str, r_str); else r = nullptr;
                op = "OR";
            }

            pos = v.find("LSHIFT");
            if (pos != std::string::npos) { 
                l_str = v.substr(0,pos-1);
                if (is_number(l_str)) l = new chann(l_str, l_str); else l = nullptr;
                r_str = v.substr(pos+7);
                if (is_number(r_str)) r = new chann(r_str, r_str); else r = nullptr;
                op = "LSHIFT";
            }

            pos = v.find("RSHIFT");
            if (pos != std::string::npos) { 
                l_str = v.substr(0,pos-1);
                if (is_number(l_str)) l = new chann(l_str, l_str); else l = nullptr;
                r_str = v.substr(pos+7);
                if (is_number(r_str)) r = new chann(r_str, r_str); else r = nullptr;
                op = "RSHIFT";
            }

            if (op.size()==0) {
                op = "CHAN";
                l_str = v;
                l = nullptr;
                r_str = "";
                r = nullptr;
            }
        }
    }


    unsigned short int getvalue() {
        unsigned short int lval = 0;
        unsigned short int rval = 0;
        if (isvalue) return value;
        else {
            if (l != nullptr) lval = l->getvalue();
            if (r != nullptr) rval = r->getvalue();
            if (op.compare("AND")==0) value = lval & rval;
            if (op.compare("OR")==0) value = lval | rval;
            if (op.compare("NOT")==0) value = ~rval;
            if (op.compare("LSHIFT")==0) value = lval << rval;
            if (op.compare("RSHIFT")==0) value = lval >> rval;
            if (op.compare("CHAN")==0) value = lval;
            isvalue = true;
            return value;
        }
    }

    void init(std::map<std::string, chann* >& channels ) {
        if (initialized) return;

        if ((l_str.size()!=0) && (l==nullptr)) { l = channels[l_str]; }
        if (l!=nullptr) l->init(channels); 
        if ((r_str.size()!=0) && (r==nullptr)) { r = channels[r_str]; }
        if (r!=nullptr) r->init(channels);

        initialized = true;
    }
};

int main() {

    std::map<std::string, chann*> channels;
    std::map<std::string, chann*> channels2;
    std::string line;

    while (getline(std::cin,line)) {
        size_t p = line.find(" -> ");
        std::string value   = line.substr(0,p);
        std::string channel = line.substr(p+4);
        channels[channel]  = new chann(channel, value);
        channels2[channel] = new chann(channel, value);
    }

    for (auto ch=channels.begin(); ch!=channels.end(); ++ch)
        ch->second->init(channels);

    auto finda = channels.find("a");
    if (finda != channels.end()) {
        int value_a = channels["a"]->getvalue();
        std::cout << "Part 1: " << value_a << std::endl;
        channels2["b"] = new chann("b", std::to_string(value_a));

        for (auto ch=channels2.begin(); ch!=channels2.end(); ++ch)
            ch->second->init(channels2);

        value_a = channels2["a"]->getvalue();
        std::cout << "Part 2: " << value_a << std::endl;
    }
}