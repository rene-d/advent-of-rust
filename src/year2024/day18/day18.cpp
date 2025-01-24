// [Day 18: RAM Run](https://adventofcode.com/2024/day/18)

#include <vector>
#include <deque>
#include <tuple>
#include <cstdio>

#define MEM_SIZE 71

struct point_t
{
    int x;
    int y;
};

static bool operator==(const point_t &a, const point_t &b)
{
    return a.x == b.x && a.y == b.y;
}

typedef std::vector<point_t> positions_t;

enum byte_t
{
    SAFE = 0,
    CORRUPTED = 1,
};

static const point_t directions[] = {
    {0, 1},   // down
    {0, -1},  // up
    {1, 0},   // right
    {-1, 0}}; // left

static unsigned find_path(const std::vector<byte_t> &memory)
{
    std::deque<std::tuple<point_t, unsigned>> queue;
    std::vector<bool> seen(MEM_SIZE * MEM_SIZE, false);

    const point_t start_pos = {0, 0};
    const point_t end_pos = {MEM_SIZE - 1, MEM_SIZE - 1};

    queue.push_back(std::make_tuple(start_pos, 0));

    while (queue.empty() == false)
    {
        point_t pos;
        int steps;

        std::tie(pos, steps) = queue.front();
        queue.pop_front();

        if (pos == end_pos)
        {
            return steps;
        }

        seen[pos.y * MEM_SIZE + pos.x] = true;

        for (const auto &dir : directions)
        {
            const int x = pos.x + dir.x;
            const int y = pos.y + dir.y;

            if (x < 0 || x >= MEM_SIZE || y < 0 || y >= MEM_SIZE)
            {
                continue;
            }

            if (!seen[y * MEM_SIZE + x] && memory[y * MEM_SIZE + x] == byte_t::SAFE)
            {
                queue.push_back(std::make_tuple(point_t{x, y}, steps + 1));
                seen[y * MEM_SIZE + x] = true;
            }
        }
    }

    return 0;
}

static void part1(const positions_t &byte_positions)
{
    std::vector<byte_t> memory(MEM_SIZE * MEM_SIZE, byte_t::SAFE);

    int count = 0;
    for (const auto &pos : byte_positions)
    {
        memory[pos.y * MEM_SIZE + pos.x] = byte_t::CORRUPTED;

        count++;
        if (count == 1024)
        {
            break;
        }
    }

    printf("%u\n", find_path(memory));
}

static void part2(const positions_t &byte_positions)
{
    std::vector<byte_t> memory(MEM_SIZE * MEM_SIZE, byte_t::SAFE);

    for (const auto &pos : byte_positions)
    {
        memory[pos.y * MEM_SIZE + pos.x] = byte_t::CORRUPTED;

        if (memory[0] == byte_t::CORRUPTED                          // start position
            || memory[MEM_SIZE * MEM_SIZE - 1] == byte_t::CORRUPTED // end position
            || find_path(memory) == 0)                              // no more path
        {
            printf("%d,%d\n", pos.x, pos.y);
            return;
        }
    }
}

static positions_t read_input(const char *filename)
{
    FILE *f;
    int x, y;
    positions_t byte_positions;

    f = fopen(filename, "r");
    if (f != NULL)
    {
        while (fscanf(f, "%d,%d\n", &x, &y) == 2)
        {
            byte_positions.push_back(point_t{x, y});
        }
        fclose(f);
    }

    return byte_positions;
}

int main(int argc, char *argv[])
{
    const char *filename = "input.txt";

    if (argc > 1 && argv[1] != NULL)
    {
        filename = argv[1];
    }

    const positions_t &&byte_positions = read_input(filename);

    part1(byte_positions);
    part2(byte_positions);

    return 0;
}
