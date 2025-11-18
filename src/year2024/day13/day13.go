// [Day 13: Claw Contraption](https://adventofcode.com/2024/day/13)

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

// Fraction represents a fraction with numerator and denominator
type Fraction struct {
	num int64
	den int64
}

// NewFraction creates a new fraction
func NewFraction(num, den int64) Fraction {
	if den == 0 {
		panic("division by zero")
	}
	// Simplify the fraction
	g := gcd(abs(num), abs(den))
	if den < 0 {
		num = -num
		den = -den
	}
	return Fraction{num: num / g, den: den / g}
}

// FromInt creates a fraction from an integer
func FromInt(n int64) Fraction {
	return Fraction{num: n, den: 1}
}

// Add adds two fractions
func (f Fraction) Add(other Fraction) Fraction {
	return NewFraction(f.num*other.den+other.num*f.den, f.den*other.den)
}

// Sub subtracts two fractions
func (f Fraction) Sub(other Fraction) Fraction {
	return NewFraction(f.num*other.den-other.num*f.den, f.den*other.den)
}

// Mul multiplies two fractions
func (f Fraction) Mul(other Fraction) Fraction {
	return NewFraction(f.num*other.num, f.den*other.den)
}

// Div divides two fractions
func (f Fraction) Div(other Fraction) Fraction {
	return NewFraction(f.num*other.den, f.den*other.num)
}

// IsInteger checks if the fraction is an integer
func (f Fraction) IsInteger() bool {
	return f.den == 1
}

// IsNegative checks if the fraction is negative
func (f Fraction) IsNegative() bool {
	return f.num < 0
}

// Numer returns the numerator if it's an integer
func (f Fraction) Numer() (int64, bool) {
	if f.IsInteger() {
		return f.num, true
	}
	return 0, false
}

// ClawMachine represents a claw machine
type ClawMachine struct {
	aX Fraction
	aY Fraction
	bX Fraction
	bY Fraction
	pX Fraction
	pY Fraction
}

// Parse creates a ClawMachine from a string
func ParseClawMachine(s string) ClawMachine {
	re := regexp.MustCompile(`\d+`)
	matches := re.FindAllString(s, -1)

	if len(matches) < 6 {
		panic("not enough numbers found")
	}

	values := make([]int64, 6)
	for i := 0; i < 6; i++ {
		val, err := strconv.ParseInt(matches[i], 10, 64)
		if err != nil {
			panic(err)
		}
		values[i] = val
	}

	return ClawMachine{
		aX: FromInt(values[0]),
		aY: FromInt(values[1]),
		bX: FromInt(values[2]),
		bY: FromInt(values[3]),
		pX: FromInt(values[4]),
		pY: FromInt(values[5]),
	}
}

// Price calculates the price for a given position offset
func (cm ClawMachine) Price(positionOffset int64) int64 {
	pX := cm.pX.Add(FromInt(positionOffset))
	pY := cm.pY.Add(FromInt(positionOffset))

	// a = (p_y - b_y * p_x / b_x) / (a_y - b_y * a_x / b_x)
	denom := cm.aY.Sub(cm.bY.Mul(cm.aX).Div(cm.bX))
	if denom.num == 0 {
		return 0
	}
	a := pY.Sub(cm.bY.Mul(pX).Div(cm.bX)).Div(denom)

	// b = (p_x - a * a_x) / b_x
	b := pX.Sub(a.Mul(cm.aX)).Div(cm.bX)

	if !a.IsInteger() || a.IsNegative() {
		return 0
	}
	if !b.IsInteger() || b.IsNegative() {
		return 0
	}

	aNum, _ := a.Numer()
	bNum, _ := b.Numer()
	return aNum*3 + bNum
}

// Puzzle represents the complete puzzle
type Puzzle struct {
	machines []ClawMachine
}

// NewPuzzle creates a new puzzle from the data
func NewPuzzle(data string) Puzzle {
	blocks := strings.Split(strings.TrimSpace(data), "\n\n")
	machines := make([]ClawMachine, 0, len(blocks))

	for _, block := range blocks {
		if strings.TrimSpace(block) == "" {
			continue
		}
		machines = append(machines, ParseClawMachine(block))
	}

	return Puzzle{machines: machines}
}

// Part1 solves part one
func (p Puzzle) Part1() int64 {
	sum := int64(0)
	for _, machine := range p.machines {
		sum += machine.Price(0)
	}
	return sum
}

// Part2 solves part two
func (p Puzzle) Part2() int64 {
	sum := int64(0)
	for _, machine := range p.machines {
		sum += machine.Price(10000000000000)
	}
	return sum
}

// Utility functions for fractions

func abs(n int64) int64 {
	if n < 0 {
		return -n
	}
	return n
}

func gcd(a, b int64) int64 {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func solve(data []byte) (int64, int64) {
	puzzle := NewPuzzle(string(data))
	return puzzle.Part1(), puzzle.Part2()
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
