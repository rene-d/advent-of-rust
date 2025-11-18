// [Day 16: Aunt Sue](https://adventofcode.com/2015/day/16)

package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

func solve(data []byte) (int, int) {
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	aunts := make(map[string]map[string]int)

	re := regexp.MustCompile(`Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)`)

	for _, line := range lines {
		matches := re.FindStringSubmatch(line)
		if len(matches) != 8 {
			continue
		}

		sue := matches[1]
		aunt := make(map[string]int)

		val1, _ := strconv.Atoi(matches[3])
		val2, _ := strconv.Atoi(matches[5])
		val3, _ := strconv.Atoi(matches[7])

		aunt[matches[2]] = val1
		aunt[matches[4]] = val2
		aunt[matches[6]] = val3

		aunts[sue] = aunt
	}

	// Part 1
	part1 := 0
	for sue, aunt := range aunts {
		if getValue(aunt, "children", 3) != 3 {
			continue
		}
		if getValue(aunt, "cats", 7) != 7 {
			continue
		}
		if getValue(aunt, "samoyeds", 2) != 2 {
			continue
		}
		if getValue(aunt, "pomeranians", 3) != 3 {
			continue
		}
		if getValue(aunt, "akitas", 0) != 0 {
			continue
		}
		if getValue(aunt, "vizslas", 0) != 0 {
			continue
		}
		if getValue(aunt, "goldfish", 5) != 5 {
			continue
		}
		if getValue(aunt, "trees", 3) != 3 {
			continue
		}
		if getValue(aunt, "cars", 2) != 2 {
			continue
		}
		if getValue(aunt, "perfumes", 1) != 1 {
			continue
		}

		part1, _ = strconv.Atoi(sue)
		break
	}

	// Part 2
	part2 := 0
	for sue, aunt := range aunts {
		if getValue(aunt, "children", 3) != 3 {
			continue
		}
		// cats should be greater than
		if getValue(aunt, "cats", 7+1) <= 7 {
			continue
		}
		if getValue(aunt, "samoyeds", 2) != 2 {
			continue
		}
		// pomeranians should be fewer than
		if getValue(aunt, "pomeranians", 3-1) >= 3 {
			continue
		}
		if getValue(aunt, "akitas", 0) != 0 {
			continue
		}
		if getValue(aunt, "vizslas", 0) != 0 {
			continue
		}
		// goldfish should be fewer than
		if getValue(aunt, "goldfish", 5-1) >= 5 {
			continue
		}
		// trees should be greater than
		if getValue(aunt, "trees", 3+1) <= 3 {
			continue
		}
		if getValue(aunt, "cars", 2) != 2 {
			continue
		}
		if getValue(aunt, "perfumes", 1) != 1 {
			continue
		}

		part2, _ = strconv.Atoi(sue)
		break
	}

	return part1, part2
}

func getValue(aunt map[string]int, key string, defaultValue int) int {
	if val, ok := aunt[key]; ok {
		return val
	}
	return defaultValue
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
