#include<iostream>
#include<stdlib.h>
#include<stdio.h>
#include<vector>

int toint(const char& c) {
  return c - '0';
}

int find_sum(const std::vector<int>& l, const int delta) {
  int sum = 0;
  int size = l.size();
  for (int i=0; i<size; i++) {
	  if (l[i] == l[(i+delta)%size])
		  sum+=l[i];
  }  
  return sum;
}

int main() {
  char ch;
  int size=0;
  std::vector<int> list;

  while (true) {
    ch = getchar();
    if (ch == EOF) break;
    list.push_back(toint(ch));
  }

  size = list.size();
  int delta=size/2;
  std::cout << "Part 1: " << find_sum(list,1) << std::endl;
  std::cout << "Part 2: " << find_sum(list,delta) << std::endl;
}