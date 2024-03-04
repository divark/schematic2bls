use schematic2bls::*;

#[derive(Copy, Clone)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

/// A mock-based tool used to create 3D Grids
/// compatible with the Maximal Cube algorithm.
struct CubePainter {
    grid: Vec<Vec<Vec<bool>>>,
    next_idx: (usize, usize, usize),
}

impl CubePainter {
    pub fn new(cube_sizes: &[usize]) -> Self {
        let grid_size = cube_sizes.iter().sum();

        CubePainter {
            grid: vec![vec![vec![false; grid_size]; grid_size]; grid_size],
            next_idx: (0, 0, 0),
        }
    }

    /// Returns the index of where the next cube will be
    /// drawn after this cube specified by cube_size has
    /// been placed in some orientation specified by Direction.
    fn draw_at(
        &mut self,
        direction: Direction,
        cube_size: usize,
        start_idx: (usize, usize, usize),
    ) -> (usize, usize, usize) {
        let (mut x, mut y, mut z) = start_idx;

        for i in x..x + cube_size {
            for j in y..y + cube_size {
                for k in z..z + cube_size {
                    self.grid[i][j][k] = true;
                }
            }
        }

        match direction {
            Direction::Horizontal => {
                x += cube_size;
            }
            Direction::Vertical => {
                z += cube_size;
            }
            Direction::Diagonal => {
                x += cube_size;
                y += cube_size;
            }
        }

        (x, y, z)
    }

    /// Places a Cube of size cube_size in the next available grid
    /// slot in some direction.
    pub fn draw(&mut self, direction: Direction, cube_size: usize) {
        let next_idx = self.draw_at(direction, cube_size, self.next_idx);
        self.next_idx = next_idx;
    }

    pub fn to_grid(&self) -> Vec<Vec<Vec<bool>>> {
        self.grid.clone()
    }
}

fn generate_grid_from(cube_sizes: &[usize], direction: Direction) -> Vec<Vec<Vec<bool>>> {
    let mut cube_painter = CubePainter::new(&cube_sizes);
    for cube_size in cube_sizes.iter() {
        cube_painter.draw(direction, *cube_size);
    }

    cube_painter.to_grid()
}

fn get_expected_side_lengths_from(cube_sizes: &[usize]) -> Vec<usize> {
    let mut expected_side_lengths = cube_sizes.iter().cloned().collect::<Vec<usize>>();
    expected_side_lengths.sort_unstable();
    expected_side_lengths.reverse();

    expected_side_lengths
}

const BLOCKS_ONE: [usize; 1] = [1];
const BLOCKS_TWO: [usize; 2] = [1, 2];
const BLOCKS_THREE: [usize; 3] = [1, 2, 4];
const BLOCKS_FOUR: [usize; 4] = [1, 2, 4, 8];
const BLOCKS_FIVE: [usize; 5] = [1, 2, 4, 8, 16];
const BLOCKS_SIX: [usize; 6] = [1, 2, 4, 8, 16, 32];

//Test Case 1   		<single>
#[test]
fn case1() {
    let cube_sizes = Vec::new();
    let direction = Direction::Horizontal;

    let grid = generate_grid_from(&cube_sizes, direction);
    let largest_cubes = extract_largest_cubes_from(grid);

    let expected_side_lengths = get_expected_side_lengths_from(&cube_sizes);
    assert_eq!(largest_cubes.len(), expected_side_lengths.len());

    for (idx, largest_cube) in largest_cubes.iter().enumerate() {
        assert_eq!(
            largest_cube.side_length as usize,
            expected_side_lengths[idx]
        );
    }
}

//Test Case 2   		(Key = 1.2.0.0.)
#[test]
fn case2() {
    let cube_sizes = BLOCKS_ONE;
    let direction = Direction::Horizontal;

    let grid = generate_grid_from(&cube_sizes, direction);
    let largest_cubes = extract_largest_cubes_from(grid);

    let expected_side_lengths = get_expected_side_lengths_from(&cube_sizes);
    assert_eq!(largest_cubes.len(), expected_side_lengths.len());

    for (idx, largest_cube) in largest_cubes.iter().enumerate() {
        assert_eq!(
            largest_cube.side_length as usize,
            expected_side_lengths[idx]
        );
    }
}

//Test Case 3   		(Key = 1.3.1.1.)
#[test]
fn case3() {}

//Test Case 4   		(Key = 1.3.1.2.)
#[test]
fn case4() {
    let cube_sizes = BLOCKS_TWO;
    let direction = Direction::Horizontal;

    let grid = generate_grid_from(&cube_sizes, direction);
    let largest_cubes = extract_largest_cubes_from(grid);

    let expected_side_lengths = get_expected_side_lengths_from(&cube_sizes);
    assert_eq!(largest_cubes.len(), expected_side_lengths.len());

    for (idx, largest_cube) in largest_cubes.iter().enumerate() {
        assert_eq!(
            largest_cube.side_length as usize,
            expected_side_lengths[idx]
        );
    }
}

//Test Case 5   		(Key = 1.3.2.1.)
#[test]
fn case5() {}

//Test Case 6   		(Key = 1.3.2.2.)
#[test]
fn case6() {
    let cube_sizes = BLOCKS_TWO;
    let direction = Direction::Vertical;

    let grid = generate_grid_from(&cube_sizes, direction);
    let largest_cubes = extract_largest_cubes_from(grid);

    let expected_side_lengths = get_expected_side_lengths_from(&cube_sizes);
    assert_eq!(largest_cubes.len(), expected_side_lengths.len());

    for (idx, largest_cube) in largest_cubes.iter().enumerate() {
        assert_eq!(
            largest_cube.side_length as usize,
            expected_side_lengths[idx]
        );
    }
}

//Test Case 7   		(Key = 1.3.3.1.)
#[test]
fn case7() {}

//Test Case 8   		(Key = 1.3.3.2.)
#[test]
fn case8() {}

//Test Case 9   		(Key = 1.4.1.1.)
#[test]
fn case9() {}

//Test Case 10  		(Key = 1.4.1.2.)
#[test]
fn case10() {}

//Test Case 11  		(Key = 1.4.2.1.)
#[test]
fn case11() {}

//Test Case 12  		(Key = 1.4.2.2.)
#[test]
fn case12() {}

//Test Case 13  		(Key = 1.4.3.1.)
#[test]
fn case13() {}

//Test Case 14  		(Key = 1.4.3.2.)
#[test]
fn case14() {}

//Test Case 15  		(Key = 1.5.1.1.)
#[test]
fn case15() {}

//Test Case 16  		(Key = 1.5.1.2.)
#[test]
fn case16() {}

//Test Case 17  		(Key = 1.5.2.1.)
#[test]
fn case17() {}

//Test Case 18  		(Key = 1.5.2.2.)
#[test]
fn case18() {}

//Test Case 19  		(Key = 1.5.3.1.)
#[test]
fn case19() {}

//Test Case 20  		(Key = 1.5.3.2.)
#[test]
fn case20() {}

//Test Case 21  		(Key = 1.6.1.1.)
#[test]
fn case21() {}

//Test Case 22  		(Key = 1.6.1.2.)
#[test]
fn case22() {}

//Test Case 23  		(Key = 1.6.2.1.)
#[test]
fn case23() {}

//Test Case 24  		(Key = 1.6.2.2.)
#[test]
fn case24() {}

//Test Case 25  		(Key = 1.6.3.1.)
#[test]
fn case25() {}

//Test Case 26  		(Key = 1.6.3.2.)
#[test]
fn case26() {}

//Test Case 27  		(Key = 1.7.1.1.)
#[test]
fn case27() {}

//Test Case 28  		(Key = 1.7.1.2.)
#[test]
fn case28() {}

//Test Case 29  		(Key = 1.7.2.1.)
#[test]
fn case29() {}

//Test Case 30  		(Key = 1.7.2.2.)
#[test]
fn case30() {
    let cube_sizes = BLOCKS_SIX;
    let direction = Direction::Vertical;

    let grid = generate_grid_from(&cube_sizes, direction);
    let largest_cubes = extract_largest_cubes_from(grid);

    let expected_side_lengths = get_expected_side_lengths_from(&cube_sizes);
    assert_eq!(largest_cubes.len(), expected_side_lengths.len());

    for (idx, largest_cube) in largest_cubes.iter().enumerate() {
        assert_eq!(
            largest_cube.side_length as usize,
            expected_side_lengths[idx]
        );
    }
}

//Test Case 31  		(Key = 1.7.3.1.)
#[test]
fn case31() {}

//Test Case 32  		(Key = 1.7.3.2.)
#[test]
fn case32() {}

//Test Case 33  		(Key = 2.4.1.1.)
#[test]
fn case33() {}

//Test Case 34  		(Key = 2.4.1.2.)
#[test]
fn case34() {}

//Test Case 35  		(Key = 2.4.2.1.)
#[test]
fn case35() {}

//Test Case 36  		(Key = 2.4.2.2.)
#[test]
fn case36() {}

//Test Case 37  		(Key = 2.4.3.1.)
#[test]
fn case37() {}

//Test Case 38  		(Key = 2.4.3.2.)
#[test]
fn case38() {}

//Test Case 39  		(Key = 2.5.1.1.)
#[test]
fn case39() {}

//Test Case 40  		(Key = 2.5.1.2.)
#[test]
fn case40() {}

//Test Case 41  		(Key = 2.5.2.1.)
#[test]
fn case41() {}

//Test Case 42  		(Key = 2.5.2.2.)
#[test]
fn case42() {}

//Test Case 43  		(Key = 2.5.3.1.)
#[test]
fn case43() {}

//Test Case 44  		(Key = 2.5.3.2.)
#[test]
fn case44() {}

//Test Case 45  		(Key = 2.6.1.1.)
#[test]
fn case45() {}

//Test Case 46  		(Key = 2.6.1.2.)
#[test]
fn case46() {}

//Test Case 47  		(Key = 2.6.2.1.)
#[test]
fn case47() {}

//Test Case 48  		(Key = 2.6.2.2.)
#[test]
fn case48() {}

//Test Case 49  		(Key = 2.6.3.1.)
#[test]
fn case49() {}

//Test Case 50  		(Key = 2.6.3.2.)
#[test]
fn case50() {}

//Test Case 51  		(Key = 2.7.1.1.)
#[test]
fn case51() {}

//Test Case 52  		(Key = 2.7.1.2.)
#[test]
fn case52() {}

//Test Case 53  		(Key = 2.7.2.1.)
#[test]
fn case53() {}

//Test Case 54  		(Key = 2.7.2.2.)
#[test]
fn case54() {}

//Test Case 55  		(Key = 2.7.3.1.)
#[test]
fn case55() {}

//Test Case 56  		(Key = 2.7.3.2.)
#[test]
fn case56() {}
