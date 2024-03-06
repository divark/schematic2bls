use crate::common::*;
use schematic2bls::blockland::mapping::BrickBuilder;
use schematic2bls::blockland::save_file::to_save_file_output;
use schematic2bls::blockland::Brick;
use schematic2bls::extract_largest_cubes_from;

mod common;

const ZERO: Vec<usize> = Vec::new();
const ONE: [usize; 1] = [1];

const EVEN_MANY_1X_CUBES: [usize; 6] = [1, 1, 1, 1, 1, 1];
const EVEN_MANY_1X_CUBES_NUM: usize = 1;
const EVEN_MANY_1X_CUBES_NUM_NONSTACKED: usize = 6;

const EVEN_MANY_MIXED: [usize; 6] = [1, 2, 1, 1, 2, 1];
const EVEN_MANY_MIXED_NUM: usize = 3;
const EVEN_MANY_MIXED_NUM_NONSTACKED: usize = 4;

const ODD_MANY_1X_CUBES: [usize; 5] = [1, 1, 1, 1, 1];
const ODD_MANY_1X_CUBES_NUM: usize = 1;
const ODD_MANY_1X_CUBES_NUM_NONSTACKED: usize = 5;

const ODD_MANY_MIXED: [usize; 5] = [1, 2, 1, 1, 4];
const ODD_MANY_MIXED_NUM: usize = 2;
const ODD_MANY_MIXED_NUM_NONSTACKED: usize = 3;

/// Returns a reference to a Brick Builder populated
/// with cubes either in a stacked or flat configuration.
fn paint(cubes: &[usize], stacked: bool) -> BrickBuilder {
    let painting_direction = if stacked {
        Direction::Vertical
    } else {
        Direction::Horizontal
    };

    let mut cube_painter = CubePainter::new(&cubes);
    for cube in cubes {
        cube_painter.draw(painting_direction, *cube);
    }

    let grid = cube_painter.to_grid();
    let largest_cubes = extract_largest_cubes_from(grid);

    let mut brick_builder = BrickBuilder::new();

    for largest_cube in largest_cubes {
        let brick = Brick::new(largest_cube.indexes, largest_cube.side_length);
        brick_builder.with_brick(brick);
    }

    brick_builder
}

/// Returns whether the Bricks identified in the chunks
/// are increasing in height.
fn chunks_in_ascending_order(bricks: &Vec<Brick>, chunks: &Vec<Vec<usize>>) -> bool {
    for chunk in chunks {
        let mut last_z = 0.0;
        for brick_idx in chunk {
            let brick = &bricks[*brick_idx];

            if brick.position.2 <= last_z {
                return false;
            }

            last_z = brick.position.2;
        }
    }

    return true;
}

/// Asserts whether all Bricks assigned to a chunk in
/// all actual_chunks matches the expected naming convention
/// based on how they alternate.
fn assert_correct_name_assignment(actual_chunks: &Vec<Vec<usize>>, brick_builder: &BrickBuilder) {
    for chunk in actual_chunks {
        for (brick_num, brick_idx) in chunk.iter().enumerate() {
            let brick = &brick_builder.bricks[*brick_idx];

            let is_brick_idx_even = brick_num == 0 || brick_num % 2 == 0;
            if !is_brick_idx_even {
                assert!(
                    !brick.bottom_1x_cube,
                    "Brick is bottom_1x when it should not be."
                );
            } else {
                assert!(
                    brick.bottom_1x_cube,
                    "Brick not bottom_1x when it should be."
                );
            }
        }
    }
}

#[test]
fn test1() {
    let stacked = false;
    let cubes = ZERO;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(actual_chunks.len(), 0, "Chunk length mismatch.");

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test2() {
    let stacked = false;
    let cubes = ONE;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(actual_chunks.len(), 1, "Chunk length mismatch.");

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test3() {
    let stacked = true;
    let cubes = EVEN_MANY_1X_CUBES;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        EVEN_MANY_1X_CUBES_NUM,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn place_1x_cube_tower() {
    let stacked = true;
    let cubes = EVEN_MANY_1X_CUBES;
    let mut brick_builder = paint(&cubes, stacked);

    let expected = include_str!("../assets/brick_comparisons/1xCubesTower6High.bls").to_string();
    let actual = to_save_file_output(&brick_builder.build());

    assert_eq!(expected, actual);
}

#[test]
fn test4() {
    let stacked = true;
    let cubes = EVEN_MANY_MIXED;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        EVEN_MANY_MIXED_NUM,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test5() {
    let stacked = false;
    let cubes = EVEN_MANY_1X_CUBES;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        EVEN_MANY_1X_CUBES_NUM_NONSTACKED,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test6() {
    let stacked = false;
    let cubes = EVEN_MANY_MIXED;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        EVEN_MANY_MIXED_NUM_NONSTACKED,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test7() {
    let stacked = true;
    let cubes = ODD_MANY_1X_CUBES;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        ODD_MANY_1X_CUBES_NUM,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test8() {
    let stacked = true;
    let cubes = ODD_MANY_MIXED;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        ODD_MANY_MIXED_NUM,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test9() {
    let stacked = false;
    let cubes = ODD_MANY_1X_CUBES;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        ODD_MANY_1X_CUBES_NUM_NONSTACKED,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}

#[test]
fn test10() {
    let stacked = false;
    let cubes = ODD_MANY_MIXED;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(
        actual_chunks.len(),
        ODD_MANY_MIXED_NUM_NONSTACKED,
        "Chunk length mismatch."
    );

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    assert_correct_name_assignment(&actual_chunks, &brick_builder);
}
