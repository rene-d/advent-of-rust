// [Day 22: Monkey Market](https://adventofcode.com/2024/day/22)

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

// nextSecret computes the next secret value using the same small PRNG
// transformations as the Rust implementation. All operations are kept
// in 24-bit space (mod 2^24).
func nextSecret(secret int64) int64 {
	secret = (secret ^ (secret * 64)) % 16_777_216
	secret = (secret ^ (secret / 32)) % 16_777_216
	secret = (secret ^ (secret * 2048)) % 16_777_216
	return secret
}

// Puzzle holds the parsed initial secrets.
type Puzzle struct {
	initialSecrets []int64
}

// NewPuzzle parses the input lines and collects initial secrets until a
// non-integer line is encountered (behaves like Rust's map_while).
func NewPuzzle(data string) *Puzzle {
	var secs []int64
	for _, line := range strings.Split(strings.TrimSpace(data), "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		v, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			break
		}
		secs = append(secs, v)
	}
	return &Puzzle{initialSecrets: secs}
}

// Part1 applies nextSecret 2000 times to each initial secret and returns
// the sum of final secrets.
func (p *Puzzle) Part1() int64 {
	var sum int64
	for _, initial := range p.initialSecrets {
		secret := initial
		for i := 0; i < 2000; i++ {
			secret = nextSecret(secret)
		}
		sum += secret
	}
	return sum
}

// Part2 analyses the price-digit sequences and accumulates banana values
// grouped by the sequence of four successive differences.
func (p *Puzzle) Part2() int64 {
	bananas := make(map[[4]int64]int64)

	for _, initial := range p.initialSecrets {
		prices := make([]int64, 0, 2001)

		secret := initial
		prices = append(prices, secret%10)
		for i := 0; i < 2000; i++ {
			secret = nextSecret(secret)
			prices = append(prices, secret%10)
		}

		seen := make(map[[4]int64]struct{})
		for i := 0; i+4 < len(prices); i++ {
			p0 := prices[i : i+5]
			seq := [4]int64{p0[1] - p0[0], p0[2] - p0[1], p0[3] - p0[2], p0[4] - p0[3]}
			if _, ok := seen[seq]; !ok {
				seen[seq] = struct{}{}
				bananas[seq] += p0[4]
			}
		}
	}

	var best int64
	for _, v := range bananas {
		if v > best {
			best = v
		}
	}

	return best
}

// solve is the package-level solver used by main and tests.
func solve(data []byte) (int64, int64) {
	p := NewPuzzle(string(data))
	return p.Part1(), p.Part2()
}

func main() {
	filename := "input.txt"
	elapsed := false

	for i := 1; i < len(os.Args); i++ {
		arg := os.Args[i]
		if arg == "--elapsed" {
			elapsed = true
		} else if !strings.HasPrefix(arg, "-") {
			filename = arg
		}
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	if elapsed {
		start := time.Now()
		r1, r2 := solve(data)
		duration := time.Since(start)

		fmt.Println(r1)
		fmt.Println(r2)
		fmt.Printf("elapsed: %f ms\n", duration.Seconds()*1000.0)
	} else {
		r1, r2 := solve(data)
		fmt.Println(r1)
		fmt.Println(r2)
	}
}
