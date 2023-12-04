pub fn grid_to_largest_cubes(grid: Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<usize>>> {
    let length = grid.len();
    let width = grid[0].len();
    let height = grid[0][0].len();

    let mut largest_cube = vec![vec![vec![0; height + 1]; width + 1]; length + 1];

    for i in 1..=length {
        for j in 1..=width {
            for k in 1..=height {
                if grid[i - 1][j - 1][k - 1] {
                    let smallest_prior_cube = largest_cube[i][j][k - 1]
                        .min(largest_cube[i][j - 1][k - 1])
                        .min(largest_cube[i - 1][j][k - 1])
                        .min(largest_cube[i - 1][j - 1][k - 1])
                        .min(largest_cube[i][j - 1][k])
                        .min(largest_cube[i - 1][j - 1][k])
                        .min(largest_cube[i - 1][j][k]);

                    largest_cube[i][j][k] = smallest_prior_cube + 1;
                }
            }
        }
    }

    largest_cube
}
