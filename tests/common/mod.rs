#[derive(Copy, Clone)]
pub enum Direction {
    XAxis,
    XYAxis,
    XZAxis,
    YAxis,
    YZAxis,
    ZAxis,
}

/// A mock-based tool used to create 3D Grids
/// compatible with the Maximal Cube algorithm.
pub struct CubePainter {
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
            Direction::XAxis => {
                x += cube_size;
            }
            Direction::XYAxis => {
                x += cube_size;
                y += cube_size;
            }
            Direction::XZAxis => {
                x += cube_size;
                z += cube_size;
            }
            Direction::YAxis => {
                y += cube_size;
            }
            Direction::YZAxis => {
                y += cube_size;
                z += cube_size;
            }
            Direction::ZAxis => {
                z += cube_size;
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

    /// Moves the next available position to draw by an amount.
    pub fn shift(&mut self, direction: Direction, amount: isize) {
        let x_shift = (self.next_idx.0 as isize + amount) as usize;
        let y_shift = (self.next_idx.1 as isize + amount) as usize;
        let z_shift = (self.next_idx.2 as isize + amount) as usize;

        match direction {
            Direction::XAxis => self.next_idx.0 = x_shift,
            Direction::XYAxis => {
                self.next_idx.0 = x_shift;
                self.next_idx.1 = y_shift;
            }
            Direction::XZAxis => {
                self.next_idx.0 = x_shift;
                self.next_idx.2 = z_shift;
            }
            Direction::YAxis => self.next_idx.1 = y_shift,
            Direction::YZAxis => {
                self.next_idx.1 = y_shift;
                self.next_idx.2 = z_shift;
            }
            Direction::ZAxis => self.next_idx.2 = z_shift,
        }
    }

    pub fn to_grid(&self) -> Vec<Vec<Vec<bool>>> {
        self.grid.clone()
    }
}
