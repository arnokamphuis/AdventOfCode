#!/bin/bash
cd $1/$2
#g++ main.cpp -o $2.exe -std=gnu++11 -Ofast
clang++ -std=c++14 main.cpp -o $2.exe -Ofast
cd ../..
