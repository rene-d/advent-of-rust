//! [Day 23: Category Six](https://adventofcode.com/2019/day/23)

use std::collections::{HashSet, VecDeque};

use intcode::{Computer, State};

enum SchedState {
    Idle,
    Active,
}

struct Message {
    dest_id: i64,
    x: i64,
    y: i64,
}

struct SchedResult {
    messages: VecDeque<Message>,
    state: SchedState,
}

impl SchedResult {
    const fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            state: SchedState::Active,
        }
    }
    fn push(&mut self, dest_id: i64, x: i64, y: i64) {
        self.messages.push_back(Message { dest_id, x, y });
    }
}

trait Task {
    fn sched(&mut self, idle_count: usize) -> SchedResult;
    fn receive(&mut self, x: i64, y: i64);
}

struct Network {
    nodes: Vec<Node>,
    nat: Nat,
    idle: HashSet<usize>,
}

impl Network {
    fn new(program: &str) -> Self {
        let mut nodes = Vec::new();
        let computer = Computer::load(program);
        for id in 0..50 {
            nodes.push(Node::new(id, &computer));
        }

        Self {
            nodes,
            nat: Nat::new(),
            idle: HashSet::new(),
        }
    }

    fn run(&mut self) -> (i64, i64) {
        let mut part1 = 0;
        let mut idle_y = HashSet::new();

        let mut id = 0;

        loop {
            let idle_count = self.idle.len();
            let a = if id == 255 {
                self.nat.sched(idle_count)
            } else {
                self.nodes[id].sched(idle_count)
            };

            match a.state {
                SchedState::Active => {
                    self.idle.remove(&id);
                }
                SchedState::Idle => {
                    self.idle.insert(id);
                }
            };

            for m in a.messages {
                // deal with part 2
                if id == 255 && idle_count == 50 && !idle_y.insert(m.y) {
                    return (part1, m.y);
                }

                if m.dest_id == 255 {
                    self.nat.receive(m.x, m.y);

                    // deal with part 1
                    if part1 == 0 {
                        part1 = m.y;
                    }
                } else {
                    let dest_idx = usize::try_from(m.dest_id).unwrap();
                    self.nodes[dest_idx].receive(m.x, m.y);
                }
            }

            id = match id {
                0..=48 => id + 1,
                49 => 255,
                255 => 0,
                _ => unreachable!(),
            };
        }
    }
}

struct Nat {
    last_x: i64,
    last_y: i64,
    input: VecDeque<i64>,
}
impl Nat {
    const fn new() -> Self {
        Self {
            last_x: 0,
            last_y: 0,
            input: VecDeque::new(),
        }
    }
}

impl Task for Nat {
    fn sched(&mut self, idle_count: usize) -> SchedResult {
        while self.input.len() >= 2 {
            self.last_x = self.input.pop_front().unwrap();
            self.last_y = self.input.pop_front().unwrap();
        }

        let mut result = SchedResult::new();

        if idle_count == 50 {
            // if !self.idle_y.insert(self.last_y) {
            //     result.state = SchedState::Halt;
            // } else {
            result.push(0, self.last_x, self.last_y);
            // }
        }

        result
    }

    fn receive(&mut self, x: i64, y: i64) {
        self.input.push_back(x);
        self.input.push_back(y);
    }
}

struct Node {
    id: i64,
    computer: Computer,
    output: VecDeque<i64>,
}

impl Node {
    fn new(id: i64, computer: &Computer) -> Self {
        let mut computer = computer.clone();
        computer.push(id);
        let state = computer.run();
        assert_eq!(state, State::Input);

        Self {
            id,
            computer: computer.clone(),
            output: VecDeque::new(),
        }
    }
}

impl Task for Node {
    fn sched(&mut self, _idle_count: usize) -> SchedResult {
        let nb_recv = if self.computer.input_len() == 0 {
            self.computer.push(-1);
            0
        } else {
            self.computer.input_len() / 3
        };

        while let State::Output(num) = self.computer.run() {
            self.output.push_back(num);
        }

        let mut result = SchedResult::new();

        while self.output.len() >= 3 {
            result.push(
                self.output.pop_front().unwrap(),
                self.output.pop_front().unwrap(),
                self.output.pop_front().unwrap(),
            );
        }

        result.state = if nb_recv == 0 {
            SchedState::Idle
        } else {
            SchedState::Active
        };

        result
    }

    fn receive(&mut self, x: i64, y: i64) {
        self.computer.push(self.id);
        self.computer.push(0);
        self.computer.push(x);
        self.computer.push(y);
    }
}

/// # Panics
/// over malformed input
#[must_use]
pub fn solve(data: &str) -> (i64, i64) {
    Network::new(data).run()
}

pub fn main() {
    let args = aoc::parse_args();
    args.run(solve);
}
