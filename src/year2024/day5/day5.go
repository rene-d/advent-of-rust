// [Day 5: Print Queue](https://adventofcode.com/2024/day/5)

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

type Puzzle struct {
	orderingRules map[int]map[int]bool
	pageUpdates   [][]int
}

func newPuzzle(data string) *Puzzle {
	p := &Puzzle{
		orderingRules: make(map[int]map[int]bool),
		pageUpdates:   make([][]int, 0),
	}

	parts := strings.SplitN(data, "\n\n", 2)
	if len(parts) != 2 {
		log.Fatal("Invalid input format")
	}

	// Parse ordering rules
	for _, line := range strings.Split(parts[0], "\n") {
		if line == "" {
			continue
		}
		rule := strings.Split(line, "|")
		if len(rule) != 2 {
			continue
		}
		p1, err1 := strconv.Atoi(strings.TrimSpace(rule[0]))
		p2, err2 := strconv.Atoi(strings.TrimSpace(rule[1]))
		if err1 != nil || err2 != nil {
			continue
		}

		if p.orderingRules[p1] == nil {
			p.orderingRules[p1] = make(map[int]bool)
		}
		p.orderingRules[p1][p2] = true
	}

	// Parse page updates
	for _, line := range strings.Split(parts[1], "\n") {
		if line == "" {
			continue
		}
		parts := strings.Split(line, ",")
		update := make([]int, 0, len(parts))
		for _, part := range parts {
			val, err := strconv.Atoi(strings.TrimSpace(part))
			if err != nil {
				continue
			}
			update = append(update, val)
		}
		if len(update) > 0 {
			p.pageUpdates = append(p.pageUpdates, update)
		}
	}

	return p
}

func (p *Puzzle) part1() int {
	result := 0

	for _, pu := range p.pageUpdates {
		correctlyOrdered := true

		for i, page := range pu {
			if or, exists := p.orderingRules[page]; exists {
				hs := make(map[int]bool)
				for j := 0; j < i; j++ {
					hs[pu[j]] = true
				}

				hasIntersection := false
				for k := range or {
					if hs[k] {
						hasIntersection = true
						break
					}
				}

				if hasIntersection {
					correctlyOrdered = false
					break
				}
			}
		}

		if correctlyOrdered {
			result += pu[len(pu)/2]
		}
	}

	return result
}

func (p *Puzzle) part2() int {
	result := 0
	for _, update := range p.pageUpdates {
		result += p.bubbleSortUpdates(update)
	}
	return result
}

// Sort the page updates.
// If a fix has been made, return the middle page number.
// Return 0 otherwise.
func (p *Puzzle) bubbleSortUpdates(updates []int) int {
	pu := make([]int, len(updates))
	copy(pu, updates)
	reorder := false
	i := 0

	for i < len(pu) {
		page := pu[i]

		if or, exists := p.orderingRules[page]; exists {
			hs := make(map[int]bool)
			for j := 0; j < i; j++ {
				hs[pu[j]] = true
			}

			hasIntersection := false
			for k := range or {
				if hs[k] {
					hasIntersection = true
					break
				}
			}

			if hasIntersection {
				pu[i-1], pu[i] = pu[i], pu[i-1]
				i = 0
				reorder = true
				continue
			}
		}

		i++
	}

	if reorder {
		return pu[len(pu)/2]
	}
	return 0
}

func solve(data []byte) (int, int) {
	input := string(data)
	puzzle := newPuzzle(input)
	return puzzle.part1(), puzzle.part2()
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
