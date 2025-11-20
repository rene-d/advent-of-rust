// cc -std=c11 -o part2 -Wall -O3 part2.c

#define _XOPEN_SOURCE 600

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static int N;
static char grid[141 * 141];
static int m = 0;

#define G(x, y) grid[N * (y) + (x)]

static void walk(int c, int x, int y)
{
    if (G(x, y) != 0) {
        return;
    }

    if (x == 1 && y == 0 && c > m) {
        m = c;
    } else {
        G(x, y) = 2;

        if (x > 0 && G(x - 1, y) == 0) {
            walk(c + 1, x - 1, y);
        }

        if (x < N - 1 && G(x + 1, y) == 0) {
            walk(c + 1, x + 1, y);
        }

        if (y > 0 && G(x, y - 1) == 0) {
            walk(c + 1, x, y - 1);
        }

        if (y < N - 1 && G(x, y + 1) == 0) {
            walk(c + 1, x, y + 1);
        }

        G(x, y) = 0;
    }
}

int main(int argc, char *argv[])
{
    FILE *f;
    char buf[200];

    // read the square maze
    f = fopen((argc > 1) ? argv[1] : "input.txt", "r");
    if (f == NULL) {
        return EXIT_FAILURE;
    }

    // check the size
    fgets(buf, 200, f);
    N = (int)strlen(buf) - 1;
    if (N != 23 && N != 141) {
        printf("bad size: %d\n", N);
        return EXIT_FAILURE;
    }

    for (int y = 0; y < N; ++y) {
        for (int x = 0; x < N; ++x) {
            G(x, y) = (buf[x] == '#') ? 1 : 0;
        }
        fgets(buf, 200, f);
    }
    fclose(f);

    struct timespec start, finish;

    clock_gettime(CLOCK_MONOTONIC, &start);

    // walk all paths
    walk(0, N - 2, N - 1);

    // print the longest one
    printf("%d\n", m);

    clock_gettime(CLOCK_MONOTONIC, &finish);
    finish.tv_nsec += 1000000000;
    start.tv_sec += 1;
    long duration = (finish.tv_nsec - start.tv_nsec) / 1000 + (finish.tv_sec - start.tv_sec) * 1000000;

    fprintf(stderr, "Time elapsed: %.6lfs\n", (double)duration / 1000000.);

    return EXIT_SUCCESS;
}
