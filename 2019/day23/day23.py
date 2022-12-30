#!/usr/bin/env python3
# https://adventofcode.com/2019/day/23

from pathlib import Path
import sys
from copy import deepcopy
from collections import defaultdict, deque
import itertools


sys.path.append("..")
from intcode.Intcode import Computer


filename = sys.argv[1] if len(sys.argv) > 1 else "input.txt"
software = Path(filename).read_text()


class Node:

    network = {}
    idle = set()

    def __init__(self, id):
        self.network[id] = self
        self.id = id

        self.computer = Computer()

        # NAT is not a true Intcode computer (software segfault...)
        if id == 255:
            self.last_X = 0  # the last NAT message received
            self.last_Y = 0
            self.idle_Y = set()
        else:
            self.computer.load(software)
            self.computer.input.append(id)
            state = self.computer.run()
            assert state == "read" and len(self.computer.output) == 0

    def sched(self, part):

        answer = None

        if self.id == 255:
            while len(self.computer.input) > 0:
                id = self.computer.input.popleft()  # seems to be always 24
                _ = self.computer.input.popleft()  # weird, see below
                X = self.computer.input.popleft()
                Y = self.computer.input.popleft()

                self.last_X = X
                self.last_Y = Y

            # if ell nodes are idle
            if len(self.idle) == 50:
                self.send_to(0, self.last_X, self.last_Y)

                if part == 2:
                    if self.last_Y in self.idle_Y:
                        return self.last_Y
                    self.idle_Y.add(self.last_Y)

            return

        if len(self.computer.input) == 0:
            self.computer.input.append(-1)
            nb_recv = 0
        else:
            nb_recv = len(self.computer.input) // 3

        self.computer.resume()

        nb_to_send = len(self.computer.output) // 3

        while len(self.computer.output) != 0:
            dest_id = self.computer.output.popleft()
            X = self.computer.output.popleft()
            Y = self.computer.output.popleft()

            if part == 1 and dest_id == 255 and not answer:
                # if we exit here, part2 can't be chained
                answer = Y

            self.send_to(dest_id, X, Y)

        if nb_recv == 0 and nb_to_send == 0:
            self.idle.add(self.id)
        else:
            self.idle.discard(self.id)

        return answer

    def send_to(self, dest_id, X, Y):

        # Second arg is weird and mysterious: value seems to be ignored by the Intcode software.
        # Moreover, it doesn't work if I send only the 3 expected values (id,X,Y).
        # Found by accident during a code refactoring, that obviously did not work before.
        # Since then, I have reread the puzzle statement several times
        # and I still don't understand.
        self.network[dest_id].computer.input.extend((self.id, 0, X, Y))


nodes = [Node(id) for id in range(50)]
nodes.append(Node(255))  # add the NAT device

for node in itertools.cycle(nodes):
    Y = node.sched(1)
    if Y:
        print(Y)
        break

for node in itertools.cycle(nodes):
    Y = node.sched(2)
    if Y:
        print(Y)
        break
