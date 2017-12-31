# require minimum cmake version 3.7
cmake_minimum_required (VERSION 3.7)

# adding a project
project (AoC-2017)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(MSVC)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /std:c++17")
endif(MSVC)

#--------------------------------------------------------------------------------------------------
add_executable(1ms 2017/1ms/main.cpp)
set_target_properties( 1ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/1ms/ )

#--------------------------------------------------------------------------------------------------
add_executable(2ms 2017/2ms/main.cpp)
set_target_properties( 2ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/2ms/ )

#--------------------------------------------------------------------------------------------------
add_executable(3ms 2017/3ms/main.cpp)
set_target_properties( 3ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/3ms/ )