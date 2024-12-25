package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	filename := "input.txt"
	if len(os.Args) >= 2 {
		filename = os.Args[1]
	}
	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	var keys, locks [][]int

	schematicsList := strings.Split(string(data), "\n\n")

	for _, schematics := range schematicsList {
		lines := strings.Split(schematics, "\n")

		heights := []int{-1, -1, -1, -1, -1}
		for _, line := range lines {
			if len(line) == 5 {
				for x, c := range line {
					if c == '#' {
						heights[x]++
					}
				}
			}
		}

		if lines[0] == "#####" {
			locks = append(locks, heights)
		} else {
			keys = append(keys, heights)
		}
	}

	sum := 0
	for _, lock := range locks {
		for _, key := range keys {
			valid := true
			for i := range key {
				if key[i]+lock[i] > 5 {
					valid = false
					break
				}
			}
			if valid {
				sum++
			}
		}
	}

	fmt.Println(sum)
}
