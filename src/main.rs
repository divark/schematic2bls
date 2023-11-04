use grid2bls::{to_save_file_output, Brick};
use nbt::{decode::*, CompoundTag};
use std::env;
use std::fs::{self, File};
use std::io::Cursor;
use std::io::Write;
use std::path::Path;

fn main() {
    // let command_args: Vec<String> = env::args().collect();

    // if command_args.len() != 2 {
    //     eprintln!("Usage: {} schematic_file_path", command_args[0]);
    //     return;
    // }

    // let file_path = Path::new(&command_args[1]);
    // if !file_path.exists() {
    //     eprintln!(
    //         "{}: File {} was not found.",
    //         command_args[0], command_args[1]
    //     );
    //     return;
    // }

    // let mut file_cursor =
    //     Cursor::new(fs::read(file_path).expect("grid2bls: Could not read file into bytes."));

    // let schematic_tag = read_gzip_compound_tag(&mut file_cursor);
    // if schematic_tag.is_err() {
    //     eprintln!(
    //         "{}: Invalid schematic file given from file {}",
    //         command_args[0], command_args[1]
    //     );
    //     return;
    // }

    // let schematic_tag = schematic_tag.expect("grid2bls: A Compound Tag should exist by now.");
    // let three_dimensional_grid = schematic_to_3dgrid(schematic_tag);
    // let largest_cubes = grid_to_largest_cubes(three_dimensional_grid);

    // let mut bricks = Vec::new();
    let size = 8;

    let mut bricks = Vec::new();
    for i in 1..=size {
        bricks.push(Brick::from_right_coordinate(
            size,
            (size as usize, size as usize, size as usize * i as usize),
        ));
    }
    // while let Some(found_largest_cube) = get_largest_cube(&largest_cubes) {
    //     let parsed_brick = Brick::from_right_coordinate(
    //         found_largest_cube.side_length as u32,
    //         found_largest_cube.indexes,
    //     );
    //     bricks.push(parsed_brick);
    // }

    let save_file_contents = to_save_file_output(&bricks);

    println!("{}", save_file_contents.to_string());

    // let mut output_file_name = String::from(
    //     file_path
    //         .file_stem()
    //         .expect("grid2bls: File does not have a stem, huh.")
    //         .to_str()
    //         .expect("grid2bls: Could not convert OSString to str."),
    // );
    // output_file_name.push_str(".bls");

    // let output = File::create(&output_file_name);
    // if let Err(msg) = output {
    //     eprintln!(
    //         "{}: Could not write output file {}: {}",
    //         command_args[0], output_file_name, msg
    //     );
    //     return;
    // }

    // let mut output_file = output.expect("grid2bls: File should be created by now.");

    // if let Err(msg) = output_file.write_all(save_file_contents.as_bytes()) {
    //     eprintln!(
    //         "{}: Could not write to newly created output file {}: {}",
    //         command_args[0], output_file_name, msg
    //     );
    // }
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

fn get_largest_cube(largest_cube_grid: &[Vec<Vec<usize>>]) -> Option<LargestCube> {
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

    if largest_entry_found == 0 {
        return None;
    }

    Some(LargestCube {
        side_length: largest_entry_found,
        indexes: (i, j, k),
    })
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

    for length_entry in grid
        .iter_mut()
        .skip(start_i)
        .take(largest_cube.indexes.0 + 1)
    {
        for width_entry in length_entry
            .iter_mut()
            .skip(start_j)
            .take(largest_cube.indexes.1 + 1)
        {
            for height_entry in width_entry
                .iter_mut()
                .skip(start_k)
                .take(largest_cube.indexes.2 + 1)
            {
                *height_entry = 0;
            }
        }
    }
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

        let actual = get_largest_cube(&found_cubes).unwrap();

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

        clear_largest_cube_from(&largest_cube_found.unwrap(), &mut found_cubes);

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

        let actual = get_largest_cube(&found_cubes).unwrap();

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

        clear_largest_cube_from(&largest_cube_found.unwrap(), &mut found_cubes);

        let expected = vec![vec![vec![0; grid_size + 1]; grid_size + 1]; grid_size + 1];
        assert_eq!(expected, found_cubes);
    }
}
