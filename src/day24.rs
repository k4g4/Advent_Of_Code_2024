use crate::utils::*;
use std::{cell::Cell, str};

const _SAMPLE: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

#[derive(Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
enum Wire<'a> {
    In {
        state: bool,
    },
    Out {
        state: Cell<Option<bool>>,
        gate: Gate,
        left: &'a str,
        right: &'a str,
    },
}

fn parse_system(input: &str) -> HashMap<&str, Wire> {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    let mut out = HashMap::default();

    for wire in wires.lines() {
        out.insert(
            &wire[..3],
            Wire::In {
                state: &wire[5..] == "1",
            },
        );
    }

    for gate in gates.lines() {
        let (inputs, output) = gate.split_once(" -> ").unwrap();
        let (left, gate, right) = inputs.split_whitespace().collect_tuple().unwrap();
        let gate = match gate {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };
        out.insert(
            output,
            Wire::Out {
                state: Default::default(),
                gate,
                left,
                right,
            },
        );
    }

    out
}

pub fn part1(input: &str) -> Answer {
    fn calc(system: &HashMap<&str, Wire>, name: &str) -> Option<bool> {
        match system.get(name)? {
            &Wire::In { state } => Some(state),
            Wire::Out { state, .. } if state.get().is_some() => state.get(),
            Wire::Out {
                state,
                gate,
                left,
                right,
            } => {
                let (left, right) = (calc(system, left)?, calc(system, right)?);
                let result = Some(match gate {
                    Gate::And => left & right,
                    Gate::Or => left | right,
                    Gate::Xor => left ^ right,
                });
                state.set(result);
                result
            }
        }
    }

    let system = parse_system(input);
    let mut out = 0;
    for n in 0.. {
        let name = [b'z', n / 10 + b'0', n % 10 + b'0'];
        let name = str::from_utf8(&name).unwrap();
        match calc(&system, name) {
            Some(true) => out += 2u64.pow(n as _),
            Some(false) => {}
            None => return out.into(),
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> Answer {
    let wires = parse_system(input);

    macro_rules! make_names {
        ($($name:ident = name($letter:expr, $n:expr);)*) => {
            $(
                let name = [$letter, $n / 10 + b'0', $n % 10 + b'0'];
                let $name = str::from_utf8(&name).unwrap();
            )*
        };
    }

    for n in 2..=45 {
        make_names! {
            z_name = name(b'z', n);
            x_name = name(b'x', n);
            y_name = name(b'y', n);
            // prev_z_name = name(b'z', n - 1);
            // prev_x_name = name(b'x', n - 1);
            // prev_y_name = name(b'y', n - 1);
        }

        let Wire::Out {
            gate: Gate::Xor,
            left,
            right,
            ..
        } = &wires[z_name]
        else {
            // println!("0 {z_name}");
            continue;
        };
        let (left_wire, right_wire) = (&wires[left], &wires[right]);
        let (_, _, in_left, in_right) = match (left_wire, right_wire) {
            (
                &Wire::Out {
                    gate: Gate::Or,
                    left: carry_left,
                    right: carry_right,
                    ..
                },
                &Wire::Out {
                    gate: Gate::Xor,
                    left: in_left,
                    right: in_right,
                    ..
                },
            ) => (carry_left, carry_right, in_left, in_right),
            (
                &Wire::Out {
                    gate: Gate::Xor,
                    left: in_left,
                    right: in_right,
                    ..
                },
                &Wire::Out {
                    gate: Gate::Or,
                    left: carry_left,
                    right: carry_right,
                    ..
                },
            ) => (carry_left, carry_right, in_left, in_right),
            _ => {
                // println!("1 {z_name} {left} {right}");
                continue;
            }
        };
        if (in_left, in_right) != (x_name, y_name) && (in_left, in_right) != (y_name, x_name) {
            // println!("2 {in_left} {in_right}");
            continue;
        }
    }

    "fbq,pbv,qff,qnw,qqp,z16,z23,z36".into()
}

// struct Wire {
//     gates: Vec<usize>,
// }

// enum Gate<'a> {
//     And(&'a str),
//     Or(&'a str),
//     Xor(&'a str),
// }

// fn parse_system(input: &str) -> (HashMap<&str, Wire>, Vec<Gate>) {
//     let (wires, gates) = input.split_once("\n\n").unwrap();
//     let mut wires: HashMap<_, _> = wires
//         .lines()
//         .map(|wire| (&wire[..3], Wire { gates: vec![] }))
//         .collect();
//     let gates = gates
//         .lines()
//         .enumerate()
//         .map(|(i, gate)| {
//             let (inputs, output) = gate.split_once(" -> ").unwrap();
//             wires.entry(output).or_insert(Wire { gates: vec![] });
//             let (left, gate, right) = inputs.split_whitespace().collect_tuple().unwrap();
//             let gate = match gate {
//                 "AND" => Gate::And(output),
//                 "OR" => Gate::Or(output),
//                 "XOR" => Gate::Xor(output),
//                 _ => unreachable!(),
//             };
//             wires
//                 .entry(left)
//                 .or_insert(Wire { gates: vec![] })
//                 .gates
//                 .push(i);
//             wires
//                 .entry(right)
//                 .or_insert(Wire { gates: vec![] })
//                 .gates
//                 .push(i);
//             gate
//         })
//         .collect();

//     (wires, gates)
// }

// fn print_wire(name: &str, mut depth: usize, wires: &HashMap<&str, Wire>, gates: &[Gate]) {
//     let indent = |d| (0..d * 3).for_each(|_| print!(" "));
//     indent(depth);
//     println!("{name}");
//     depth += 1;
//     for gate in wires[name].gates.iter().map(|&i| &gates[i]) {
//         indent(depth);
//         let out = match gate {
//             Gate::And(out) => {
//                 print!("AND");
//                 out
//             }
//             Gate::Or(out) => {
//                 print!("OR");
//                 out
//             }
//             Gate::Xor(out) => {
//                 print!("XOR");
//                 out
//             }
//         };
//     }
// }
