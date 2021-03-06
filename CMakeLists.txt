# require minimum cmake version 2.8
cmake_minimum_required (VERSION 2.8)

enable_language(CXX)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wmultichar -std=gnu++17")
#set(CMAKE_CXX_STANDARD 17)
#set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_compile_options("$<$<CONFIG:RELEASE>:-Ofast>")

include_directories(${PROJECT_SOURCE_DIR}/general)

# adding a project
project (AoC)

set(TARGETS2018 day01-18 day02-18 day03-18 day04-18 day05-18 day06-18 day07-18 day08-18 day09-18 day10-18 day11-18 day12-18 day13-18 day14-18 day15-18 day16-18 day17-18 day18-18 day19-18 day20-18 day21-18 day22-18 day23-18 day24-18 day25-18)
FOREACH(t ${TARGETS2018})
    # add_executable(${t} ${PROJECT_SOURCE_DIR}/2018/${t}/main.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp ${PROJECT_SOURCE_DIR}/general/basetimer.cpp)
    add_executable(${t} ${PROJECT_SOURCE_DIR}/2018/${t}/main.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp)
    set_target_properties( ${t} PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2018/${t}/ )
    target_include_directories( ${t} PUBLIC ${PROJECT_SOURCE_DIR}/general )
ENDFOREACH(t)

set(TARGETS2017 1ms 2ms 3ms 4ms 5ms 6ms 7ms 8ms 9ms 11ms 12ms 13ms 15ms 16ms 17ms 18ms 19ms 20ms 21ms 22ms 23ms 24ms 25ms)
FOREACH(t ${TARGETS2017})
    add_executable(${t} ${PROJECT_SOURCE_DIR}/2017/${t}/main.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp)
    set_target_properties( ${t} PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/${t}/ )
    target_include_directories( ${t} PUBLIC ${PROJECT_SOURCE_DIR}/general )
ENDFOREACH(t)

set(TARGETS2015 day1 day2 day3 day5 day6 day7 day8 day9 day10 day11 day12 day13 day14 day15 day16 day17 day18 day19 day20 day21 day23 day24)
FOREACH(t ${TARGETS2015})
    add_executable(${t} ${PROJECT_SOURCE_DIR}/2015/${t}/main.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp)
    set_target_properties( ${t} PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2015/${t}/ )
    target_include_directories( ${t} PUBLIC ${PROJECT_SOURCE_DIR}/general )
ENDFOREACH(t)

add_executable(day4 ${PROJECT_SOURCE_DIR}/2015/day4/main.cpp ${PROJECT_SOURCE_DIR}/general/md5.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp)
set_target_properties( day4 PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2015/day4/ )
target_include_directories( day4 PUBLIC ${PROJECT_SOURCE_DIR}/general )

add_executable(10ms ${PROJECT_SOURCE_DIR}/2017/10ms/main.cpp ${PROJECT_SOURCE_DIR}/general/knot.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp)
set_target_properties( 10ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/10ms/ )
target_include_directories( 10ms PUBLIC ${PROJECT_SOURCE_DIR}/general )

add_executable(14ms ${PROJECT_SOURCE_DIR}/2017/14ms/main.cpp ${PROJECT_SOURCE_DIR}/general/knot.cpp ${PROJECT_SOURCE_DIR}/general/logger.cpp)
set_target_properties( 14ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/14ms/ )
target_include_directories( 14ms PUBLIC ${PROJECT_SOURCE_DIR}/general )
