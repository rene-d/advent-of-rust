#!/usr/bin/env python3

import sys

data = open(sys.argv[1] if len(sys.argv) > 1 else "input.txt").read()

packet = "".join(f"{int(d,16):04b}" for d in data.strip())

part1 = 0


def read_packet(packet, offset, indent=""):
    global part1

    if offset >= len(packet):
        return

    # print(packet[offset:])

    version = int(packet[offset : offset + 3], 2)
    type_id = int(packet[offset + 3 : offset + 6], 2)
    offset += 6

    part1 += version

    if type_id == 4:
        literal_value = 0
        while True:
            last = packet[offset : offset + 1]
            nibble = int(packet[offset + 1 : offset + 5], 2)
            literal_value = (literal_value * 16) + nibble
            offset += 5
            if last == "0":
                break

        # print(f"{indent}version={version} type_id={type_id} literal_value={literal_value}")

        return offset, literal_value

    # length of subpackets
    length_id = int(packet[offset : offset + 1], 2)
    offset += 1

    length, count = 0, 0
    if length_id == 0:
        length = int(packet[offset : offset + 15], 2)
        offset += 15
        # print(f"{indent}version={version} type_id={type_id} length={length}")
    else:
        count = int(packet[offset : offset + 11], 2)
        offset += 11
        # print(f"{indent}version={version} type_id={type_id} count={count}")

    end_offset = offset + length

    # subpackets
    values = []
    while (length != 0 and offset < end_offset) or (length == 0 and count > 0):
        offset, value = read_packet(packet, offset, indent + "  ")
        values.append(value)
        count -= 1

    computed = 0

    if type_id == 0:
        # Packets with type ID 0 are sum packets
        computed = sum(values)

    elif type_id == 1:
        # Packets with type ID 1 are product packets
        computed = 1
        for v in values:
            computed *= v

    elif type_id == 2:
        # Packets with type ID 2 are minimum packets
        computed = min(values)

    elif type_id == 3:
        # Packets with type ID 3 are maximum packets
        computed = max(values)

    elif type_id == 5:
        # Packets with type ID 5 are greater than packets
        computed = 1 if values[0] > values[1] else 0

    elif type_id == 6:
        # Packets with type ID 6 are less than packets
        computed = 1 if values[0] < values[1] else 0

    elif type_id == 7:
        # Packets with type ID 7 are equal to packets
        computed = 1 if values[0] == values[1] else 0

    return offset, computed


_, part2 = read_packet(packet, 0)
print(part1)
print(part2)
