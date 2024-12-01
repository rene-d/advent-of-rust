#!/usr/bin/env ruby
# [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

input_file = ARGV.length == 0 ? "input.txt" : ARGV[0]

left = []
right = []

File.open(input_file, "r") do |file|
  file.each_line do |line|
    a, b = line.split.map(&:to_i)
    left << a
    right << b
  end
end

left.sort!
right.sort!

# part 1
puts left.zip(right).map { |a, b| (a - b).abs }.sum

# part 2
right_counts = right.tally
puts left.sum { |a| a * right_counts[a].to_i }