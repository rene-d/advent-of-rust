// [Day 1: Secret Entrance](https://adventofcode.com/2025/day/1)
// cc -std=c11 -o day1 -Wall -O2 day1.c

#define _XOPEN_SOURCE 600

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static void solve(const char *data)
{
    const char *line;
    const char *end;
    int part1_pos = 50;
    int part1_count_zero = 0;
    int part2_pos = 50;
    int part2_count_zero = 0;

    line = data;
    while ((end = strchr(line, '\n')) != NULL) {

        char dir = line[0];
        int num = atoi(line + 1);
        int step;

        // compute part one
        switch (dir) {
        case 'L':
            part1_pos = (part1_pos + 100 - num) % 100;
            break;
        case 'R':
            part1_pos = (part1_pos + 100 + num) % 100;
            break;
        }
        part1_count_zero += (part1_pos == 0);

        // compute part two
        switch (dir) {
        case 'L':
            step = -1;
            break;
        case 'R':
            step = 1;
            break;
        }
        for (int i = 0; i < num; ++i) {
            part2_pos = (part2_pos + 100 + step) % 100;
            part2_count_zero += (part2_pos == 0);
        }

        line = end + 1;
    }

    printf("%d\n", part1_count_zero);
    printf("%d\n", part2_count_zero);
}

int main(int argc, char *argv[])
{
    FILE *f;
    long data_size;
    char *data;
    struct timespec ts_start, ts_end;

    f = fopen((argc >= 2) ? argv[1] : "input.txt", "r");
    if (f == NULL) {
        perror("fopen");
        exit(1);
    }
    fseek(f, 0, SEEK_END);
    data_size = ftell(f);
    if (data_size < 0) {
        perror("ftell");
        exit(1);
    }
    data = (char *)malloc(data_size + 1);
    fseek(f, 0, SEEK_SET);
    if (fread(data, 1, data_size, f) != (size_t)data_size) {
        perror("fread");
        exit(1);
    }
    data[data_size] = '\0';
    fclose(f);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);

    solve(data);

    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    free(data);

    for (int i = 2; i < argc; ++i) {
        if (strcmp(argv[i], "--elapsed") == 0) {
            double elapsed = (ts_end.tv_sec - ts_start.tv_sec) + (ts_end.tv_nsec - ts_start.tv_nsec) / 1e9;
            printf("elapsed: %.6f ms\n", elapsed * 1000.);
            break;
        }
    }

    return 0;
}
