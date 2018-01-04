#ifndef __LOGGER_H__
#define __LOGGER_H__

#include <ostream>

enum class logtype { logINFO, logERROR };

class logger {
private:
public:
    static std::ostream& get(logtype lt);
};

#endif // __LOGGER_H__