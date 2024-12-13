use crate::utils::*;

const _SAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

pub fn part1(input: &str) -> Answer {
    fn flood_fill(
        garden: &GridOwned,
        plant: Byte,
        index: Index,
        perim: &mut u64,
        seen: &mut HashSet<Index>,
    ) {
        if seen.insert(index) {
            *perim += 4;
            for neighbor in dirs(index) {
                if garden.get(neighbor) == Some(plant) {
                    *perim -= 1;
                    flood_fill(garden, plant, neighbor, perim, seen);
                }
            }
        }
    }

    let mut garden = GridOwned::new(input);
    let mut seen = HashSet::default();

    garden
        .indices()
        .map(|index| {
            let plant = unsafe { garden.get(index).unwrap_unchecked() };
            if plant.is_null() {
                0
            } else {
                let mut perim = 0;
                flood_fill(&garden, plant, index, &mut perim, &mut seen);
                let area = seen.len() as u64;
                for index in seen.drain() {
                    unsafe { garden.get_mut(index).unwrap_unchecked() }.make_null();
                }
                area * perim
            }
        })
        .sum::<u64>()
        .into()
}

pub fn part2(input: &str) -> Answer {
    #[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
    struct Side {
        dir: Dir,
        anchor: Index,
    }

    fn flood_fill(
        garden: &GridOwned,
        plant: Byte,
        index: Index,
        seen_plants: &mut HashSet<Index>,
        seen_sides: &mut HashSet<Side>,
    ) {
        if seen_plants.insert(index) {
            for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                if garden.get(index + dir) == Some(plant) {
                    flood_fill(garden, plant, index + dir, seen_plants, seen_sides);
                } else {
                    let raycast_dir = dir.clockwise();
                    let mut anchor = index + raycast_dir;
                    let mut neighbor = index + dir + raycast_dir;

                    while garden.get(anchor) == Some(plant) && garden.get(neighbor) != Some(plant) {
                        anchor += raycast_dir;
                        neighbor += raycast_dir;
                    }
                    seen_sides.insert(Side { dir, anchor });
                }
            }
        }
    }

    let mut garden = GridOwned::new(input);
    let (mut seen_plants, mut seen_sides) = (HashSet::default(), HashSet::default());

    garden
        .indices()
        .map(|index| {
            let plant = unsafe { garden.get(index).unwrap_unchecked() };
            if plant.is_null() {
                0
            } else {
                flood_fill(&garden, plant, index, &mut seen_plants, &mut seen_sides);
                let (area, sides) = (seen_plants.len(), seen_sides.len());
                for &index in &seen_plants {
                    unsafe { garden.get_mut(index).unwrap_unchecked() }.make_null();
                }
                seen_plants.clear();
                seen_sides.clear();
                (area * sides) as _
            }
        })
        .sum::<u64>()
        .into()
}
