use crate::utils::*;
use std::iter;

const NULL: u16 = u16::MAX;
const _SAMPLE: &str = "2333133121414131402";

fn checksum(fs: &[u16]) -> u64 {
    fs.iter()
        .copied()
        .enumerate()
        .map(|(i, id)| i as u64 * id as u64)
        .sum()
}

pub fn part1(input: &str) -> Answer {
    let bytes = input.as_bytes();
    let mut fs = Vec::with_capacity(bytes.iter().map(|&b| b - b'0').map_into::<usize>().sum());

    for (chunk, id) in bytes.chunks(2).zip(0..) {
        let file = (chunk[0] - b'0').into();
        let empty = chunk.get(1).map(|&b| b - b'0').unwrap_or(0).into();

        fs.extend(iter::repeat_n(id, file));
        fs.extend(iter::repeat_n(NULL, empty));
    }

    let (mut front, mut back) = (0, fs.len() - 1);
    while front < back {
        if fs[front] == NULL {
            fs[front] = fs[back];
            back -= 1;
            while fs[back] == NULL {
                back -= 1;
            }
        }
        front += 1;
    }
    fs.resize(back + 1, NULL);

    checksum(&fs).into()
}

pub fn part2(input: &str) -> Answer {
    struct File {
        size: u8,
        id: u16,
    }

    let bytes = input.as_bytes();
    let half = bytes.len() / 2 + 1;
    let (mut files, mut spaces) = (Vec::with_capacity(half), Vec::with_capacity(half));

    for (chunk, id) in bytes.chunks(2).zip(0..) {
        files.push(File {
            size: chunk[0] - b'0',
            id,
        });
        if let Some(&space_size) = chunk.get(1) {
            spaces.push(ArrayVec::from_array_len(
                [NULL; 9],
                (space_size - b'0').into(),
            ));
        }
    }

    for File { size, id } in files.iter_mut().rev() {
        let next_empty = spaces[..(*id).into()].iter_mut().find_map(|space| {
            let first_null = space.iter().position(|&b| b == NULL)?;
            let rem = &mut space[first_null..];
            (rem.len() >= (*size).into()).then_some(rem)
        });
        if let Some(empty) = next_empty {
            empty[..(*size).into()].fill(*id);
            *id = NULL;
        }
    }

    let (mut pos, mut checksum) = (0, 0);
    for (File { size, id }, filled_space) in files.into_iter().zip(spaces) {
        for _ in 0..size {
            if id != NULL {
                checksum += pos * u64::from(id);
            }
            pos += 1;
        }
        for space in filled_space {
            if space != NULL {
                checksum += pos * u64::from(space);
            }
            pos += 1;
        }
    }

    checksum.into()
}
