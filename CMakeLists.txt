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

#--------------------------------------------------------------------------------------------------
add_executable(1ms 2017/1ms/main.cpp)
set_target_properties( 1ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/1ms/ )

#--------------------------------------------------------------------------------------------------
add_executable(2ms 2017/2ms/main.cpp)
set_target_properties( 2ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/2ms/ )

#--------------------------------------------------------------------------------------------------
add_executable(3ms 2017/3ms/main.cpp)
set_target_properties( 3ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/3ms/ )

#--------------------------------------------------------------------------------------------------
add_executable(4ms 2017/4ms/main.cpp)
set_target_properties( 4ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/4ms/ )

#--------------------------------------------------------------------------------------------------
add_executable(5ms 2017/5ms/main.cpp)
set_target_properties( 5ms PROPERTIES RUNTIME_OUTPUT_DIRECTORY 2017/5ms/ )
