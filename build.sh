#!/bin/bash
cd $1/$2
g++ main.cpp -o $2.exe -std=gnu++11 -Ofast
cd ../..
