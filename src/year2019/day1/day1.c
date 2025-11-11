// [Day 1: The Tyranny of the Rocket Equation](https://adventofcode.com/2019/day/1)
// cc -o day1 -Wall -O2 day1.c

#include <stdio.h>

int main(int argc, char *argv[])
{
    FILE *f = fopen((argc >= 2) ? argv[1] : "input.txt", "r");

    int mass, part1 = 0, part2 = 0;
    while (fscanf(f, "%d", &mass) == 1)
    {
        part1 += mass / 3 - 2;

        int fuel = mass;
        while ((fuel = fuel / 3 - 2) > 0)
        {
            part2 += fuel;
        }
    }

    fclose(f);

    printf("%d\n", part1);
    printf("%d\n", part2);

    return 0;
}
