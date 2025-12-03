// [Day 2: Gift Shop](https://adventofcode.com/2025/day/2)

package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Range struct {
	start uint64
	end   uint64
}

type Puzzle struct {
	ranges    []Range
	maxEnd    uint64
	maxDigits int
}

func newPuzzle(data string) (*Puzzle, error) {
	ranges := []Range{}

	for _, rangeStr := range strings.Split(strings.TrimSpace(data), ",") {
		parts := strings.Split(rangeStr, "-")
		if len(parts) != 2 {
			continue
		}

		a, errA := strconv.ParseUint(parts[0], 10, 64)
		if errA != nil {
			return nil, fmt.Errorf("error parsing start value %s: %w", parts[0], errA)
		}

		b, errB := strconv.ParseUint(parts[1], 10, 64)
		if errB != nil {
			return nil, fmt.Errorf("error parsing end value %s: %w", parts[1], errB)
		}

		ranges = append(ranges, Range{start: a, end: b})
	}

	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i].start < ranges[j].start
	})

	var maxEnd uint64
	if len(ranges) > 0 {
		for _, r := range ranges {
			if r.end > maxEnd {
				maxEnd = r.end
			}
		}
	} else {
		return nil, fmt.Errorf("no ranges")
	}

	maxDigits := len(strconv.FormatUint(maxEnd, 10))

	return &Puzzle{
		ranges:    ranges,
		maxEnd:    maxEnd,
		maxDigits: maxDigits,
	}, nil
}

func (p *Puzzle) inRanges(n uint64) bool {
	i := sort.Search(len(p.ranges), func(i int) bool {
		return p.ranges[i].start >= n
	})

	if i < len(p.ranges) && p.ranges[i].start == n {
		return n <= p.ranges[i].end
	}

	if i > 0 {
		prevRange := p.ranges[i-1]
		return n >= prevRange.start && n <= prevRange.end
	}

	return false
}

func (p *Puzzle) part1() uint64 {
	var answer uint64

	hMin := uint64(1)
	hMax := uint64(10)

	for i := 1; i <= p.maxDigits/2; i++ {

		loopEnd := hMax
		if p.maxEnd < loopEnd {
			loopEnd = p.maxEnd
		}

		for h := hMin; h < loopEnd; h++ {
			n := h*hMax + h

			if p.inRanges(n) {
				answer += n
			}
		}

		hMin *= 10
		hMax *= 10
	}

	return answer
}

func (p *Puzzle) part2() uint64 {
	found := make(map[uint64]struct{})

	sMin := uint64(1)
	sMax := uint64(10)

	for h := 1; h <= p.maxDigits/2; h++ {

		for s := sMin; s < sMax; s++ {
			n := s

			for j := 1; j <= (p.maxDigits / h); j++ {
				n = n*sMax + s

				if n > p.maxEnd {
					break
				}

				if p.inRanges(n) {
					found[n] = struct{}{}
				}
			}
		}

		sMin *= 10
		sMax *= 10
	}

	var sum uint64
	for n := range found {
		sum += n
	}

	return sum
}

func solve(data []byte) (uint64, uint64) {

	puzzle, err := newPuzzle(string(data))
	if err != nil {
		log.Fatal("Error:", err)
	}

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
