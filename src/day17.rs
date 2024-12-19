use crate::utils::*;
use std::{array, fmt::Debug, hint};

const _SAMPLE: &str = "\
Register A: 202367025818154
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,4,7,1,4,0,3,5,5,3,0";

pub fn part1(input: &str) -> Answer {
    type Registers = [u64; 3];

    #[derive(Copy, Clone)]
    struct Op(u64, u64);

    impl Debug for Op {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0 {
                0 => write!(f, "ADV({})", self.1),
                1 => write!(f, "BXL({})", self.1),
                2 => write!(f, "BST({})", self.1),
                3 => write!(f, "JNZ({})", self.1),
                4 => write!(f, "BXC"),
                5 => write!(f, "OUT({})", self.1),
                6 => write!(f, "BDV({})", self.1),
                7 => write!(f, "CDV({})", self.1),
                _ => unreachable!(),
            }
        }
    }

    fn parse_debugger(input: &str) -> (Registers, Vec<Op>) {
        let mut lines = input.lines();
        let registers = array::from_fn(|_| parse(&lines.next().unwrap()[12..]).0);
        lines.next().unwrap();
        let program = &lines.next().unwrap()[9..];
        let mut octals = program
            .split(',')
            .map(|s| s.as_bytes()[0] - b'0')
            .map_into();
        let mut ops = vec![];
        while let Some(opcode) = octals.next() {
            ops.push(Op(opcode, octals.next().unwrap()));
        }
        (registers, ops)
    }

    fn get_combo(combo: u64, [a, b, c]: Registers) -> u64 {
        match combo {
            0..=3 => combo,
            4 => a,
            5 => b,
            6 => c,
            7.. => unsafe { hint::unreachable_unchecked() },
        }
    }

    enum Out {
        Output(u64),
        Jump(usize),
        None,
    }

    fn apply(Op(opcode, operand): Op, registers: &mut Registers) -> Out {
        match opcode {
            0 => {
                let combo = get_combo(operand, *registers);
                let [a, ..] = registers;
                *a = *a / 2u64.pow(combo.try_into().unwrap());
                Out::None
            }
            1 => {
                let [_, b, ..] = registers;
                *b ^= operand;
                Out::None
            }
            2 => {
                let combo = get_combo(operand, *registers);
                let [_, b, ..] = registers;
                *b = combo % 8;
                Out::None
            }
            3 => {
                let &mut [a, ..] = registers;
                if a != 0 {
                    Out::Jump(operand as usize / 2)
                } else {
                    Out::None
                }
            }
            4 => {
                let [.., b, c] = registers;
                *b ^= *c;
                Out::None
            }
            5 => {
                let combo = get_combo(operand, *registers);
                Out::Output(combo % 8)
            }
            6 => {
                let combo = get_combo(operand, *registers);
                let [a, b, ..] = registers;
                *b = *a / 2u64.pow(combo.try_into().unwrap());
                Out::None
            }
            7 => {
                let combo = get_combo(operand, *registers);
                let [a, .., c] = registers;
                *c = *a / 2u64.pow(combo.try_into().unwrap());
                Out::None
            }
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    let (mut registers, ops) = parse_debugger(input);
    let mut pc = 0;
    let mut outputs = vec![];
    while pc < ops.len() {
        match apply(ops[pc], &mut registers) {
            Out::Output(output) => {
                outputs.push(output);
                pc += 1;
            }
            Out::Jump(jump) => pc = jump,
            Out::None => pc += 1,
        }
    }
    outputs.iter().join(",").into()
}

pub fn part2(input: &str) -> Answer {
    fn find_a(a: u64, expected: &[u64]) -> Option<u64> {
        if let [expect_b, expected @ ..] = expected {
            (0..8).map(|i| a + i).find_map(|a| {
                let b = (a % 8) ^ 1;
                let c = a >> b;
                if (b ^ c ^ 4) % 8 == *expect_b {
                    find_a(a * 8, expected)
                } else {
                    None
                }
            })
        } else {
            Some(a)
        }
    }

    let (_, program) = input.split_once("\n\n").unwrap();
    let program = &program[9..];
    let outputs = program
        .split(',')
        .rev()
        .map(|output| output.parse().unwrap())
        .collect_vec();

    (find_a(0, &outputs).unwrap() / 8).into()
}

// pub fn part2_attempt1(input: &str) -> Answer {
//     type Registers = [Option<u64>; 3];

//     #[derive(Default, Debug)]
//     enum From {
//         Start,
//         #[default]
//         Prev,
//         Jump(usize),
//     }

//     #[derive(Debug)]
//     struct Node {
//         op: Op,
//         from: TinyVec<[From; 2]>,
//     }

//     fn build_graph(input: &str) -> (Vec<Node>, Vec<u64>) {
//         let (_, ops) = parse_debugger(input);
//         let expected = ops
//             .iter()
//             .rev()
//             .flat_map(|&Op(opcode, operand)| [operand, opcode])
//             .collect();
//         let mut nodes = ops
//             .into_iter()
//             .map(|op| Node {
//                 op,
//                 from: tiny_vec![{ Default::default() }],
//             })
//             .collect_vec();

//         nodes[0].from[0] = From::Start;
//         for i in 0..nodes.len() {
//             if let Op(3, to) = nodes[i].op {
//                 nodes[to as usize].from.push(From::Jump(i));
//             }
//         }

//         (nodes, expected)
//     }

//     fn recurse(
//         graph: &[Node],
//         (pc, jumped): (usize, bool),
//         registers: Registers,
//         expected: &[u64],
//     ) -> Option<u64> {
//         if pc == graph.len() {
//             (expected.is_empty()
//                 && registers[1].is_none_or(|b| b == 0)
//                 && registers[2].is_none_or(|c| c == 0))
//             .then_some(registers[0].unwrap_or(0))
//         } else {
//             let mut steps = graph[pc].from.iter().map(|from| match from {
//                 From::Start => (graph.len(), false),
//                 From::Prev => (pc - 1, false),
//                 &From::Jump(to) => (to, true),
//             });

//             match (graph[pc].op, registers) {
//                 (Op(0, operand @ 0..=3), [Some(a), b, c]) => {
//                     let denom = 2u64.pow(operand as _);
//                     let a = a * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| recurse(graph, step, [Some(a), b, c], expected))
//                 }
//                 (Op(0, 4), [Some(0), b, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [None, b, c], expected))
//                 }
//                 (Op(0, 5), [Some(a), Some(b), c]) => {
//                     let denom = 2u64.pow(b as _);
//                     let a = a * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| recurse(graph, step, [Some(a), Some(b), c], expected))
//                 }
//                 (Op(0, 6), [Some(a), b, Some(c)]) => {
//                     let denom = 2u64.pow(c as _);
//                     let a = a * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| recurse(graph, step, [Some(a), b, Some(c)], expected))
//                 }
//                 (Op(1, operand), [a, Some(b), c]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, Some(b ^ operand), c], expected))
//                 }
//                 (Op(1, _), [a, _, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, c], expected))
//                 }
//                 (Op(2, operand @ 0..=3), [a, Some(b @ 0..=3), c]) if operand == b => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, c], expected))
//                 }
//                 (Op(2, 0..=3), [a, None, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, c], expected))
//                 }
//                 (Op(2, 4), [Some(a), Some(b @ 0..8), c]) if a % 8 == b => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), None, c], expected))
//                 }
//                 (Op(2, 4), [Some(a), None, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), None, c], expected))
//                 }
//                 (Op(2, 4), [None, Some(b @ 0..8), c]) => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(graph, step, [Some(0 * i + b), None, c], expected)
//                     // })
//                     steps.find_map(|step| recurse(graph, step, [Some(b), None, c], expected))
//                 }
//                 (Op(2, 4), [None, None, c]) => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(graph, step, [Some(0 * i + b), None, c], expected)
//                     // })
//                     steps.find_map(|step| recurse(graph, step, [None, None, c], expected))
//                 }
//                 (Op(2, 5), [a, Some(b @ 0..8), c]) => {
//                     // (0..)
//                     // .cartesian_product(steps)
//                     // .find_map(|(i, step)| recurse(graph, step, [a, Some(0 * i + b), c], expected))
//                     steps.find_map(|step| recurse(graph, step, [a, Some(b), c], expected))
//                 }
//                 (Op(2, 5), [a, None, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, c], expected))
//                 }
//                 (Op(2, 6), [a, Some(b @ 0..8), Some(c)]) if c % 8 == b => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, Some(c)], expected))
//                 }
//                 (Op(2, 6), [a, None, Some(c)]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, Some(c)], expected))
//                 }
//                 (Op(2, 6), [a, Some(b @ 0..8), None]) => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(graph, step, [Some(0 * i + b), None, c], expected)
//                     // })
//                     steps.find_map(|step| recurse(graph, step, [a, None, Some(b)], expected))
//                 }
//                 (Op(2, 6), [a, None, None]) => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(graph, step, [Some(0 * i + b), None, c], expected)
//                     // })
//                     steps.find_map(|step| recurse(graph, step, [a, None, None], expected))
//                 }
//                 (Op(3, _), [Some(a), b, c]) if jumped && a != 0 => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), b, c], expected))
//                 }
//                 (Op(3, _), [None, b, c]) if jumped => {
//                     // what if a is 0?
//                     steps.find_map(|step| recurse(graph, step, [None, b, c], expected))
//                 }
//                 (Op(3, _), [_, b, c]) if !jumped => {
//                     steps.find_map(|step| recurse(graph, step, [Some(0), b, c], expected))
//                 }
//                 (Op(4, _), [a, Some(b), Some(c)]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, Some(b ^ c), Some(c)], expected))
//                 }
//                 // (Op(4, _), [a, Some(b), None]) => {
//                 //     (0..).cartesian_product(steps).find_map(|(i, step)| {

//                 //          recurse(graph, step, [a, Some(b ^ i), Some(i)], expected)
//                 //     })
//                 // }
//                 (Op(4, _), [a, None, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, c], expected))
//                 }
//                 (Op(5, operand @ 0..=3), registers) if expected.first() == Some(&operand) => {
//                     steps.find_map(|step| recurse(graph, step, registers, &expected[1..]))
//                 }
//                 (Op(5, 4), [Some(a), b, c]) if expected.first() == Some(&(a % 8)) => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), b, c], &expected[1..]))
//                 }
//                 (Op(5, 4), [None, b, c]) if !expected.is_empty() => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(
//                     //         graph,
//                     //         step,
//                     //         [Some(i * 8 + expected[0]), b, c],
//                     //         &expected[1..],
//                     //     )
//                     // })
//                     steps.find_map(|step| {
//                         recurse(graph, step, [Some(expected[0]), b, c], &expected[1..])
//                     })
//                 }
//                 (Op(5, 5), [a, Some(b), c]) if expected.first() == Some(&(b % 8)) => {
//                     steps.find_map(|step| recurse(graph, step, [a, Some(b), c], &expected[1..]))
//                 }
//                 (Op(5, 5), [a, None, c]) if !expected.is_empty() => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(
//                     //         graph,
//                     //         step,
//                     //         [a, Some(i * 8 + expected[0]), c],
//                     //         &expected[1..],
//                     //     )
//                     // })
//                     steps.find_map(|step| {
//                         recurse(graph, step, [a, Some(expected[0]), c], &expected[1..])
//                     })
//                 }
//                 (Op(5, 6), [a, b, Some(c)]) if expected.first() == Some(&(c % 8)) => {
//                     steps.find_map(|step| recurse(graph, step, [a, b, Some(c)], &expected[1..]))
//                 }
//                 (Op(5, 6), [a, b, None]) if !expected.is_empty() => {
//                     // (0..).cartesian_product(steps).find_map(|(i, step)| {
//                     //     recurse(
//                     //         graph,
//                     //         step,
//                     //         [a, b, Some(i * 8 + expected[0])],
//                     //         &expected[1..],
//                     //     )
//                     // })
//                     steps.find_map(|step| {
//                         recurse(graph, step, [a, b, Some(expected[0])], &expected[1..])
//                     })
//                 }
//                 (Op(6, operand @ 0..=3), [Some(a), Some(b), c])
//                     if a / 2u64.pow(operand as _) == b =>
//                 {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), None, c], expected))
//                 }
//                 (Op(6, operand @ 0..=3), [None, Some(b), c]) => {
//                     let denom = 2u64.pow(operand as _);
//                     let a = b * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| recurse(graph, step, [Some(a), None, c], expected))
//                 }
//                 (Op(6, 4), [Some(a), Some(0), c]) => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), None, c], expected))
//                 }
//                 (Op(6, 6), [Some(a), Some(b), Some(c)]) if a / 2u64.pow(c as _) == b => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), None, Some(c)], expected))
//                 }
//                 (Op(6, 6), [None, Some(b), Some(c)]) => {
//                     let denom = 2u64.pow(c as _);
//                     let a = b * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| {
//                             recurse(graph, step, [Some(a), None, Some(c)], expected)
//                         })
//                 }
//                 (Op(6, _), [a, None, c]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, None, c], expected))
//                 }
//                 (Op(7, operand @ 0..=3), [Some(a), b, Some(c)])
//                     if a / 2u64.pow(operand as _) == c =>
//                 {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), b, None], expected))
//                 }
//                 (Op(7, operand @ 0..=3), [None, b, Some(c)]) => {
//                     let denom = 2u64.pow(operand as _);
//                     let a = c * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| recurse(graph, step, [Some(a), b, None], expected))
//                 }
//                 (Op(7, 4), [Some(a), b, Some(0)]) => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), b, None], expected))
//                 }
//                 (Op(7, 5), [Some(a), Some(b), Some(c)]) if a / 2u64.pow(b as _) == c => {
//                     steps.find_map(|step| recurse(graph, step, [Some(a), Some(b), None], expected))
//                 }
//                 (Op(7, 5), [None, Some(b), Some(c)]) => {
//                     let denom = 2u64.pow(b as _);
//                     let a = c * denom;
//                     (a..a + denom)
//                         .cartesian_product(steps)
//                         .find_map(|(a, step)| {
//                             recurse(graph, step, [Some(a), Some(b), None], expected)
//                         })
//                 }
//                 (Op(7, _), [a, b, None]) => {
//                     steps.find_map(|step| recurse(graph, step, [a, b, None], expected))
//                 }
//                 _ => {
//                     if let Some(Node { op, .. }) = graph.get(pc) {
//                         println!("\n({pc}) {op:?} with registers {registers:?}\n(expecting {expected:?})\n",);
//                     }
//                     None
//                 }
//             }
//         }
//     }

//     let (graph, expected) = build_graph(input);
//     recurse(&graph, (graph.len() - 1, false), [None; 3], &expected)
//         .unwrap()
//         .into()
// }
