#ifndef __LOGGER_H__
#define __LOGGER_H__

#include <ostream>
#include <iostream>

class logger {
private:
    std::ostream& __stream;
    std::ostream& __err_stream;
public:
    logger(std::ostream& s, std::ostream& es) : __stream(s), __err_stream(es) {}

    template <typename T>
    void log(T v) const {
        __stream << v;
    }

    template <typename T>
    void error(T v) const {
        __err_stream << v;
    }

};

#endif // __LOGGER_H__