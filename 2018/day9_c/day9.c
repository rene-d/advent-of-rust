// cc -o day9 -Wall -Wextra -DSTANDALONE -O2 day9.c

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
#include <assert.h>

struct Marble
{
    uint32_t value;
    struct Marble *prev, *next;
};

struct Marble *ll_insert(struct Marble *cur, uint32_t value)
{
    struct Marble *m = malloc(sizeof(struct Marble));

    m->value = value;

    if (cur == NULL)
    {
        m->next = m;
        m->prev = m;
    }
    else
    {
        m->next = cur->next;
        m->prev = cur;

        cur->next->prev = m;
        cur->next = m;
    }

    return m;
}

struct Marble *ll_remove(struct Marble *cur)
{
    struct Marble *ret;

    ret = (cur->next == cur) ? NULL : cur->next;

    cur->next->prev = cur->prev;
    cur->prev->next = cur->next;

    free(cur);

    return ret;
}

uint32_t solve(uint32_t elves, uint32_t points)
{
    uint32_t *scores;
    struct Marble *current;

    scores = (uint32_t *)calloc(elves, sizeof(uint32_t));

    current = ll_insert(NULL, 0);

    for (uint32_t i = 1; i < points; ++i)
    {

        if (i % 23 == 0)
        {
            for (int k = 0; k < 7; ++k)
                current = current->prev;
            scores[i % elves] += i + current->value;

            current = ll_remove(current);
        }
        else
        {
            for (int k = 0; k < 1; ++k)
                current = current->next;

            current = ll_insert(current, i);
        }
    }

    uint32_t max = scores[0];
    for (uint32_t i = 1; i < elves; i++)
    {
        if (max < scores[i])
            max = scores[i];
    }

    while ((current = ll_remove(current)) != NULL)
        ;
    free(scores);

    return max;
}

uint32_t c_solve(uint32_t elves, uint32_t points)
{
    // fprintf(stderr, "\033[2musing C implementation...\033[0m\n");
    return solve(elves, points);
}


#ifdef STANDALONE

void tests()
{
    static const uint32_t testcases[][3] = {{9, 25, 32},
                                            {10, 1618, 8317},
                                            {13, 7999, 146373},
                                            {21, 6111, 54718},
                                            {30, 5807, 37305}};
    int ret = EXIT_SUCCESS;

    for (size_t i = 0; i < sizeof(testcases) / sizeof(testcases[0]); ++i)
    {
        const uint32_t *t = testcases[i];
        uint32_t res = solve(t[0], t[1]);
        printf("solve(%2u, %4u) = %6u ", t[0], t[1], res);
        if (res == t[2])
        {
            printf("\033[32mOK\033[0m\n");
        }
        else
        {
            printf("\033[31mFAIL\033[0m (expected: %u)\n", t[2]);
            ret = EXIT_FAILURE;
        }
    }

    exit(ret);
}

int main(int argc, char *argv[])
{
    uint32_t elves, points;
    FILE *input;

    if (argc >= 2 && strcmp(argv[1], "-t") == 0)
    {
        tests();
        return EXIT_SUCCESS;
    }

    input = fopen(argc <= 1 ? "input.txt" : argv[1], "r");
    if (input == NULL)
    {
        printf("cannot read input\n");
        return EXIT_FAILURE;
    }
    if (fscanf(input, "%u players; last marble is worth %u points", &elves, &points) != 2)
    {
        printf("cannot parse input\n");
        return EXIT_FAILURE;
    }
    fclose(input);

    printf("%u\n", solve(elves, points));
    printf("%u\n", solve(elves, points * 100));

    return 0;
}

#endif
