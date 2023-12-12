use super::LargestCube;

pub fn get_largest_cube(largest_cube_grid: &[Vec<Vec<usize>>]) -> Option<LargestCube> {
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
    use crate::largest_cube::mapping::grid_to_largest_cubes;

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
