use super::mapping::{idx_1d_from, idx_3d_from, GridReader, GridSizes};
use super::LargestCube;
use std::collections::{HashMap, HashSet};

pub struct BinaryIndexHeap {
    pub max_idx_heap: Vec<usize>,
    pub visited: HashSet<usize>,
    pub node_data: HashMap<usize, u16>,
}

impl BinaryIndexHeap {
    pub fn from(grid: GridReader) -> BinaryIndexHeap {
        let mut node_data = HashMap::new();
        let mut max_idx_heap = Vec::new();

        for (i, grid_item) in grid.data().iter().enumerate() {
            if *grid_item == 0 {
                continue;
            }

            max_idx_heap.push(i);
            node_data.insert(i, *grid_item);
        }

        let mut index_max_heap = BinaryIndexHeap {
            max_idx_heap,
            visited: HashSet::new(),
            node_data,
        };

        index_max_heap.heapsort();
        index_max_heap
    }

    fn heapsort(&mut self) {
        for size in 1..=self.max_idx_heap.len() {
            self.heapify_up(size);
        }
    }

    fn heapify_up(&mut self, size: usize) {
        // To preserve the max heap, we have to bubble up the new
        // index until it reaches a point where everything above it
        // is bigger than it. This is considered heapify-up.
        let mut current_heap_idx = size - 1;
        while current_heap_idx != 0 {
            let current_idx = self.max_idx_heap[current_heap_idx];
            let current_data = self.get_data(current_idx);

            let parent_heap_idx = (current_heap_idx - 1) / 2;
            let parent_idx = self.max_idx_heap[parent_heap_idx];
            let parent_data = self.get_data(parent_idx);

            if parent_data > current_data {
                break;
            } else if parent_data == current_data && parent_idx > current_idx {
                break;
            }

            let old_parent_idx = parent_idx;
            self.max_idx_heap[parent_heap_idx] = current_idx;
            self.max_idx_heap[current_heap_idx] = old_parent_idx;

            current_heap_idx = parent_heap_idx;
        }
    }

    pub fn push(&mut self, index: usize) {
        self.max_idx_heap.push(index);

        if self.max_idx_heap.len() <= 1 {
            return;
        }

        self.heapify_up(self.max_idx_heap.len());
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.max_idx_heap.is_empty() {
            return None;
        }

        // The last element is swapped with the root of the max heap
        // in order to ensure that we are only going to swap values downward.
        let max_value = self.max_idx_heap[0];
        self.max_idx_heap[0] = self.max_idx_heap[self.max_idx_heap.len() - 1];
        self.max_idx_heap.pop();

        if self.max_idx_heap.is_empty() {
            return Some(max_value);
        }

        // We still want to preserve the max heap, even when removing the root.
        // This segment is the inverse of heapify-up, where we want to shift down the newest
        // item we chose as root to be in a spot where there is nothing smaller below it.
        let mut current_heap_idx = 0;
        loop {
            let current_idx = self.max_idx_heap[current_heap_idx];
            let current_data = self.get_data(current_idx);

            let left_heap_idx = (2 * current_heap_idx) + 1;
            let right_heap_idx = (2 * current_heap_idx) + 2;

            let mut max_data = current_data;
            let mut max_heap_idx = current_heap_idx;
            let mut max_idx = current_idx;
            if let Some(left_idx) = self.max_idx_heap.get(left_heap_idx) {
                let left_data = self.get_data(*left_idx);

                if left_data > max_data || (left_data == max_data && left_heap_idx > max_heap_idx) {
                    max_data = left_data;
                    max_heap_idx = left_heap_idx;
                    max_idx = *left_idx;
                }
            }

            if let Some(right_idx) = self.max_idx_heap.get(right_heap_idx) {
                let right_data = self.get_data(*right_idx);

                if right_data > max_data
                    || (right_data == max_data && right_heap_idx > max_heap_idx)
                {
                    max_heap_idx = right_heap_idx;
                    max_idx = *right_idx;
                }
            }

            if max_heap_idx == current_heap_idx {
                break;
            }

            self.max_idx_heap[max_heap_idx] = current_idx;
            self.max_idx_heap[current_heap_idx] = max_idx;

            current_heap_idx = max_heap_idx;
        }

        Some(max_value)
    }

    pub fn has_visited(&self, idx: usize) -> bool {
        self.visited.contains(&idx)
    }

    pub fn get_data(&self, idx: usize) -> u16 {
        *self.node_data.get(&idx).unwrap()
    }
}

fn nearest_power_of_two(side_length: u16) -> u16 {
    let power_of_two = f32::log2(side_length as f32).floor();

    f32::powi(2.0, power_of_two as i32) as u16
}

pub fn get_largest_cubes(largest_cube_grid: GridReader) -> Vec<LargestCube> {
    let mut largest_cubes = Vec::new();
    let sizes = largest_cube_grid.size_cloned();

    let mut max_heap = BinaryIndexHeap::from(largest_cube_grid);
    while let Some(idx_1d) = max_heap.pop() {
        if max_heap.has_visited(idx_1d) {
            continue;
        }

        let largest_cube_size = max_heap.get_data(idx_1d);
        if largest_cube_size == 0 {
            continue;
        }

        let idx_3d = idx_3d_from(idx_1d, &sizes);
        let largest_cube = LargestCube {
            side_length: nearest_power_of_two(largest_cube_size.clamp(1, 64)),
            indexes: idx_3d,
        };

        mark_visited_from(&largest_cube, &sizes, &mut max_heap);
        largest_cubes.push(largest_cube);
    }

    largest_cubes
}

pub fn mark_visited_from(
    largest_cube: &LargestCube,
    sizes: &GridSizes,
    max_heap: &mut BinaryIndexHeap,
) {
    let side_length = largest_cube.side_length as usize;

    let start_i = largest_cube.indexes.0 + 1 - side_length;
    let start_j = largest_cube.indexes.1 + 1 - side_length;
    let start_k = largest_cube.indexes.2 + 1 - side_length;

    for i in largest_cube.indexes.0..=start_i {
        for j in largest_cube.indexes.1..=start_j {
            for k in largest_cube.indexes.2..=start_k {
                let idx_1d = idx_1d_from(i, j, k, sizes);
                max_heap.visited.insert(idx_1d);
            }
        }
    }
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

        let found_cubes = grid_to_largest_cubes(grid);

        let expected = LargestCube {
            side_length: 2,
            indexes: (2, 2, 2),
        };

        let actual = &get_largest_cubes(found_cubes)[0];

        assert_eq!(expected, *actual);
    }

    // #[test]
    // fn clear_largest_cube_simple_2x2() {
    //     let cube_size = 2;
    //     let mut grid = vec![vec![vec![false; cube_size]; cube_size]; cube_size];

    //     for length_entry in grid.iter_mut() {
    //         for width_entry in length_entry {
    //             for height_entry in width_entry {
    //                 *height_entry = true;
    //             }
    //         }
    //     }

    //     let mut found_cubes = grid_to_largest_cubes(grid);
    //     let largest_cube_found = get_largest_cubes(found_cubes)[0];

    //     //clear_largest_cube_from(&largest_cube_found.unwrap(), &mut found_cubes);

    //     let expected = vec![vec![vec![0; cube_size + 1]; cube_size + 1]; cube_size + 1];
    //     assert_eq!(expected, found_cubes);
    // }

    #[test]
    fn largest_cube_spaced_3x3() {
        let cube_size = 3;
        let grid_size = 9;
        let mut grid = vec![vec![vec![false; grid_size]; grid_size]; grid_size];

        let start_idx = 2;
        for length_entry in grid.iter_mut().skip(start_idx).take(cube_size) {
            for width_entry in length_entry.iter_mut().skip(start_idx).take(cube_size) {
                for height_entry in width_entry.iter_mut().skip(start_idx).take(cube_size) {
                    *height_entry = true;
                }
            }
        }

        let found_cubes = grid_to_largest_cubes(grid);

        let expected = LargestCube {
            side_length: 3,
            indexes: (5, 5, 5),
        };

        let actual = &get_largest_cubes(found_cubes)[0];

        assert_eq!(expected, *actual);
    }

    // #[test]
    // fn clear_largest_cube_spaced_3x3() {
    //     let cube_size = 3;
    //     let grid_size = 9;
    //     let mut grid = vec![vec![vec![false; grid_size]; grid_size]; grid_size];

    //     let start_idx = 2;
    //     for length_entry in grid.iter_mut().skip(start_idx).take(cube_size) {
    //         for width_entry in length_entry.iter_mut().skip(start_idx).take(cube_size) {
    //             for height_entry in width_entry.iter_mut().skip(start_idx).take(cube_size) {
    //                 *height_entry = true;
    //             }
    //         }
    //     }

    //     let mut found_cubes = grid_to_largest_cubes(grid);
    //     let largest_cube_found = &get_largest_cubes(found_cubes)[0];

    //     clear_largest_cube_from(&largest_cube_found.unwrap(), &mut found_cubes);

    //     let expected = vec![vec![vec![0; grid_size + 1]; grid_size + 1]; grid_size + 1];
    //     assert_eq!(expected, found_cubes);
    // }
}
