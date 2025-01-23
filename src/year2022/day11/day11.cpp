#include <vector>
#include <iostream>
#include <list>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <cstring>
#include <cassert>
#include <algorithm>

struct Operation
{
    char opcode;
    uint64_t param;

    uint64_t calc(uint64_t n) const
    {
        // WARNING no overflow checks in C++
        switch (this->opcode)
        {
        case '^':
            return n * n;
        case '+':
            return n + this->param;
        case '*':
            return n * this->param;
        default:
            throw std::logic_error("operation");
        }
    }
};

struct Monkey
{
    std::list<uint64_t> items;
    Operation operation;
    uint64_t divisible_by;
    size_t if_true;
    size_t if_false;
    size_t inspections;
};

void solve(const std::vector<Monkey> &monkeys_orig, int rounds)
{
    std::vector<Monkey> monkeys = monkeys_orig;

    uint64_t modulus = 1;
    for (const auto &monkey : monkeys)
    {
        modulus *= monkey.divisible_by;
    }

    for (int round = 0; round < rounds; ++round)
    {
        for (auto &monkey : monkeys)
        {
            // Nota: this loop can run forever if if_true/if_false refers the current monkey and C++ allows it.
            // In Rust it's not possible because of the borrowing that forces to copy the items vector
            while (!monkey.items.empty())
            {
                monkey.inspections += 1;

                uint64_t item = monkey.items.front();
                monkey.items.pop_front();

                uint64_t worry_level = monkey.operation.calc(item);

                if (rounds == 20)
                {
                    worry_level = worry_level / 3;
                }
                else
                {
                    worry_level = worry_level % modulus;
                }

                if (worry_level % monkey.divisible_by == 0)
                {
                    monkeys[monkey.if_true].items.push_back(worry_level);
                }
                else
                {
                    monkeys[monkey.if_false].items.push_back(worry_level);
                }
            }
        }
    }

    std::sort(monkeys.begin(), monkeys.end(),
              [](const Monkey &a, const Monkey &b) -> bool
              {
                  return a.inspections > b.inspections;
              });
    uint64_t monkey_business_level = (uint64_t)monkeys[0].inspections * (uint64_t)monkeys[1].inspections;

    std::cout << monkey_business_level << std::endl;
}

void read_data(const char *filename, std::vector<Monkey> &monkeys)
{
    std::ifstream f;
    std::string line;

    f.open(filename);

    if (!f.is_open())
    {
        throw std::logic_error("bad filename");
    }

    while (true)
    {
        Monkey monkey{
            .items = std::list<uint64_t>(),
            .operation = Operation{.opcode = 0, .param = 0},
            .divisible_by = 0,
            .if_true = 0,
            .if_false = 0,
            .inspections = 0,
        };

        if (!std::getline(f, line))
            break;

        // cumbersome parsing of a list of numbers
        std::getline(f, line);
        assert(line.find("  Starting items: ") == 0);
        line = line.substr(strlen("  Starting items: "));
        std::string item;
        std::stringstream ss(line);
        while (std::getline(ss, item, ','))
        {
            monkey.items.push_back(std::stoul(item));
        }

        // equally tedious parsing
        std::getline(f, line);
        assert(line.find("  Operation: new = ") == 0);
        line = line.substr(strlen("  Operation: new = "));
        if (line == "old * old")
        {
            monkey.operation.opcode = '^';
        }
        else if (line.find("old + ") == 0)
        {
            monkey.operation.opcode = '+';
            monkey.operation.param = std::stoul(line.substr(strlen("old + ")));
        }
        else if (line.find("old * ") == 0)
        {
            monkey.operation.opcode = '*';
            monkey.operation.param = std::stoul(line.substr(strlen("old * ")));
        }
        else
        {
            throw std::logic_error("unknown operation");
        }

        // and so one. I don't even know if std:stoul throws in case of error
        std::getline(f, line);
        assert(line.find("  Test: divisible by ") == 0);
        monkey.divisible_by = std::stoul(line.substr(strlen("  Test: divisible by ")));

        std::getline(f, line);
        assert(line.find("    If true: throw to monkey ") == 0);
        monkey.if_true = std::stoul(line.substr(strlen("    If true: throw to monkey ")));

        std::getline(f, line);
        assert(line.find("    If false: throw to monkey ") == 0);
        monkey.if_false = std::stoul(line.substr(strlen("    If false: throw to monkey ")));

        // skip the empty line
        std::getline(f, line);

        monkeys.push_back(monkey);
    }
    f.close();
}

int main(int argc, char *argv[])
{
    std::vector<Monkey> monkeys;

    read_data(argc >= 2 ? argv[1] : "input.txt", monkeys);

    solve(monkeys, 20);
    solve(monkeys, 10000);

    return 0;
}
