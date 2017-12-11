#include<iostream>
#include<stdlib.h>
#include<stdio.h>

int toint(const char& c) {
  return c - '0';
}

int main() {
  int firstv, v, prevv;
  char ch = getchar();
  int sum=0;
  
  firstv = toint(ch);
  prevv = firstv;
  while (ch = getchar()) {
	v = toint(ch);
	if (v==prevv)
	  sum += v;
	prevv=v;
  }
  if (v==firstv)
	sum+=v;
	
  std::cout << sum << std::endl;
}