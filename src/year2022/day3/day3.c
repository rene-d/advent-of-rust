// cc -std=c11 -o day3 -Wall -O2 day3.c

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[])
{
    char line[100];
    FILE *f;
    int prio[52];
    int part1 = 0;
    int part2 = 0;

    f = fopen(argc >= 2 ? argv[1] : "input.txt", "r");
    while (fgets(line, sizeof(line), f) != NULL) {
        memset(prio, 0, sizeof(prio));

        for (size_t i = 0; i < strlen(line); ++i) {
            int shift = (i < strlen(line) / 2) ? 0 : 1;
            char c = line[i];
            if (c >= 'a' && c <= 'z')
                prio[c - 'a'] |= 1 << shift;
            if (c >= 'A' && c <= 'Z')
                prio[c - 'A' + 26] |= 1 << shift;
        }

        for (int i = 0; i < 52; ++i) {
            if (prio[i] == 3)
                part1 += i + 1;
        }
    }
    fclose(f);
    printf("%d\n", part1);

    f = fopen(argc >= 2 ? argv[1] : "input.txt", "r");
    while (1) {
        memset(prio, 0, sizeof(prio));

        for (int shift = 0; shift < 3; ++shift) {
            if (fgets(line, sizeof(line), f) == NULL)
                goto exit;
            for (const char *c = line; *c; ++c) {
                if (*c >= 'a' && *c <= 'z')
                    prio[*c - 'a'] |= 1 << shift;
                if (*c >= 'A' && *c <= 'Z')
                    prio[*c - 'A' + 26] |= 1 << shift;
            }
        }
        for (int i = 0; i < 52; ++i) {
            if (prio[i] == 7)
                part2 += i + 1;
        }
    }
exit:
    fclose(f);
    printf("%d\n", part2);

    return 0;
}
