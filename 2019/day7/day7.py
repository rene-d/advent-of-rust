#!/usr/bin/env python3
# https://adventofcode.com/2019/day/7

from pathlib import Path
import sys
import itertools

sys.path.append("..")
from intcode.Intcode import Computer


filename = "test.txt" if len(sys.argv) > 1 and sys.argv[1] == "-t" else "input.txt"
data = Path(filename).read_text()
lines = data.splitlines()


def run_amplifiers(phases):
    input_signal = 0
    for phase in phases:
        amp.input.append(phase)  # phase setting
        amp.input.append(input_signal)  # input signal
        amp.run()
        input_signal = amp.output.popleft()
    return input_signal


amp = Computer()

# tests from puzzle
amp.load("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
assert run_amplifiers([4, 3, 2, 1, 0]) == 43210

amp.load("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
assert run_amplifiers([0, 1, 2, 3, 4]) == 54321

amp.load("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
assert run_amplifiers([1, 0, 4, 3, 2]) == 65210


# part 1
amp.load_from("input.txt")
print(max(run_amplifiers(phases) for phases in itertools.permutations(range(5))))


# part 2
def run_feedback(program, phase):
    amps = []
    for i in range(5):
        amp = Computer()
        amp.load(program)
        amp.start(output_mode="yield")
        amps.append(amp)

    input_signal = 0

    for k in range(11):
        for i in range(5):
            if k == 0:
                amps[i].input.append(phase[i])
            amps[i].input.append(input_signal)
            state = amps[i].resume()
            if state == "halted":
                break
            input_signal = amps[i].output.popleft()
            assert len(amps[i].input) == 0
            assert len(amps[i].output) == 0
            # print("loop",k,"amp", i, state, "signal:", input_signal)

        if state == "halted":
            break

    return input_signal


# part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", phase=[9, 8, 7, 6, 5])

# part2(
#     "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
#     phase=[9, 7, 8, 5, 6],
# )

print(max(run_feedback(amp.program, phases) for phases in itertools.permutations(range(5, 10))))
