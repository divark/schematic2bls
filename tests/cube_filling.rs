use crate::common::*;
use schematic2bls::blockland::mapping::BrickBuilder;
use schematic2bls::blockland::Brick;
use schematic2bls::extract_largest_cubes_from;

mod common;

const ZERO: Vec<usize> = Vec::new();
const ONE: [usize; 1] = [1];
const EVEN_MANY_1xCUBES: [usize; 6] = [1, 1, 1, 1, 1, 1];
const EVEN_MANY_MIXED: [usize; 6] = [1, 2, 1, 1, 2, 1];
const ODD_MANY_1xCUBES: [usize; 5] = [1, 1, 1, 1, 1];
const ODD_MANY_MIXED: [usize; 5] = [1, 2, 1, 1, 4];

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

/*
 * let stacked = true or false
 * let bricks = ZERO or ONE or EVENMANY or ODDMANY
 * let brick_builder = paint(bricks, stacked)
 * let actual_chunks = brick_builder.map_chunks()
 * actual_chunks must be
 * - # 1x Cubes if not stacked
 * - 1 if stacked with same brick type
 * - manually checked otherwise
 * actual_chunks must be
 * - in ascending order
 * for each chunk in actual_chunks
 *  for each (brick_#, brick) in chunk
 *      if brick_# is odd
 *      - brick.bottom_1x_cube must be false
 *      else
 *      - brick.bottom_1x_cube must be true
 */

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
    for chunk in actual_chunks {
        for brick_idx in chunk {
            let brick = &brick_builder.bricks[brick_idx];

            let is_brick_idx_odd = brick_idx % 2 != 0;
            if is_brick_idx_odd {
                assert_eq!(
                    brick.bottom_1x_cube, false,
                    "Brick is bottom_1x when it should not be."
                );
            } else {
                assert_eq!(
                    brick.bottom_1x_cube, true,
                    "Brick not bottom_1x when it should be."
                );
            }
        }
    }
}

#[test]
fn test3() {
    let stacked = true;
    let cubes = EVEN_MANY_1xCUBES;
    let mut brick_builder = paint(&cubes, stacked);

    let actual_chunks = brick_builder.get_1x_cube_bundles();
    assert!(chunks_in_ascending_order(
        &brick_builder.bricks,
        &actual_chunks
    ));
    assert_eq!(actual_chunks.len(), 1, "Chunk length mismatch.");

    brick_builder.map_1x_cube_bundles(actual_chunks.clone());
    for chunk in actual_chunks {
        for brick_idx in chunk {
            let brick = &brick_builder.bricks[brick_idx];

            let is_brick_idx_odd = brick_idx % 2 != 0;
            if is_brick_idx_odd {
                assert_eq!(
                    brick.bottom_1x_cube, false,
                    "Brick is bottom_1x when it should not be."
                );
            } else {
                assert_eq!(
                    brick.bottom_1x_cube, true,
                    "Brick not bottom_1x when it should be."
                );
            }
        }
    }
}
