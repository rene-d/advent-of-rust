// [Day 5: Cafeteria](https://adventofcode.com/2025/day/5)

package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

// mergeRanges merges overlapping or adjacent ranges.
func mergeRanges(ranges [][2]uint64) [][2]uint64 {
	if len(ranges) == 0 {
		return [][2]uint64{}
	}

	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i][0] < ranges[j][0]
	})

	merged := make([][2]uint64, 0)
	curStart := ranges[0][0]
	curEnd := ranges[0][1]

	for _, r := range ranges[1:] {
		s, e := r[0], r[1]

		if s <= curEnd+1 {
			if e > curEnd {
				curEnd = e
			}
		} else {
			merged = append(merged, [2]uint64{curStart, curEnd})
			curStart = s
			curEnd = e
		}
	}

	merged = append(merged, [2]uint64{curStart, curEnd})
	return merged
}

// solve parses the input and returns (part1, part2).
func solve(data []byte) (int, uint64) {

	var ranges [][2]uint64
	var freshIDs []uint64

	rangeScanner := bufio.NewScanner(strings.NewReader(string(data)))
	for rangeScanner.Scan() {
		line := rangeScanner.Text()
		if line == "" {
			continue
		}
		p := strings.SplitN(line, "-", 2)
		if len(p) == 2 {
			a, err := strconv.ParseUint(p[0], 10, 64)
			if err != nil {
				panic(err)
			}
			b, err := strconv.ParseUint(p[1], 10, 64)
			if err != nil {
				panic(err)
			}

			ranges = append(ranges, [2]uint64{a, b})
		} else {
			id, err := strconv.ParseUint(p[0], 10, 64)
			if err != nil {
				panic(err)
			}
			freshIDs = append(freshIDs, id)
		}
	}

	merged := mergeRanges(ranges)

	// Part 1
	part1 := 0
	for _, id := range freshIDs {

		i := sort.Search(len(merged), func(j int) bool {
			return merged[j][0] >= id
		})

		if i > 0 && id <= merged[i-1][1] {
			part1++

		}
	}

	// Part 2
	var part2 uint64 = 0
	for _, r := range merged {
		part2 += r[1] - r[0] + 1
	}

	return part1, part2
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
		result1, result2 := solve(data)
		duration := time.Since(start)

		fmt.Println(result1)
		fmt.Println(result2)
		fmt.Printf("elapsed: %f ms\n", duration.Seconds()*1000.0)
	} else {
		result1, result2 := solve(data)

		fmt.Println(result1)
		fmt.Println(result2)
	}
}
