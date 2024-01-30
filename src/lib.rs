mod blockland;
mod largest_cube;
mod model;

use std::{
    fs::{self, File},
    io::{Cursor, Write},
};

use blockland::{mapping::BrickBuilder, save_file::to_save_file_output, Brick};
use largest_cube::{extraction::get_largest_cubes, mapping::grid_to_largest_cubes, LargestCube};
use model::{conversion::schematic_to_3dgrid, scaling::scale_grid};
use nbt::{decode::read_gzip_compound_tag, CompoundTag};

pub fn load_schematic(model_arg: &String) -> CompoundTag {
    let mut file_cursor = Cursor::new(
        fs::read(model_arg)
            .expect("grid2bls: Could not read file into bytes.")
            .to_vec(),
    );
    read_gzip_compound_tag(&mut file_cursor).expect("Could not read given schematic file.")
}

pub fn parse_grid_from_model(model: CompoundTag) -> Vec<Vec<Vec<bool>>> {
    schematic_to_3dgrid(model)
}

pub fn scale_up_grid(scaling_factor: u8, voxel_grid: Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    scale_grid(&voxel_grid, scaling_factor as usize)
}

pub fn extract_largest_cubes_from(voxel_grid: Vec<Vec<Vec<bool>>>) -> Vec<LargestCube> {
    let largest_cubes_grid = grid_to_largest_cubes(voxel_grid);

    get_largest_cubes(largest_cubes_grid)
}

pub fn extract_bricks_from(largest_cubes: Vec<LargestCube>) -> Vec<Brick> {
    let mut brick_builder = BrickBuilder::new();

    for largest_cube in largest_cubes {
        let brick = Brick::new(largest_cube.indexes, largest_cube.side_length);
        brick_builder.with_brick(brick);
    }

    brick_builder.build()
}

pub fn write_save_file(bricks: &Vec<Brick>) {
    let mut save_file = File::create("output.bls").expect("grid2bls: Could not create save file.");
    let save_file_content = to_save_file_output(&bricks);
    save_file
        .write_all(save_file_content.as_bytes())
        .expect("grid2bls: Could not write save data to new save file");
}
