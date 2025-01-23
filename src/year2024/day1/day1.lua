#!/usr/bin/env lua
-- [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

input_file = arg[1] or "input.txt"

local left = {}
local right = {}

for line in io.lines(input_file) do
    local a, b = line:match("(%d+)%s*(%d+)")
    table.insert(left, tonumber(a))
    table.insert(right, tonumber(b))
end

-- Sort both lists
table.sort(left)
table.sort(right)

-- Part 1: Calculate the sum of absolute differences
local part1 = 0
for i = 1, #left do
    part1 = part1 + math.abs(left[i] - right[i])
end
print(part1)

-- Part 2: Count occurrences of each element in the right list
local right_counts = {}
for _, num in ipairs(right) do
    right_counts[num] = (right_counts[num] or 0) + 1
end

-- Calculate the weighted sum for part 2
local part2 = 0
for _, num in ipairs(left) do
    part2 = part2 + num * (right_counts[num] or 0)
end
print(part2)
