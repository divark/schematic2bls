mod common;

use crate::blockland::save_file::to_save_file_output;
use crate::common::*;
use schematic2bls::*;

/// Returns a Blockland Save File formatted String based on what
/// Bricks (cube_sizes) were placed depending on its direction.
/// NOTE: All cube_sizes will be scaled up (multiplied) by 4.
fn paint_to_bricks(cube_sizes: &[usize], direction: Direction) -> String {
    let scaling_factor = 4;

    let mut cube_painter = CubePainter::new(&cube_sizes);
    for cube_size in cube_sizes.iter() {
        cube_painter.draw(direction, *cube_size);
    }

    let grid = cube_painter.to_grid();
    let largest_cubes = extract_largest_cubes_from(grid, scaling_factor);
    let bricks = extract_bricks_from(largest_cubes);

    to_save_file_output(&bricks)
}

#[test]
fn place_one_4x_cube() {
    let cube_sizes = [1];
    let expected = include_str!("../assets/brick_comparisons/4xCube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_4x_cube_tower() {
    let cube_sizes = [1; 4];
    let expected = include_str!("../assets/brick_comparisons/4xCubesTower.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Vertical);

    assert_eq!(expected, actual);
}

#[test]
fn place_1x4_4x_cubes() {
    let cube_sizes = [1; 4];
    let expected = include_str!("../assets/brick_comparisons/4xCubesLine.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_4_8_cube() {
    let cube_sizes = [1, 2];
    let expected = include_str!("../assets/brick_comparisons/4-8Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_8_4_cube() {
    let cube_sizes = [2, 1];
    let expected = include_str!("../assets/brick_comparisons/8-4Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_4_16_cube() {
    let cube_sizes = [1, 4];
    let expected = include_str!("../assets/brick_comparisons/4-16Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_4_32_cube() {
    let cube_sizes = [1, 8];
    let expected = include_str!("../assets/brick_comparisons/4-32Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_4_64_cube() {
    let cube_sizes = [1, 16];
    let expected = include_str!("../assets/brick_comparisons/4-64Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_one_8x_cube() {
    let cube_sizes = [2];
    let expected = include_str!("../assets/brick_comparisons/8xCube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_8_16_cube() {
    let cube_sizes = [2, 4];
    let expected = include_str!("../assets/brick_comparisons/8-16Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_8_32_cube() {
    let cube_sizes = [2, 8];
    let expected = include_str!("../assets/brick_comparisons/8-32Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_8_64_cube() {
    let cube_sizes = [2, 16];
    let expected = include_str!("../assets/brick_comparisons/8-64Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_16_8_cube() {
    let cube_sizes = [4, 2];
    let expected = include_str!("../assets/brick_comparisons/16-8Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_16_4_8_cube() {
    let cube_sizes = [4, 1, 2];
    let expected = include_str!("../assets/brick_comparisons/16-4-8Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_16_32_cube() {
    let cube_sizes = [4, 8];
    let expected = include_str!("../assets/brick_comparisons/16-32Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_16_64_cube() {
    let cube_sizes = [4, 16];
    let expected = include_str!("../assets/brick_comparisons/16-64Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_32_64_cube() {
    let cube_sizes = [8, 16];
    let expected = include_str!("../assets/brick_comparisons/32-64Cube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_1x4_8x_cubes() {
    let cube_sizes = [2; 4];
    let expected = include_str!("../assets/brick_comparisons/8xCubesLine.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_8x_cube_tower() {
    let cube_sizes = [2; 4];
    let expected = include_str!("../assets/brick_comparisons/8xCubesTower.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Vertical);

    assert_eq!(expected, actual);
}

#[test]
fn place_one_16x_cube() {
    let cube_sizes = [4];
    let expected = include_str!("../assets/brick_comparisons/16xCube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_1x4_16x_cubes() {
    let cube_sizes = [4; 4];
    let expected = include_str!("../assets/brick_comparisons/16xCubesLine.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_16x_cube_tower() {
    let cube_sizes = [4; 4];
    let expected = include_str!("../assets/brick_comparisons/16xCubesTower.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Vertical);

    assert_eq!(expected, actual);
}

#[test]
fn place_one_32x_cube() {
    let cube_sizes = [8];
    let expected = include_str!("../assets/brick_comparisons/32xCube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_1x4_32x_cubes() {
    let cube_sizes = [8; 4];
    let expected = include_str!("../assets/brick_comparisons/32xCubesLine.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_32x_cube_tower() {
    let cube_sizes = [8; 4];
    let expected = include_str!("../assets/brick_comparisons/32xCubesTower.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Vertical);

    assert_eq!(expected, actual);
}

#[test]
fn place_one_64x_cube() {
    let cube_sizes = [16];
    let expected = include_str!("../assets/brick_comparisons/64xCube.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_1x4_64x_cubes() {
    let cube_sizes = [16; 4];
    let expected = include_str!("../assets/brick_comparisons/64xCubesLine.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}

#[test]
fn place_64x_cube_tower() {
    let cube_sizes = [16; 4];
    let expected = include_str!("../assets/brick_comparisons/64xCubesTower.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Vertical);

    assert_eq!(expected, actual);
}

#[test]
fn place_increasing_cubes_scale() {
    let cube_sizes = [1, 2, 4, 8, 16];
    let expected = include_str!("../assets/brick_comparisons/CubeScale.bls").to_string();

    let actual = paint_to_bricks(&cube_sizes, Direction::Horizontal);

    assert_eq!(expected, actual);
}
