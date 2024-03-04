#[derive(Copy, Clone)]
pub enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
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
