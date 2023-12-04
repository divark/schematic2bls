use super::Brick;

fn right_to_center_coord(right_coord: usize, size: u32) -> f32 {
    right_coord as f32 - (size as f32 / 2.0)
}

impl Brick {
    pub fn new(size: u32) -> Brick {
        Brick {
            position: (0.0, 0.0, size as f32 / 4.0),
            size,
            floored: true,
        }
    }

    pub fn from_right_coordinate(
        size: u32,
        min_size: u32,
        right_position: (usize, usize, usize),
    ) -> Brick {
        let x = right_to_center_coord(right_position.0, size);
        let y = right_to_center_coord(right_position.1, size);
        let floored = right_position.2 == size as usize;

        Brick {
            position: (
                (x / 2.0) - (min_size as f32 / 4.0),
                (y / 2.0) - (min_size as f32 / 4.0),
                (right_position.2 as f32 / 2.0) - (size as f32 / 4.0),
            ),
            size,
            floored,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::blockland::save_file::to_save_file_output;

    use super::*;

    #[test]
    fn place_one_4x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/4xCube.bls").to_string();
        let actual = to_save_file_output(&vec![Brick::new(4)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4x_cube_tower() {
        let size = 4;

        let mut bricks = Vec::new();
        for i in 1..=size {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/4xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_4x_cubes() {
        let size = 4;

        let desired_x_coordinate = 4;
        let desired_y_coordinates = vec![4, 8, 12, 16];
        let desired_z_coordinate = 4;

        let mut bricks = Vec::new();
        for y_coord in desired_y_coordinates {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/4xCubesLine.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_8_cube() {
        let min_size = 4;

        let bricks = vec![
            Brick::from_right_coordinate(4, min_size, (4, 4, 4)),
            Brick::from_right_coordinate(8, min_size, (12, 8, 8)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/4-8Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_16_cube() {
        let min_size = 4;

        let bricks = vec![
            Brick::from_right_coordinate(4, min_size, (4, 4, 4)),
            Brick::from_right_coordinate(16, min_size, (20, 16, 16)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/4-16Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_32_cube() {
        let min_size = 4;

        let bricks = vec![
            Brick::from_right_coordinate(4, min_size, (4, 4, 4)),
            Brick::from_right_coordinate(32, min_size, (36, 32, 32)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/4-32Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_64_cube() {
        let min_size = 4;

        let bricks = vec![
            Brick::from_right_coordinate(4, min_size, (4, 4, 4)),
            Brick::from_right_coordinate(64, min_size, (68, 64, 64)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/4-64Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_8x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/8xCube.bls").to_string();
        let actual = to_save_file_output(&vec![Brick::new(8)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_16_cube() {
        let min_size = 8;

        let bricks = vec![
            Brick::from_right_coordinate(8, min_size, (8, 8, 8)),
            Brick::from_right_coordinate(16, min_size, (24, 16, 16)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/8-16Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_32_cube() {
        let min_size = 8;

        let bricks = vec![
            Brick::from_right_coordinate(8, min_size, (8, 8, 8)),
            Brick::from_right_coordinate(32, min_size, (40, 32, 32)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/8-32Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_64_cube() {
        let min_size = 8;

        let bricks = vec![
            Brick::from_right_coordinate(8, min_size, (8, 8, 8)),
            Brick::from_right_coordinate(64, min_size, (72, 64, 64)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/8-64Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16_32_cube() {
        let min_size = 16;

        let bricks = vec![
            Brick::from_right_coordinate(16, min_size, (16, 16, 16)),
            Brick::from_right_coordinate(32, min_size, (48, 32, 32)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/16-32Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16_64_cube() {
        let min_size = 16;

        let bricks = vec![
            Brick::from_right_coordinate(16, min_size, (16, 16, 16)),
            Brick::from_right_coordinate(64, min_size, (80, 64, 64)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/16-64Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_32_64_cube() {
        let min_size = 32;

        let bricks = vec![
            Brick::from_right_coordinate(32, min_size, (32, 32, 32)),
            Brick::from_right_coordinate(64, min_size, (96, 64, 64)),
        ];

        let expected = include_str!("../../assets/brick_comparisons/32-64Cube.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_8x_cubes() {
        let size = 8;

        let desired_x_coordinate = 8;
        let desired_y_coordinates = vec![8, 16, 24, 32];
        let desired_z_coordinate = 8;

        let mut bricks = Vec::new();
        for y_coord in desired_y_coordinates {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/8xCubesLine.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8x_cube_tower() {
        let size = 8;

        let mut bricks = Vec::new();
        for i in 1..=4 {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/8xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_16x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/16xCube.bls").to_string();
        let actual = to_save_file_output(&vec![Brick::new(16)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_16x_cubes() {
        let size = 16;

        let desired_x_coordinate = 16;
        let desired_y_coordinates = vec![16, 32, 48, 64];
        let desired_z_coordinate = 16;

        let mut bricks = Vec::new();
        for y_coord in desired_y_coordinates {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/16xCubesLine.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16x_cube_tower() {
        let size = 16;

        let mut bricks = Vec::new();
        for i in 1..=4 {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/16xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_32x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/32xCube.bls").to_string();
        let actual = to_save_file_output(&vec![Brick::new(32)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_32x_cubes() {
        let size = 32;

        let desired_x_coordinate = 32;
        let desired_y_coordinates = vec![32, 64, 96, 128];
        let desired_z_coordinate = 32;

        let mut bricks = Vec::new();
        for y_coord in desired_y_coordinates {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/32xCubesLine.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_32x_cube_tower() {
        let size = 32;

        let mut bricks = Vec::new();
        for i in 1..=4 {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/32xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_64x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/64xCube.bls").to_string();
        let actual = to_save_file_output(&vec![Brick::new(64)]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_64x_cubes() {
        let size = 64;

        let desired_x_coordinate = 64;
        let desired_y_coordinates = vec![64, 128, 192, 256];
        let desired_z_coordinate = 64;

        let mut bricks = Vec::new();
        for y_coord in desired_y_coordinates {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/64xCubesLine.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_64x_cube_tower() {
        let size = 64;

        let mut bricks = Vec::new();
        for i in 1..=4 {
            bricks.push(Brick::from_right_coordinate(
                size,
                size,
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/64xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_increasing_cubes_scale() {
        let expected = include_str!("../../assets/brick_comparisons/CubeScale.bls").to_string();

        let min_size = 4;
        let bricks = vec![
            Brick::from_right_coordinate(4, min_size, (4, 4, 4)),
            Brick::from_right_coordinate(8, min_size, (12, 8, 8)),
            Brick::from_right_coordinate(16, min_size, (28, 16, 16)),
            Brick::from_right_coordinate(32, min_size, (60, 32, 32)),
            Brick::from_right_coordinate(64, min_size, (124, 64, 64)),
        ];
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }
}
