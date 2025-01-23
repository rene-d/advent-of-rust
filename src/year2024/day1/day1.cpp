// c++ -Wall -Wextra -Werror -std=c++23 -O2 -o day1 day1.cpp

#include <vector>
#include <iostream>
#include <fstream>
#include <sstream>
#include <ranges>
#include <map>
#include <algorithm>
#include <inttypes.h>

static void read_data(const char *filename, std::vector<int> &left, std::vector<int> &right)
{
    std::ifstream f;
    std::string line;

    f.open(filename);

    if (!f.is_open())
    {
        throw std::logic_error("bad filename");
    }

    while (!f.eof())
    {
        int a, b;
        f >> a >> b;

        if (!f.fail())
        {
            left.push_back(a);
            right.push_back(b);
        }
    }

    f.close();
}

int main(int argc, char *argv[])
{
    std::vector<int> left, right;

    if (argc >= 2)
        read_data(argv[1], left, right);
    else
        read_data("input.txt", left, right);

    std::sort(left.begin(), left.end());
    std::sort(right.begin(), right.end());

    // part 1

    int part1 = 0;
    for (auto [a, b] : std::views::zip(left, right)) // C++23
    {
        part1 += abs(a - b);
    }
    std::cout << part1 << std::endl;

    // part 2

    std::map<int, int> counter;
    for (auto b : right)
    {
        counter[b] += 1;
    }

    int part2 = 0;
    for (auto a : left)
    {
        part2 += a * counter[a];
    }
    std::cout << part2 << std::endl;

    return 0;
}
