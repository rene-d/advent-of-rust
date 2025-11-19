// [Day 19: Linen Layout](https://adventofcode.com/2024/day/19)

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

// puzzle holds the patterns and the designs to evaluate.
type puzzle struct {
	patterns []string
	designs  []string
}

// newPuzzle parses the input where the first section (comma-separated
// patterns) and the second section (designs, one per line) are separated
// by a blank line.
func newPuzzle(data string) *puzzle {
	parts := strings.SplitN(data, "\n\n", 2)
	var patterns []string
	var designs []string
	if len(parts) >= 1 {
		patterns = strings.Split(strings.TrimSpace(parts[0]), ", ")
	}
	if len(parts) == 2 {
		for _, l := range strings.Split(strings.TrimSpace(parts[1]), "\n") {
			l = strings.TrimSpace(l)
			if l != "" {
				designs = append(designs, l)
			}
		}
	}
	return &puzzle{patterns: patterns, designs: designs}
}

// countDesignWays returns the number of ways the given design string can be
// composed by concatenating patterns from the puzzle. The count uses dynamic
// programming and is returned as uint64 to match potential growth.
func (p *puzzle) countDesignWays(design string) uint64 {
	n := len(design)
	dp := make([]uint64, n+1)
	dp[0] = 1

	for i := 1; i <= n; i++ {
		for _, pattern := range p.patterns {
			m := len(pattern)
			if i >= m && design[i-m:i] == pattern {
				dp[i] += dp[i-m]
			}
		}
	}

	return dp[n]
}

// part1 counts how many designs have at least one valid decomposition.
func (p *puzzle) part1() int {
	cnt := 0
	for _, d := range p.designs {
		if p.countDesignWays(d) != 0 {
			cnt++
		}
	}
	return cnt
}

// part2 returns the total number of decompositions across all designs.
func (p *puzzle) part2() uint64 {
	var sum uint64
	for _, d := range p.designs {
		sum += p.countDesignWays(d)
	}
	return sum
}

// solve is the package-level solver used by main and tests.
func solve(data []byte) (int, uint64) {
	p := newPuzzle(string(data))
	return p.part1(), p.part2()
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
