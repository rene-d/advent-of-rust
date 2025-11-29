// cc -std=c11 -o day1 -Wall -O2 day1.c

#define _XOPEN_SOURCE 600

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static int int_cmp(const void *a, const void *b)
{
    return *(int *)a - *(int *)b;
}

int main(int argc, char *argv[])
{
    FILE *f;
    long data_size;
    char *data;
    int a, b;
    int *left = NULL, *right = NULL;
    size_t size = 0;
    size_t n = 0;
    int *right_count;
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
    if (fread(data, 1, data_size, f) != (size_t) data_size) {
        perror("fread");
	exit(1);
    }
    data[data_size] = '\0';
    fclose(f);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);

    // beginning of puzzle solution

    right_count = (int *)calloc(100000, sizeof(int));

    const char *p = data;
    int consumed = 0;
    while (p < (data + data_size) && sscanf(p, "%d %d%n", &a, &b, &consumed) == 2) {
        if (consumed <= 0)
            break;
        p += consumed;

        if (b >= 100000) {
            fprintf(stderr, "Error: b value %d exceeds right_count size\n", b);
            exit(1);
        }
        right_count[b] += 1;

        if (n >= size) {
            size += 128;
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
    for (size_t i = 0; i < n; ++i) {
        part1 += abs(left[i] - right[i]);
    }

    // part 2
    int part2 = 0;
    for (size_t i = 0; i < n; ++i) {
        int a = left[i];
        part2 += right_count[a] * a;
    }

    free(left);
    free(right);
    free(right_count);

    // end of puzzle solution

    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    free(data);

    printf("%d\n", part1);
    printf("%d\n", part2);

    for (int i = 2; i < argc; ++i) {
        if (strcmp(argv[i], "--elapsed") == 0) {
            double elapsed = (ts_end.tv_sec - ts_start.tv_sec) + (ts_end.tv_nsec - ts_start.tv_nsec) / 1e9;
            printf("elapsed: %.6f ms\n", elapsed * 1000.);
            break;
        }
    }

    return 0;
}
