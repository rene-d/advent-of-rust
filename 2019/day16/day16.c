// cc -o day16 -Wall -O2 day16.c
// https://adventofcode.com/2019/day/16

#include <stdio.h> // well... is it really necessary ?
#include <stdlib.h>
#include <string.h>
#include <inttypes.h>

int main()
{
    int8_t *data;
    int size;
    long m;
    FILE *f;
    int8_t *p, *t;
    static const int8_t pattern[] = {0, 1, 0, -1};

    //
    // read and parse the input
    //

    f = fopen("input.txt", "r");     // no failure check
    fseek(f, 0, SEEK_END);           // no failure check
    m = ftell(f);                    // not really convenient to get the size
    fseek(f, 0, SEEK_SET);           // rewind to the beginning
    data = malloc(m);                // no failure check, no cast mandatory, and I imply the element is 1-byte wide
    fread(data, 1, m, f);            // no check
    fclose(f);                       // the compiler could guess itself the handle should be closed
    for (size = 0; size < m; ++size) // silent (and implied) mix of types between long and int
    {
        if (data[size] >= '0' && data[size] <= '9') // actually it doesn't overflow because I have allocated enough memory
            data[size] -= '0';                      // but there is no check here, and it could be difficult to verify in a review
        else                                        // and I use a signed integer to index the array. can be handy, but dangerous too
            break;
    }

    //
    // part 1
    //

    t = malloc(size);             // still no check
    p = malloc(size);             // no cast and no element size
    memcpy(p, data, size);        // no check
    for (int k = 0; k < 100; k++) // ++i or i++ ? I should decide !
    {
        for (int n = 0; n < size; n++)
        {
            int s = 0;
            for (int i = 0; i < size; ++i)
            {
                s += pattern[((i + 1) / (n + 1)) % 4] * p[i]; // mix between int and int8_t
            }                                                 // and only a SAST could verify there is no overflow here
            t[n] = abs(s) % 10;                               // other silent cast int to int8_t, even it is legitimate
        }
        memcpy(p, t, size);
    }
    for (int i = 0; i < 8; i++)
        printf("%d", p[i]); // you have to know precisely the formatter for the variable type (but modern compilers eventually complain)
    printf("\n");
    free(p); // p is deallocated but the pointer p is still usable. SAST analyzers could verify that
    free(t);

    //
    // part 2
    //

    // ugly but no way to do this shorter/comprehensive I think
    int offset = (((((data[0] * 10 + data[1]) * 10 + data[2]) * 10 + data[3]) * 10 + data[4]) * 10 + data[5]) * 10 + data[6];
    int n = size * 10000 - offset; // overflow, carry ?
    printf("%d\n", n);
    p = malloc(n); // check, element type, cast ?
    t = malloc(n);
    for (int i = 0; i < n; ++i)
    {                                     // accolade or not accolade for a only one statement ?
        p[i] = data[(i + offset) % size]; // I should remember that data is at most size byte wide
    }
    for (int k = 0; k < 100; ++k) // naming this variable is useless
    {
        int8_t s = 0;
        for (int i = n - 1; i >= 0; i--) // if you decide later to use an unsigned you're dead
        {
            s = (s + p[i]) % 10;
            t[i] = s;
        }
        memcpy(p, t, n);
    }
    for (int i = 0; i < 8; i++)
        printf("%d", p[i]);
    printf("\n");
    free(p);
    free(t);

    // free data
    free(data); // only valgrind or a SAST could verify that I haven't forget to deallocate memory

    return 0;
}

// Nota: no doc, no tests
