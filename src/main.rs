mod blockland;
mod largest_cube;
mod model;

use nbt::{decode::*, CompoundTag};
use std::env;
use std::fs::{self, File};
use std::io::Cursor;
use std::io::Write;
use std::path::Path;

use crate::blockland::save_file::to_save_file_output;
use crate::blockland::Brick;

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
    // let size = 8;

    // let mut bricks = Vec::new();
    // for i in 1..=size {
    //     bricks.push(Brick::calculate_right_offset(
    //         size,
    //         size,
    //         (size as usize, size as usize, size as usize * i as usize),
    //     ));
    // }
    // while let Some(found_largest_cube) = get_largest_cube(&largest_cubes) {
    //     let parsed_brick = Brick::from_right_coordinate(
    //         found_largest_cube.side_length as u32,
    //         found_largest_cube.indexes,
    //     );
    //     bricks.push(parsed_brick);
    // }

    // let save_file_contents = to_save_file_output(&bricks);

    // println!("{}", save_file_contents);

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
