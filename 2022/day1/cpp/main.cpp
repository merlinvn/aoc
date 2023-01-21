#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

int main(int argc, char *argv[]) {
  // read input from file
  std::ifstream input_file("/home/neo/Projects/aoc/2022/day1/input.txt");

  // read input into vector by reading line by line
  std::vector<int> elves;

  int calories = 0;

  for (std::string line; std::getline(input_file, line);) {
    /* std::cout << line << std::endl; */
    if (line.empty()) {
      elves.push_back(calories);
      calories = 0;
      continue;
    }
    calories += std::stoi(line);
  }

  // sort vector
  std::sort(elves.begin(), elves.end());

  // print out the last element
  std::cout << "Highest calories: " << elves.back() << std::endl;


  // get the sum of the last 3 elves
  int sum = 0;
  for (int i = elves.size() - 1; i > elves.size() - 4; i--) {
    sum += elves[i];
  }

  std::cout << "Sum of last 3 elves: " << sum << std::endl;

  input_file.close();
  return 0;
}
