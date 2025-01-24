#!/usr/bin/env node
// [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

const fs = require('fs');

// Main function to handle file reading and calculations
async function main() {
    // Default input file or the one provided as a command line argument
    const inputFile = process.argv.length >= 3 ? process.argv[2] : 'input.txt';

    let left = [];
    let right = [];

    try {
        const data = await fs.promises.readFile(inputFile, 'utf-8');
        const lines = data.split('\n');

        // Reading lines and populating left and right arrays
        for (const line of lines) {
            // Split by one or more spaces (whitespace characters)
            const [a, b] = line.trim().split(/\s+/).map(Number);
            if (!isNaN(a) && !isNaN(b)) {
                left.push(a);
                right.push(b);
            }
        }

        // Sorting both arrays
        left.sort();
        right.sort();

        // Part 1: Calculate the sum of absolute differences
        const part1 = left.reduce((sum, a, index) => sum + Math.abs(a - right[index]), 0);
        console.log(part1);

        // Part 2: Count occurrences of each number in the right array
        const rightCounts = {};
        for (const b of right) {
            rightCounts[b] = (rightCounts[b] || 0) + 1;
        }

        // Calculate the weighted sum based on left array and counts in right array
        const part2 = left.reduce((sum, a) => sum + a * (rightCounts[a] || 0), 0);
        console.log(part2);

    } catch (err) {
        console.error('Error reading file:', err);
    }
}

main();
