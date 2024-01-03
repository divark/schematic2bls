fn scale_start_idx(idx: usize, scale: usize) -> usize {
    idx * scale
}

fn scale_copy(
    scaled_up_grid: &mut [Vec<Vec<bool>>],
    entry_indices: (usize, usize, usize),
    scale: usize,
) {
    let scaled_x = scale_start_idx(entry_indices.0, scale);
    let scaled_y = scale_start_idx(entry_indices.1, scale);
    let scaled_z = scale_start_idx(entry_indices.2, scale);

    for i in scaled_x..scaled_x + scale {
        for j in scaled_y..scaled_y + scale {
            for k in scaled_z..scaled_z + scale {
                scaled_up_grid[i][j][k] = true;
            }
        }
    }
}

pub fn scale_grid(grid: &Vec<Vec<Vec<bool>>>, scale: usize) -> Vec<Vec<Vec<bool>>> {
    let mut scaled_up_grid = vec![
        vec![vec![false; grid[0][0].len() * scale]; grid[0].len() * scale];
        grid.len() * scale
    ];

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            for (k, entry) in col.iter().enumerate() {
                if !entry {
                    continue;
                }

                scale_copy(&mut scaled_up_grid, (i, j, k), scale);
            }
        }
    }

    scaled_up_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_1x1x1_to_2x2x2() {
        let scale = 2;

        let initial_grid = vec![vec![vec![true]; 1]; 1];

        let expected_scaled_grid = vec![vec![vec![true; scale]; scale]; scale];
        let actual_scaled_grid = scale_grid(&initial_grid, scale);

        assert_eq!(expected_scaled_grid, actual_scaled_grid);
    }

    #[test]
    fn scale_2x2x2_top_left_to_4x4x4_top_left() {
        let scale = 2;

        let mut initial_grid = vec![vec![vec![false; 2]; 2]; 2];
        initial_grid[0][0][0] = true;

        let mut expected_scaled_grid = vec![vec![vec![false; scale * 2]; scale * 2]; scale * 2];
        scale_copy(&mut expected_scaled_grid, (0, 0, 0), 2);

        let actual_scaled_grid = scale_grid(&initial_grid, scale);

        assert_eq!(expected_scaled_grid, actual_scaled_grid);
    }

    #[test]
    fn scale_2x2x2_top_right_to_4x4x4_top_right() {
        let scale = 2;

        let mut initial_grid = vec![vec![vec![false; 2]; 2]; 2];
        initial_grid[0][1][1] = true;

        let mut expected_scaled_grid = vec![vec![vec![false; scale * 2]; scale * 2]; scale * 2];
        scale_copy(&mut expected_scaled_grid, (0, 1, 1), 2);

        let actual_scaled_grid = scale_grid(&initial_grid, scale);

        assert_eq!(expected_scaled_grid, actual_scaled_grid);
    }
}
