// [Day 9: Disk Fragmenter](https://adventofcode.com/2024/day/9)

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

const freeSpace = uint32(0xFFFFFFFF) // u32::MAX

func loadDisk(data string) []uint32 {
	data = strings.TrimSpace(data)
	var disk []uint32

	for i, c := range data {
		size := int(c - '0')
		for j := 0; j < size; j++ {
			if i%2 == 1 {
				disk = append(disk, freeSpace)
			} else {
				disk = append(disk, uint32(i/2))
			}
		}
	}

	return disk
}

func computeChecksum(disk []uint32) uint64 {
	var checksum uint64

	for i, c := range disk {
		if c != freeSpace {
			checksum += uint64(c) * uint64(i)
		}
	}

	return checksum
}

func part1(data string) uint64 {
	disk := loadDisk(data)

	i := 0
	j := len(disk) - 1
	for i < j {
		if disk[i] == freeSpace {
			for disk[j] == freeSpace && i < j {
				j--
			}
			disk[i], disk[j] = disk[j], disk[i]
			j--
		}
		i++
	}

	return computeChecksum(disk)
}

func part2(data string) uint64 {
	disk := loadDisk(data)

	j := len(disk) - 1
	moved := make([]bool, len(disk))

	for {
		// Get file size
		for disk[j] == freeSpace {
			if j == 0 {
				return computeChecksum(disk)
			}
			j--
		}

		k := j
		for disk[k] == disk[j] {
			if k == 0 {
				return computeChecksum(disk)
			}
			k--
		}

		fileSize := j - k
		nextJ := k

		if moved[j] {
			j = nextJ
			continue
		}

		// Find free space
		i := 0
		for {
			for i < len(disk) && disk[i] != freeSpace {
				i++
			}
			k = i
			for k < len(disk) && disk[k] == freeSpace {
				k++
			}
			freeSpaceSize := k - i

			if freeSpaceSize >= fileSize && i < j {
				for count := 0; count < fileSize; count++ {
					disk[i], disk[j] = disk[j], disk[i]
					moved[i] = true
					i++
					j--
				}
				break
			}

			i = k
			if i >= len(disk) {
				break
			}
		}

		j = nextJ
	}
}

func solve(data []byte) (uint64, uint64) {
	input := string(data)
	return part1(input), part2(input)
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
