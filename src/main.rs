use nbt::{decode::*, CompoundTag};
use std::io::Cursor;

fn main() {
    println!("Hello! This is a placeholder!");
}

pub fn schematic_to_3dgrid(schematic_root: CompoundTag) -> Vec<Vec<Vec<bool>>> {
    let length = schematic_root
        .get_i16("Length")
        .expect("Could not find Length field.") as usize;
    let width = schematic_root
        .get_i16("Width")
        .expect("Could not find Width field.") as usize;
    let height = schematic_root
        .get_i16("Height")
        .expect("Could not get Height field") as usize;

    let mut grid = vec![vec![vec![false; height]; width]; length];

    let blocks = schematic_root
        .get_i8_vec("Blocks")
        .expect("Could not get Blocks field in schematic.");
    for (i, length_entry) in grid.iter_mut().enumerate() {
        for (j, width_entry) in length_entry.iter_mut().enumerate() {
            for (k, height_entry) in width_entry.iter_mut().enumerate() {
                let blocks_index = (j * length + k) * width + i;
                *height_entry = blocks[blocks_index] == 1;
            }
        }
    }

    grid
}

#[derive(PartialEq, Debug)]
pub struct LargestCube {
    pub side_length: usize,
    pub indexes: (usize, usize, usize),
}

fn get_largest_cube(largest_cube_grid: &Vec<Vec<Vec<usize>>>) -> LargestCube {
    let mut largest_entry_found = 0;
    let (mut i, mut j, mut k) = (0, 0, 0);

    for (length_idx, length_entry) in largest_cube_grid.iter().enumerate() {
        for (width_idx, width_entry) in length_entry.iter().enumerate() {
            for (entry_idx, entry) in width_entry.iter().enumerate() {
                if *entry > largest_entry_found {
                    largest_entry_found = *entry;

                    i = length_idx;
                    j = width_idx;
                    k = entry_idx;
                }
            }
        }
    }

    LargestCube {
        side_length: largest_entry_found,
        indexes: (i, j, k),
    }
}

pub fn grid_to_largest_cubes(grid: Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<usize>>> {
    let length = grid.len();
    let width = grid[0].len();
    let height = grid[0][0].len();

    let mut largest_cube = vec![vec![vec![0; height + 1]; width + 1]; length + 1];

    for i in 1..=length {
        for j in 1..=width {
            for k in 1..=height {
                if grid[i - 1][j - 1][k - 1] {
                    let smallest_prior_cube = largest_cube[i][j][k - 1]
                        .min(largest_cube[i][j - 1][k - 1])
                        .min(largest_cube[i - 1][j][k - 1])
                        .min(largest_cube[i - 1][j - 1][k - 1])
                        .min(largest_cube[i][j - 1][k])
                        .min(largest_cube[i - 1][j - 1][k])
                        .min(largest_cube[i - 1][j][k]);

                    largest_cube[i][j][k] = smallest_prior_cube + 1;
                }
            }
        }
    }

    largest_cube
}

pub fn clear_largest_cube_from(largest_cube: &LargestCube, grid: &mut [Vec<Vec<usize>>]) {
    let start_i = (largest_cube.indexes.0 - largest_cube.side_length) + 1;
    let start_j = (largest_cube.indexes.1 - largest_cube.side_length) + 1;
    let start_k = (largest_cube.indexes.2 - largest_cube.side_length) + 1;

    for length_entry in grid.iter_mut().skip(start_i).take(largest_cube.indexes.0 + 1) {
        for width_entry in length_entry.iter_mut().skip(start_j).take(largest_cube.indexes.1 + 1) {
            for height_entry in width_entry.iter_mut().skip(start_k).take(largest_cube.indexes.2 + 1) {
                *height_entry = 0;
            }
        }
    }
}

pub struct BlocklandBrick {
    coordinates: (f32, f32, f32),
    cube_length: f32,
}

impl BlocklandBrick {
    pub fn new(largest_cube: &LargestCube) -> BlocklandBrick {
        let cube_length = largest_cube.side_length as f32;

        let x = largest_cube.indexes.0 as f32 - (cube_length / 2.0);
        let y = largest_cube.indexes.1 as f32 - (cube_length / 2.0);
        let mut z = f32::log2(largest_cube.indexes.2 as f32 - (cube_length / 2.0)).floor();
        if cube_length == 1.0 {
            z = 0.3;
            for _i in 1..largest_cube.indexes.2 {
                z += 0.6;
            }
        }

        BlocklandBrick {
            coordinates: (x, y, z),
            cube_length,
        }
    }

    // TODO: Map properties to string accordingly from following resource:
    // https://docs.rs/bl_save/latest/bl_save/struct.BrickBase.html
    // pub fn to_string(&self) -> String {
    //     let block_line = format!("{}x{}\" {} {} {} 0 1 0  0 0 1 1 1");

    //     block_line
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_test_schematic() -> CompoundTag {
        let mut file_cursor =
            Cursor::new(include_bytes!("../assets/peachs_castle.schematic").to_vec());
        read_gzip_compound_tag(&mut file_cursor).expect("Could not read given schematic file.")
    }

    fn get_num_blocks_filled(grid: &Vec<Vec<Vec<bool>>>) -> usize {
        let mut num_blocks = 0;

        for length_entry in grid {
            for width_entry in length_entry {
                for entry in width_entry {
                    if *entry {
                        num_blocks += 1;
                    }
                }
            }
        }

        num_blocks
    }

    #[test]
    fn correct_num_blocks_loaded() {
        let schematic = load_test_schematic();
        let grid = schematic_to_3dgrid(schematic);

        let expected = 182446;
        let actual = get_num_blocks_filled(&grid);

        assert_eq!(expected, actual);
    }

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

        let actual = get_largest_cube(&found_cubes);

        assert_eq!(expected, actual);
    }

    #[test]
    fn clear_largest_cube_simple_2x2() {
        let cube_size = 2;
        let mut grid = vec![vec![vec![false; cube_size]; cube_size]; cube_size];

        for length_entry in grid.iter_mut() {
            for width_entry in length_entry {
                for height_entry in width_entry {
                    *height_entry = true;
                }
            }
        }

        let mut found_cubes = grid_to_largest_cubes(grid);
        let largest_cube_found = get_largest_cube(&found_cubes);

        clear_largest_cube_from(&largest_cube_found, &mut found_cubes);

        let expected = vec![vec![vec![0; cube_size + 1]; cube_size + 1]; cube_size + 1];
        assert_eq!(expected, found_cubes);
    }

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

        let actual = get_largest_cube(&found_cubes);

        assert_eq!(expected, actual);
    }

    #[test]
    fn clear_largest_cube_spaced_3x3() {
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

        let mut found_cubes = grid_to_largest_cubes(grid);
        let largest_cube_found = get_largest_cube(&found_cubes);

        clear_largest_cube_from(&largest_cube_found, &mut found_cubes);

        let expected = vec![vec![vec![0; grid_size + 1]; grid_size + 1]; grid_size + 1];
        assert_eq!(expected, found_cubes);
    }
}
