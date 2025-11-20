// [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)
// cc -std=c11 -o day2 -Wall -O2 day2.c

#define _XOPEN_SOURCE 600

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Check if the report of levels in the line is "safe".
// If 'skip' is non-negative, skip that index when checking safety.
static bool is_safe(const char *line, int *count, int skip)
{
    const char *next = line;
    int n = 0;
    long a, b;
    bool safe_ab = true;
    bool safe_ba = true;
    bool first = true;

    while (*next != '\0' && *next != '\n') {
        long curr = strtol(next, (char **)&next, 10);
        if (n++ == skip) {
            continue;
        }

        a = b;
        b = curr;

        if (first) {
            first = false;
            continue;
        }

        if (!(1 <= (a - b) && (a - b) <= 3)) {
            safe_ab = false;
        }
        if (!(1 <= (b - a) && (b - a) <= 3)) {
            safe_ba = false;
        }
    }

    if (count != NULL) {
        *count = n;
    }
    return safe_ab || safe_ba;
}

// Algorithm 1: check with parsing intergers on the fly.
// Slower but no need to store levels.
void algo1(const char *line, int *part1, int *part2)
{
    int count = 0;
    if (is_safe(line, &count, -1)) {
        (*part1)++;
    }
    for (int skip = 0; skip < count; ++skip) {
        if (is_safe(line, NULL, skip)) {
            (*part2)++;
            break;
        }
    }
}

// Algorithm 2: parse all integers first then check safety.
// Faster but requires to presume the number of levels of realloc the array.
void algo2(const char *line, int *part1, int *part2)
{
    int levels[16] = {0};
    size_t n = 0;
    const char *next = line;

    while (*next != '\0' && *next != '\n') {
        if (n >= sizeof(levels) / sizeof(levels[0])) {
            fprintf(stderr, "Error: too many levels in line\n");
            exit(1);
        }
        levels[n++] = (int)strtol(next, (char **)&next, 10);
    }

    // first check without skipping
    {
        bool safe_ab = true;
        bool safe_ba = true;
        for (size_t i = 0; i < n - 1; ++i) {
            int a = levels[i];
            int b = levels[i + 1];
            if (!(1 <= (a - b) && (a - b) <= 3)) {
                safe_ab = false;
            }
            if (!(1 <= (b - a) && (b - a) <= 3)) {
                safe_ba = false;
            }
        }

        if (safe_ab || safe_ba) {
            (*part1)++;
            (*part2)++;
            return; // levels are safe, no need to check further
        }
    }

    // second check with trying to skip each level
    for (size_t skip = 0; skip < n; ++skip) {
        bool first = true;
        bool safe_ab = true;
        bool safe_ba = true;
        int a, b;

        for (size_t i = 0; i < n; ++i) {

            if (i == skip) {
                continue;
            }

            a = b;
            b = levels[i];

            if (first) {
                first = false;
                continue;
            }

            if (!(1 <= (a - b) && (a - b) <= 3)) {
                safe_ab = false;
            }
            if (!(1 <= (b - a) && (b - a) <= 3)) {
                safe_ba = false;
            }
        }

        if (safe_ab || safe_ba) {
            (*part2)++;
            break;
        }
    }
}

int main(int argc, char *argv[])
{
    bool elapsed = false;
    void (*algo)(const char *, int *, int *) = algo2;

    for (int i = 2; i < argc; ++i) {
        if (strcmp(argv[i], "--elapsed") == 0) {
            elapsed = true;
        }
        if (strcmp(argv[i], "--alt") == 0) {
            algo = algo1;
        }
    }

    struct timespec ts_start, ts_end;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);

    FILE *f = fopen((argc >= 2) ? argv[1] : "input.txt", "r");
    if (f == NULL) {
        perror("fopen");
        exit(1);
    }

    int part1 = 0;
    int part2 = 0;

    while (!feof(f)) {
        char line[256];

        if (fgets(line, sizeof(line), f) == NULL) {
            break;
        }

        algo(line, &part1, &part2);
    }

    fclose(f);

    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    printf("%d\n", part1);
    printf("%d\n", part2);

    if (elapsed) {
        double elapsed = (ts_end.tv_sec - ts_start.tv_sec) +
                         (ts_end.tv_nsec - ts_start.tv_nsec) / 1e9;
        printf("elapsed: %.6f ms\n", elapsed * 1000.);
    }

    return 0;
}
