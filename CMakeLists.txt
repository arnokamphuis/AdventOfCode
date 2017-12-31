# require minimum cmake version 2.8
cmake_minimum_required (VERSION 2.8)

# adding a project
project (AoC-2017)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(MSVC)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /std:c++17")
else(MSVC)
SET(CMAKE_C_COMPILER gcc.exe)
SET(CMAKE_CXX_COMPILER g++.exe)
endif(MSVC)

set(TARGETS2017 1ms 2ms 3ms 4ms 5ms 6ms 7ms 8ms 9ms 10ms 11ms 12ms 13ms 14ms 15ms 16ms 17ms 18ms 19ms 20ms 21ms 22ms 23ms 24ms 25ms)
FOREACH(t ${TARGETS2017})
    MESSAGE("PROCESSING" ${t})
    add_executable(${t} 2017/${t}/main.cpp)
    set_target_properties( ${t} PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/${t}/ )
ENDFOREACH(t)

set(TARGETS2015 day1 day2 day3 day5 day6 day7 day8 day9 day10 day11 day12 day13 day14 day15 day16 day17 day18 day19 day20 day21)
FOREACH(t ${TARGETS2015})
    MESSAGE("PROCESSING" ${t})
    add_executable(${t} 2015/${t}/main.cpp)
    set_target_properties( ${t} PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2015/${t}/ )
ENDFOREACH(t)

add_executable(day4 2015/day4/main.cpp 2015/day4/md5.cpp)
set_target_properties( day4 PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2015/day4/ )
