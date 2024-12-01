// cc -o day1 -Wall -O2 day1.c

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

static int int_cmp(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}

int main(int argc, char *argv[])
{
    FILE *f;
    int a, b;
    int *left = NULL, *right = NULL;
    size_t size = 0;
    size_t n = 0;

    if (argc >= 2)
        f = fopen(argv[1], "r");
    else
        f = fopen("input.txt", "r");

    while (fscanf(f, "%d %d", &a, &b) == 2)
    {
        if (n >= size)
        {
            size += 16;
            left = (int *)realloc(left, size * sizeof(int));
            right = (int *)realloc(right, size * sizeof(int));
        }
        left[n] = a;
        right[n] = b;
        n += 1;
    }

    qsort(left, n, sizeof(int), int_cmp);
    qsort(right, n, sizeof(int), int_cmp);

    // part 1
    int part1 = 0;
    for (size_t i = 0; i < n; ++i)
    {
        part1 += abs(left[i] - right[i]);
    }
    printf("%d\n", part1);

    // part 2
    int part2 = 0;
    for (size_t i = 0; i < n; ++i)
    {
        int a = left[i];
        for (size_t j = 0; j < n; ++j)
        {
            if (right[j] == a)
            {
                part2 += a;
            }
        }
    }
    printf("%d\n", part2);

    free(left);
    free(right);

    return 0;
}
