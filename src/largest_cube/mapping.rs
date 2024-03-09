#[derive(Clone)]
pub struct GridSizes {
    pub x_len: usize,
    pub y_len: usize,
    pub z_len: usize,
}

pub struct GridReader {
    sizes: GridSizes,
    grid: Vec<u16>,
}

pub fn idx_1d_from(x: usize, y: usize, z: usize, sizes: &GridSizes) -> usize {
    (z * sizes.x_len * sizes.y_len) + (y * sizes.x_len) + x
}

pub fn idx_3d_from(mut index_1d: usize, sizes: &GridSizes) -> (usize, usize, usize) {
    let z = index_1d / (sizes.x_len * sizes.y_len);
    index_1d -= z * sizes.x_len * sizes.y_len;
    let y = index_1d / sizes.x_len;
    let x = index_1d % sizes.x_len;

    (x, y, z)
}

impl GridReader {
    pub fn new(length: usize, width: usize, height: usize) -> GridReader {
        GridReader {
            sizes: GridSizes {
                x_len: length,
                y_len: width,
                z_len: height,
            },
            grid: vec![0; length * width * height],
        }
    }

    pub fn size(&self) -> &GridSizes {
        &self.sizes
    }

    pub fn size_cloned(&self) -> GridSizes {
        self.sizes.clone()
    }

    pub fn idx_1d_from(&self, i: usize, j: usize, k: usize) -> usize {
        idx_1d_from(i, j, k, self.size())
    }

    pub fn get(&self, i: usize, j: usize, k: usize) -> u16 {
        let idx = self.idx_1d_from(i, j, k);
        *self.grid.get(idx).unwrap()
    }

    pub fn get_mut(&mut self, i: usize, j: usize, k: usize) -> Option<&mut u16> {
        let idx = self.idx_1d_from(i, j, k);
        self.grid.get_mut(idx)
    }

    pub fn data(&self) -> &Vec<u16> {
        &self.grid
    }
}

pub fn grid_to_largest_cubes(grid: Vec<Vec<Vec<bool>>>, scale: u16) -> GridReader {
    let length = grid.len();
    let width = grid.first().unwrap_or(&Vec::new()).len();
    let height = grid
        .first()
        .unwrap_or(&Vec::new())
        .first()
        .unwrap_or(&Vec::new())
        .len();

    let mut largest_cube = GridReader::new(length + 1, width + 1, height + 1);

    for i in 1..=length {
        for j in 1..=width {
            for k in 1..=height {
                if grid[i - 1][j - 1][k - 1] {
                    let smallest_prior_cube = largest_cube
                        .get(i, j, k - 1)
                        .min(largest_cube.get(i, j - 1, k - 1))
                        .min(largest_cube.get(i - 1, j, k - 1))
                        .min(largest_cube.get(i - 1, j - 1, k - 1))
                        .min(largest_cube.get(i, j - 1, k))
                        .min(largest_cube.get(i - 1, j - 1, k))
                        .min(largest_cube.get(i - 1, j, k));

                    let result = largest_cube.get_mut(i, j, k).unwrap();
                    *result = smallest_prior_cube + scale;
                }
            }
        }
    }

    largest_cube
}
