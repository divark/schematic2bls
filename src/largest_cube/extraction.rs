use super::mapping::{idx_1d_from, idx_3d_from, GridReader, GridSizes};
use super::LargestCube;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct LargestCubeEntry {
    pub data: u16,
    pub idx: usize,
}

pub struct BinaryIndexHeap {
    pub heap: BinaryHeap<LargestCubeEntry>,
    pub visited: HashSet<usize>,
}

impl BinaryIndexHeap {
    pub fn from(grid: GridReader) -> BinaryIndexHeap {
        let mut heap = BinaryHeap::with_capacity(grid.data().len());

        for (i, grid_item) in grid.data().iter().enumerate() {
            if *grid_item == 0 {
                continue;
            }

            heap.push(LargestCubeEntry {
                data: *grid_item,
                idx: i,
            });
        }

        let index_max_heap = BinaryIndexHeap {
            heap,
            visited: HashSet::new(),
        };

        index_max_heap
    }

    pub fn pop(&mut self) -> Option<LargestCubeEntry> {
        self.heap.pop()
    }

    pub fn has_visited(&self, idx: usize) -> bool {
        self.visited.contains(&idx)
    }
}

fn nearest_power_of_two(side_length: u16) -> u16 {
    let power_of_two = f32::log2(side_length as f32).floor();

    f32::powi(2.0, power_of_two as i32) as u16
}

pub fn get_largest_cubes(largest_cube_grid: GridReader, scale: u16) -> Vec<LargestCube> {
    let mut largest_cubes = Vec::new();
    let sizes = largest_cube_grid.size_cloned();

    let mut max_heap = BinaryIndexHeap::from(largest_cube_grid);
    while let Some(largest_cube_entry) = max_heap.pop() {
        let idx_1d = largest_cube_entry.idx;
        if max_heap.has_visited(idx_1d) {
            continue;
        }

        let largest_cube_size = largest_cube_entry.data;
        if largest_cube_size == 0 {
            break;
        }

        let mut idx_3d = idx_3d_from(idx_1d, &sizes);
        idx_3d.0 *= scale as usize;
        idx_3d.1 *= scale as usize;
        idx_3d.2 *= scale as usize;

        let clamped_side_length = nearest_power_of_two(largest_cube_size.clamp(1, 64));
        let mut largest_cube = LargestCube {
            side_length: clamped_side_length,
            indexes: idx_3d,
        };

        let found_overlap = mark_visited_from(&largest_cube, &sizes, &mut max_heap, scale as usize);
        if found_overlap {
            // An overlap means that the current size of the Largest
            // Cube is too big. Knocking it down by our scale guarantees
            // there will not be another overlap.
            largest_cube.side_length = nearest_power_of_two(clamped_side_length - scale);
        }

        largest_cubes.push(largest_cube);
    }

    largest_cubes
}

/// Returns whether marking all points contained by the claimed
/// Largest Cube found no overlaps in the process, or false otherwise.
pub fn mark_visited_from(
    largest_cube: &LargestCube,
    sizes: &GridSizes,
    max_heap: &mut BinaryIndexHeap,
    scale: usize,
) -> bool {
    let side_length = largest_cube.side_length as usize / scale;

    let x = largest_cube.indexes.0 / scale;
    let y = largest_cube.indexes.1 / scale;
    let z = largest_cube.indexes.2 / scale;

    let end_x = x - side_length + 1;
    let end_y = y - side_length + 1;
    let end_z = z - side_length + 1;

    // There exists an edge case in the Maximal Cube algorithm where
    // if you have two cubes of size 16, and 8 in that order from
    // left to right, for example, and both slightly overlap each other,
    // the 16 will be correctly chosen, but the 8 will also be selected,
    // which is wrong, since that will cause an overlap.
    //
    // Because of this situation, we need a way of checking
    // if there's overlap, and if so, to undo what we just did
    // and signal to the caller that there was a problem.
    let mut found_overlap = false;
    let mut found_indexes = Vec::new();
    for i in (end_x..=x).rev() {
        for j in (end_y..=y).rev() {
            for k in (end_z..=z).rev() {
                let idx_1d = idx_1d_from(i, j, k, sizes);
                if max_heap.visited.contains(&idx_1d) {
                    found_overlap = true;
                    break;
                }

                found_indexes.push(idx_1d);
                max_heap.visited.insert(idx_1d);
            }
        }
    }

    if !found_overlap {
        return found_overlap;
    }

    while let Some(visited_idx) = found_indexes.pop() {
        max_heap.visited.remove(&visited_idx);
    }

    found_overlap
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::largest_cube::mapping::grid_to_largest_cubes;

    #[test]
    fn largest_cube_simple_2x2() {
        let cube_size = 2;
        let mut grid = vec![vec![vec![false; cube_size]; cube_size]; cube_size];

        for length_entry in grid.iter_mut() {
            for width_entry in length_entry {
                for height_entry in width_entry {
                    *height_entry = true;
                }
            }
        }

        let found_cubes = grid_to_largest_cubes(grid, 1);

        let expected = LargestCube {
            side_length: 2,
            indexes: (2, 2, 2),
        };

        let actual = &get_largest_cubes(found_cubes, 1)[0];

        assert_eq!(expected, *actual);
    }
}
