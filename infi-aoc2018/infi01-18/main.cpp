#include <iostream>
#include <map>
#include <set>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vector>
#include <wchar.h>

//#include "basetimer.h"
#include "logger.h"

std::vector<char32_t> directions = {L'║', L'╔', L'╗', L'╠', L'╦', L'╚',
                                    L'╝', L'╬', L'╩', L'═', L'╣'};

int main() {

  std::vector<std::wstring> input;

  std::wstring line;
  while (getline(std::wcin, line)) {
    input.push_back(line);
  }

  int size_x = input[0].length();
  int size_y = input.size();

  for (auto c : directions)
    std::wcout << c << "\t";
  std::wcout << "\n";
  std::wcout << L"Line 0: " << (char32_t)input[0][0] << "\n";

  std::wcout << "Size: " << size_x << "\t" << size_y << "\n";
  return 0;
}
