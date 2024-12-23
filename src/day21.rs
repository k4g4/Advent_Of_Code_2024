use crate::utils::*;
use std::{
    array,
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    hint,
};

const _SAMPLE: &str = "\
029A
980A
179A
456A
379A";

pub fn part1(input: &str) -> Answer {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
    struct Pos<const ROBOT: u8>(usize);

    const R1_0: Pos<1> = Pos(0);
    const R1_1: Pos<1> = Pos(1);
    const R1_2: Pos<1> = Pos(2);
    const R1_3: Pos<1> = Pos(3);
    const R1_4: Pos<1> = Pos(4);
    const R1_5: Pos<1> = Pos(5);
    const R1_6: Pos<1> = Pos(6);
    const R1_7: Pos<1> = Pos(7);
    const R1_8: Pos<1> = Pos(8);
    const R1_9: Pos<1> = Pos(9);
    const R1_A: Pos<1> = Pos(10);

    const R2_UP: Pos<2> = Pos(0);
    const R2_LEFT: Pos<2> = Pos(1);
    const R2_DOWN: Pos<2> = Pos(2);
    const R2_RIGHT: Pos<2> = Pos(3);
    const R2_A: Pos<2> = Pos(4);

    const R3_UP: Pos<3> = Pos(0);
    const R3_LEFT: Pos<3> = Pos(1);
    const R3_DOWN: Pos<3> = Pos(2);
    const R3_RIGHT: Pos<3> = Pos(3);
    const R3_A: Pos<3> = Pos(4);

    type Cost = u64;
    type Memo<const ROBOT: u8> = HashMap<(Pos<ROBOT>, Pos<ROBOT>), Cost>;

    struct Node<const ROBOT_A: u8, const ROBOT_B: u8> {
        pos_a: Pos<ROBOT_A>,
        pos_b: Pos<ROBOT_B>,
        pressed: bool,
        cost: Cost,
        seen: ArrayVec<[Pos<ROBOT_A>; 11]>,
    }

    impl<const ROBOT_A: u8, const ROBOT_B: u8> PartialEq for Node<ROBOT_A, ROBOT_B> {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }

    impl<const ROBOT_A: u8, const ROBOT_B: u8> Eq for Node<ROBOT_A, ROBOT_B> {}

    impl<const ROBOT_A: u8, const ROBOT_B: u8> PartialOrd for Node<ROBOT_A, ROBOT_B> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Reverse(self.cost).partial_cmp(&Reverse(other.cost))
        }
    }

    impl<const ROBOT_A: u8, const ROBOT_B: u8> Ord for Node<ROBOT_A, ROBOT_B> {
        fn cmp(&self, other: &Self) -> Ordering {
            Reverse(self.cost).cmp(&Reverse(other.cost))
        }
    }

    fn robot1(pos: Pos<1>, target: Pos<1>, memo1: &mut Memo<1>, memo2: &mut Memo<2>) -> Cost {
        const NUMPAD: [&[(Pos<1>, Pos<2>)]; 11] = [
            &[(R1_2, R2_UP), (R1_A, R2_RIGHT)],
            &[(R1_2, R2_RIGHT), (R1_4, R2_UP)],
            &[
                (R1_0, R2_DOWN),
                (R1_1, R2_LEFT),
                (R1_3, R2_RIGHT),
                (R1_5, R2_UP),
            ],
            &[(R1_2, R2_LEFT), (R1_6, R2_UP), (R1_A, R2_DOWN)],
            &[(R1_1, R2_DOWN), (R1_5, R2_RIGHT), (R1_7, R2_UP)],
            &[
                (R1_2, R2_DOWN),
                (R1_4, R2_LEFT),
                (R1_6, R2_RIGHT),
                (R1_8, R2_UP),
            ],
            &[(R1_3, R2_DOWN), (R1_5, R2_LEFT), (R1_9, R2_UP)],
            &[(R1_4, R2_DOWN), (R1_8, R2_RIGHT)],
            &[(R1_5, R2_DOWN), (R1_7, R2_LEFT), (R1_9, R2_RIGHT)],
            &[(R1_6, R2_DOWN), (R1_8, R2_LEFT)],
            &[(R1_0, R2_LEFT), (R1_3, R2_UP)],
        ];

        if pos == target {
            return 1;
        }
        if let Some(&cost) = memo1.get(&(pos, target)) {
            return cost;
        }

        let mut nodes = BinaryHeap::default();
        nodes.push(Node {
            pos_a: pos,
            pos_b: R2_A,
            pressed: false,
            cost: 0,
            seen: array_vec![],
        });
        loop {
            let Node {
                pos_a: r1_pos,
                pos_b: r2_pos,
                pressed,
                cost,
                seen,
            } = nodes.pop().unwrap();
            if r1_pos == target {
                if pressed {
                    memo1.insert((pos, target), cost);
                    break cost;
                }
                let cost = cost + robot2(r2_pos, R2_A, memo2);
                nodes.push(Node {
                    pos_a: r1_pos,
                    pos_b: r2_pos,
                    pressed: true,
                    cost,
                    seen,
                });
            }
            for &(r1_neighbor, r2_target) in NUMPAD[r1_pos.0] {
                if !seen.contains(&r1_neighbor) {
                    let cost = cost + robot2(r2_pos, r2_target, memo2);
                    let mut seen = seen.clone();
                    seen.push(r1_pos);
                    nodes.push(Node {
                        pos_a: r1_neighbor,
                        pos_b: r2_target,
                        pressed,
                        cost,
                        seen,
                    });
                }
            }
        }
    }

    fn robot2(pos: Pos<2>, target: Pos<2>, memo: &mut Memo<2>) -> Cost {
        const DIRPAD: [&[(Pos<2>, Pos<3>)]; 5] = {
            [
                &[(R2_DOWN, R3_DOWN), (R2_A, R3_RIGHT)],
                &[(R2_DOWN, R3_RIGHT)],
                &[(R2_UP, R3_UP), (R2_LEFT, R3_LEFT), (R2_RIGHT, R3_RIGHT)],
                &[(R2_DOWN, R3_LEFT), (R2_A, R3_UP)],
                &[(R2_UP, R3_LEFT), (R2_RIGHT, R3_DOWN)],
            ]
        };

        if pos == target {
            return 1;
        }
        if let Some(&cost) = memo.get(&(pos, target)) {
            return cost;
        }

        let mut nodes = BinaryHeap::default();
        nodes.push(Node {
            pos_a: pos,
            pos_b: R3_A,
            pressed: false,
            cost: 0,
            seen: array_vec![],
        });
        loop {
            let Node {
                pos_a: r2_pos,
                pos_b: r3_pos,
                pressed,
                cost,
                seen,
            } = nodes.pop().unwrap();
            if r2_pos == target {
                if pressed {
                    memo.insert((pos, target), cost);
                    break cost;
                }
                let cost = cost + robot3(r3_pos, R3_A);
                nodes.push(Node {
                    pos_a: r2_pos,
                    pos_b: r3_pos,
                    pressed: true,
                    cost,
                    seen,
                });
            }
            for &(r2_neighbor, r3_target) in DIRPAD[r2_pos.0] {
                if !seen.contains(&r2_neighbor) {
                    let cost = cost + robot3(r3_pos, r3_target);
                    let mut seen = seen.clone();
                    seen.push(r2_pos);
                    nodes.push(Node {
                        pos_a: r2_neighbor,
                        pos_b: r3_target,
                        pressed,
                        cost,
                        seen,
                    });
                }
            }
        }
    }

    fn robot3(pos: Pos<3>, target: Pos<3>) -> Cost {
        match (pos, target) {
            (R3_UP, R3_UP) => 1,
            (R3_UP, R3_LEFT) => 3,
            (R3_UP, R3_DOWN) => 2,
            (R3_UP, R3_RIGHT) => 3,
            (R3_UP, R3_A) => 2,
            (R3_LEFT, R3_UP) => 3,
            (R3_LEFT, R3_LEFT) => 1,
            (R3_LEFT, R3_DOWN) => 2,
            (R3_LEFT, R3_RIGHT) => 3,
            (R3_LEFT, R3_A) => 4,
            (R3_DOWN, R3_UP) => 2,
            (R3_DOWN, R3_LEFT) => 2,
            (R3_DOWN, R3_DOWN) => 1,
            (R3_DOWN, R3_RIGHT) => 2,
            (R3_DOWN, R3_A) => 3,
            (R3_RIGHT, R3_UP) => 3,
            (R3_RIGHT, R3_LEFT) => 3,
            (R3_RIGHT, R3_DOWN) => 2,
            (R3_RIGHT, R3_RIGHT) => 1,
            (R3_RIGHT, R3_A) => 2,
            (R3_A, R3_UP) => 2,
            (R3_A, R3_LEFT) => 4,
            (R3_A, R3_DOWN) => 3,
            (R3_A, R3_RIGHT) => 2,
            (R3_A, R3_A) => 1,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    fn new_pos1(num: u8) -> Pos<1> {
        match num {
            b'0'..=b'9' => Pos((num - b'0').into()),
            b'A' => R1_A,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    let (mut memo1, mut memo2) = (Memo::<1>::default(), Memo::<2>::default());

    input
        .lines()
        .map(|line| {
            parse::<Cost>(line).0
                * line
                    .bytes()
                    .map(new_pos1)
                    .scan(R1_A, |pos, target| {
                        let cost = robot1(*pos, target, &mut memo1, &mut memo2);
                        *pos = target;
                        Some(cost)
                    })
                    .sum::<Cost>()
        })
        .sum::<Cost>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    type Pos = usize;
    type Cost = u64;
    type Memos = [HashMap<(Pos, Pos), Cost>];

    const NUM_0: Pos = 0;
    const NUM_1: Pos = 1;
    const NUM_2: Pos = 2;
    const NUM_3: Pos = 3;
    const NUM_4: Pos = 4;
    const NUM_5: Pos = 5;
    const NUM_6: Pos = 6;
    const NUM_7: Pos = 7;
    const NUM_8: Pos = 8;
    const NUM_9: Pos = 9;
    const NUM_A: Pos = 10;

    const DIR_UP: Pos = 0;
    const DIR_LEFT: Pos = 1;
    const DIR_DOWN: Pos = 2;
    const DIR_RIGHT: Pos = 3;
    const DIR_A: Pos = 4;

    struct Node {
        pos_a: Pos,
        pos_b: Pos,
        pressed: bool,
        cost: Cost,
        seen: ArrayVec<[Pos; 11]>,
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.cost == other.cost
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Reverse(self.cost).partial_cmp(&Reverse(other.cost))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            Reverse(self.cost).cmp(&Reverse(other.cost))
        }
    }

    fn robot1(pos: Pos, target: Pos, memos: &mut Memos) -> Cost {
        const NUMPAD: [&[(Pos, Pos)]; 11] = [
            &[(NUM_2, DIR_UP), (NUM_A, DIR_RIGHT)],
            &[(NUM_2, DIR_RIGHT), (NUM_4, DIR_UP)],
            &[
                (NUM_0, DIR_DOWN),
                (NUM_1, DIR_LEFT),
                (NUM_3, DIR_RIGHT),
                (NUM_5, DIR_UP),
            ],
            &[(NUM_2, DIR_LEFT), (NUM_6, DIR_UP), (NUM_A, DIR_DOWN)],
            &[(NUM_1, DIR_DOWN), (NUM_5, DIR_RIGHT), (NUM_7, DIR_UP)],
            &[
                (NUM_2, DIR_DOWN),
                (NUM_4, DIR_LEFT),
                (NUM_6, DIR_RIGHT),
                (NUM_8, DIR_UP),
            ],
            &[(NUM_3, DIR_DOWN), (NUM_5, DIR_LEFT), (NUM_9, DIR_UP)],
            &[(NUM_4, DIR_DOWN), (NUM_8, DIR_RIGHT)],
            &[(NUM_5, DIR_DOWN), (NUM_7, DIR_LEFT), (NUM_9, DIR_RIGHT)],
            &[(NUM_6, DIR_DOWN), (NUM_8, DIR_LEFT)],
            &[(NUM_0, DIR_LEFT), (NUM_3, DIR_UP)],
        ];

        if pos == target {
            return 1;
        }
        let (memo, memos) = memos.split_first_mut().unwrap();
        if let Some(&cost) = memo.get(&(pos, target)) {
            return cost;
        }

        let mut nodes = BinaryHeap::default();
        nodes.push(Node {
            pos_a: pos,
            pos_b: DIR_A,
            pressed: false,
            cost: 0,
            seen: array_vec![],
        });
        loop {
            let Node {
                pos_a: r1_pos,
                pos_b: r2_pos,
                pressed,
                cost,
                seen,
            } = nodes.pop().unwrap();
            if r1_pos == target {
                if pressed {
                    memo.insert((pos, target), cost);
                    break cost;
                }
                let cost = cost + robots2_to_26(r2_pos, DIR_A, memos);
                nodes.push(Node {
                    pos_a: r1_pos,
                    pos_b: r2_pos,
                    pressed: true,
                    cost,
                    seen,
                });
            } else {
                for &(r1_neighbor, r2_target) in NUMPAD[r1_pos] {
                    if !seen.contains(&r1_neighbor) {
                        let cost = cost + robots2_to_26(r2_pos, r2_target, memos);
                        let mut seen = seen.clone();
                        seen.push(r1_pos);
                        nodes.push(Node {
                            pos_a: r1_neighbor,
                            pos_b: r2_target,
                            pressed,
                            cost,
                            seen,
                        });
                    }
                }
            }
        }
    }

    fn robots2_to_26(pos: Pos, target: Pos, memos: &mut Memos) -> Cost {
        const DIRPAD: [&[(Pos, Pos)]; 5] = {
            [
                &[(DIR_DOWN, DIR_DOWN), (DIR_A, DIR_RIGHT)],
                &[(DIR_DOWN, DIR_RIGHT)],
                &[
                    (DIR_UP, DIR_UP),
                    (DIR_LEFT, DIR_LEFT),
                    (DIR_RIGHT, DIR_RIGHT),
                ],
                &[(DIR_DOWN, DIR_LEFT), (DIR_A, DIR_UP)],
                &[(DIR_UP, DIR_LEFT), (DIR_RIGHT, DIR_DOWN)],
            ]
        };

        if pos == target {
            return 1;
        }
        let [memo, memos @ ..] = memos else {
            return robot26(pos, target);
        };
        if let Some(&cost) = memo.get(&(pos, target)) {
            return cost;
        }

        let mut nodes = BinaryHeap::default();
        nodes.push(Node {
            pos_a: pos,
            pos_b: DIR_A,
            pressed: false,
            cost: 0,
            seen: array_vec![],
        });
        loop {
            let Node {
                pos_a,
                pos_b,
                pressed,
                cost,
                seen,
            } = nodes.pop().unwrap();
            if pos_a == target {
                if pressed {
                    memo.insert((pos, target), cost);
                    break cost;
                }
                let cost = cost + robots2_to_26(pos_b, DIR_A, memos);
                nodes.push(Node {
                    pos_a,
                    pos_b,
                    pressed: true,
                    cost,
                    seen,
                });
            } else {
                for &(pos_a_neighbor, pos_b_target) in DIRPAD[pos_a] {
                    if !seen.contains(&pos_a_neighbor) {
                        let cost = cost + robots2_to_26(pos_b, pos_b_target, memos);
                        let mut seen = seen.clone();
                        seen.push(pos_a);
                        nodes.push(Node {
                            pos_a: pos_a_neighbor,
                            pos_b: pos_b_target,
                            pressed,
                            cost,
                            seen,
                        });
                    }
                }
            }
        }
    }

    fn robot26(pos: Pos, target: Pos) -> Cost {
        match (pos, target) {
            (DIR_UP, DIR_UP) => 1,
            (DIR_UP, DIR_LEFT) => 3,
            (DIR_UP, DIR_DOWN) => 2,
            (DIR_UP, DIR_RIGHT) => 3,
            (DIR_UP, DIR_A) => 2,
            (DIR_LEFT, DIR_UP) => 3,
            (DIR_LEFT, DIR_LEFT) => 1,
            (DIR_LEFT, DIR_DOWN) => 2,
            (DIR_LEFT, DIR_RIGHT) => 3,
            (DIR_LEFT, DIR_A) => 4,
            (DIR_DOWN, DIR_UP) => 2,
            (DIR_DOWN, DIR_LEFT) => 2,
            (DIR_DOWN, DIR_DOWN) => 1,
            (DIR_DOWN, DIR_RIGHT) => 2,
            (DIR_DOWN, DIR_A) => 3,
            (DIR_RIGHT, DIR_UP) => 3,
            (DIR_RIGHT, DIR_LEFT) => 3,
            (DIR_RIGHT, DIR_DOWN) => 2,
            (DIR_RIGHT, DIR_RIGHT) => 1,
            (DIR_RIGHT, DIR_A) => 2,
            (DIR_A, DIR_UP) => 2,
            (DIR_A, DIR_LEFT) => 4,
            (DIR_A, DIR_DOWN) => 3,
            (DIR_A, DIR_RIGHT) => 2,
            (DIR_A, DIR_A) => 1,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    fn new_pos(num: u8) -> Pos {
        match num {
            b'0'..=b'9' => (num - b'0').into(),
            b'A' => NUM_A,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }

    let mut memos: [_; 25] = array::from_fn(|_| HashMap::default());

    input
        .lines()
        .map(|line| {
            parse::<Cost>(line).0
                * line
                    .bytes()
                    .map(new_pos)
                    .scan(NUM_A, |pos, target| {
                        let cost = robot1(*pos, target, &mut memos);
                        *pos = target;
                        Some(cost)
                    })
                    .sum::<Cost>()
        })
        .sum::<Cost>()
        .into()
}
