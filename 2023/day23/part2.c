// cc -o part2 -Wall -O3 part2.c

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>
#include <stdbool.h>
#include <time.h>
#include <stdarg.h>
#include <unistd.h>
#include <signal.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/stat.h>

static int N;
static char grid[141 * 141];
static int m = 0;

#define G(x, y) grid[N * (y) + (x)]

static void walk(int c, int x, int y)
{
    if (G(x, y) != 0)
    {
        return;
    }

    if (x == N - 2 && y == N - 1 && c > m)
    {
        m = c;
    }
    else
    {
        G(x, y) = 2;

        if (x > 0 && G(x - 1, y) == 0)
        {
            walk(c + 1, x - 1, y);
        }

        if (x < N - 1 && G(x + 1, y) == 0)
        {
            walk(c + 1, x + 1, y);
        }

        if (y > 0 && G(x, y - 1) == 0)
        {
            walk(c + 1, x, y - 1);
        }

        if (y < N - 1 && G(x, y + 1) == 0)
        {
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
    if (f == NULL)
    {
        return EXIT_FAILURE;
    }

    // check the size
    fgets(buf, 200, f);
    N = strlen(buf) - 1;
    if (N != 23 && N != 141)
    {
        printf("bad size: %d\n", N);
        return EXIT_FAILURE;
    }

    for (int y = 0; y < N; ++y)
    {
        for (int x = 0; x < N; ++x)
        {
            G(x, y) = (buf[x] == '#') ? 1 : 0;
        }
        fgets(buf, 200, f);
    }
    fclose(f);

    // walk all paths
    walk(0, 1, 0);

    // print the longest one
    printf("%d\n", m);

    return EXIT_SUCCESS;
}
