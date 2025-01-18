import Foundation

// Main function to handle file reading and calculations
func main() {
    // Default input file or the one provided as a command line argument
    let inputFile = CommandLine.arguments.count >= 2 ? CommandLine.arguments[1] : "input.txt"

    var left: [Int] = []
    var right: [Int] = []

    do {
        // Read the file content
        let data = try String(contentsOfFile: inputFile, encoding: .utf8)
        let lines = data.split(separator: "\n")

        // Read lines and populate the left and right arrays
        for line in lines {
            let components = line.split(whereSeparator: { $0.isWhitespace }).map { Int($0.trimmingCharacters(in: .whitespaces)) }
            if components.count == 2, let a = components[0], let b = components[1] {
                left.append(a)
                right.append(b)
            }
        }

        // Sorting both arrays
        left.sort()
        right.sort()

        // Part 1: Calculate the sum of absolute differences
        let part1 = zip(left, right).reduce(0) { sum, pair in
            sum + abs(pair.0 - pair.1)
        }
        print(part1)

        // Part 2: Count occurrences of each number in the right array
        var rightCounts: [Int: Int] = [:]
        for b in right {
            rightCounts[b, default: 0] += 1
        }

        // Calculate the weighted sum based on left array and counts in right array
        let part2 = left.reduce(0) { sum, a in
            sum + a * (rightCounts[a] ?? 0)
        }
        print(part2)

    } catch {
        print("Error reading file: \(error)")
    }
}

main()
