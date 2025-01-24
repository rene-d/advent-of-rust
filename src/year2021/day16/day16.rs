//! [Day 16: Packet Decoder](https://adventofcode.com/2021/day/16)

fn int_value(bits: &str) -> u32 {
    bits.chars()
        .map_while(|c| c.to_digit(2))
        .fold(0, |acc, d| (acc << 1) + d)
}

fn read_packet(packet: &str, mut offset: usize, sum_of_versions: &mut u32) -> (usize, u64) {
    assert!(offset < packet.len());

    let version = int_value(&packet[offset..offset + 3]);
    let type_id = int_value(&packet[offset + 3..offset + 6]);
    offset += 6;

    // compute part 1 answer
    *sum_of_versions += version;

    //
    if type_id == 4 {
        let mut literal_value = 0;
        loop {
            let last = &packet[offset..=offset];
            let nibble = int_value(&packet[offset + 1..offset + 5]);
            literal_value = (literal_value * 16) + u64::from(nibble);
            offset += 5;
            if offset > packet.len() || last == "0" {
                return (offset, literal_value);
            }
        }
    }

    // subpackets
    let mut values = Vec::new();

    // length of subpackets
    let length_id = int_value(&packet[offset..=offset]);
    offset += 1;

    if length_id == 0 {
        //
        let length = int_value(&packet[offset..offset + 15]);
        offset += 15;

        let end_offset: usize = offset + usize::try_from(length).unwrap();

        while length != 0 && offset < end_offset {
            let (new_offset, value) = read_packet(packet, offset, sum_of_versions);
            values.push(value);
            offset = new_offset;
        }
    } else {
        let mut count = int_value(&packet[offset..offset + 11]);
        offset += 11;

        while count > 0 && offset < packet.len() {
            let (new_offset, value) = read_packet(packet, offset, sum_of_versions);
            values.push(value);
            count -= 1;
            offset = new_offset;
        }
    };

    let computed = match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => u64::from(values[0] > values[1]),
        6 => u64::from(values[0] < values[1]),
        7 => u64::from(values[0] == values[1]),
        _ => panic!(),
    };

    (offset, computed)
}

/// # Panics
#[must_use]
pub fn solve(data: &str) -> (u32, u64) {
    let bin_data: String = data
        .chars()
        .map_while(|c| c.to_digit(16))
        .map(|d| match d {
            0 => "0000",
            1 => "0001",
            2 => "0010",
            3 => "0011",
            4 => "0100",
            5 => "0101",
            6 => "0110",
            7 => "0111",
            8 => "1000",
            9 => "1001",
            10 => "1010",
            11 => "1011",
            12 => "1100",
            13 => "1101",
            14 => "1110",
            15 => "1111",
            _ => panic!(),
        })
        .collect::<Vec<_>>()
        .join("");

    let mut part1 = 0;
    let (_, part2) = read_packet(&bin_data, 0, &mut part1);

    (part1, part2)
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bin() {
        assert_eq!(int_value("a"), 0);
        assert_eq!(int_value("0"), 0);
        assert_eq!(int_value("01"), 1);
        assert_eq!(int_value("11"), 3);
        assert_eq!(int_value("10000"), 16);
        assert_eq!(int_value("10101010101010101010101010101010"), 0xAAAAAAAA);
    }

    #[test]
    fn test_part1() {
        let puzzle = solve("8A004A801A8002F478");
        assert_eq!(puzzle.0, 16);

        let puzzle = solve("620080001611562C8802118E34");
        assert_eq!(puzzle.0, 12);

        let puzzle = solve("C0015000016115A2E0802F182340");
        assert_eq!(puzzle.0, 23);

        let puzzle = solve("A0016C880162017C3686B18A3D4780");
        assert_eq!(puzzle.0, 31);
    }

    #[test]
    fn test_part2() {
        let puzzle = solve("C200B40A82");
        assert_eq!(puzzle.1, 3);

        let puzzle = solve("04005AC33890");
        assert_eq!(puzzle.1, 54);

        let puzzle = solve("880086C3E88112");
        assert_eq!(puzzle.1, 7);

        let puzzle = solve("CE00C43D881120");
        assert_eq!(puzzle.1, 9);

        let puzzle = solve("D8005AC2A8F0");
        assert_eq!(puzzle.1, 1);

        let puzzle = solve("F600BC2D8F");
        assert_eq!(puzzle.1, 0);

        let puzzle = solve("9C005AC2F8F0");
        assert_eq!(puzzle.1, 0);

        let puzzle = solve("9C0141080250320F1802104A08");
        assert_eq!(puzzle.1, 1);
    }
}
