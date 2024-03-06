use super::Brick;

pub struct BrickBuilder {
    pub bricks: Vec<Brick>,
}

impl BrickBuilder {
    pub fn new() -> Self {
        BrickBuilder { bricks: Vec::new() }
    }

    pub fn with_brick(&mut self, brick: Brick) {
        self.bricks.push(brick);
    }

    /// Returns a collection of stacked bricks
    /// as a set of indexes that are of size 1.
    pub fn get_1x_cube_bundles(&self) -> Vec<Vec<usize>> {
        let mut filtered_1x_bricks = self
            .bricks
            .iter()
            .enumerate()
            .filter(|brick_info| brick_info.1.size == 1)
            .map(|brick_info| (brick_info.0, brick_info.1.position))
            .collect::<Vec<(usize, (f32, f32, f32))>>();

        filtered_1x_bricks.sort_unstable_by(|brick1, brick2| {
            (brick1.1 .0 as usize)
                .cmp(&(brick2.1 .0 as usize))
                .then((brick1.1 .1 as usize).cmp(&(brick2.1 .1 as usize)))
                .then((brick1.1 .2 as usize).cmp(&(brick2.1 .2 as usize)))
        });

        let mut bundles: Vec<Vec<usize>> = Vec::new();

        if filtered_1x_bricks.first().is_none() {
            return Vec::new();
        }
        let mut current_bundle: Vec<usize> = vec![filtered_1x_bricks.first().unwrap().0];

        for (brick_number, brick_info) in filtered_1x_bricks.iter().enumerate() {
            if brick_number == 0 {
                continue;
            }

            let brick = &self.bricks[brick_info.0];
            let previous_brick = &self.bricks[filtered_1x_bricks[brick_number - 1].0];

            if brick.is_above(&previous_brick) {
                current_bundle.push(brick_info.0);
            } else {
                bundles.push(current_bundle);
                current_bundle = vec![brick_info.0];
            }
        }

        if !current_bundle.is_empty() {
            bundles.push(current_bundle);
        }

        bundles
    }

    /// Changes each Brick in the given bundles Brick sets
    /// to either be labeled as the top or bottom of the
    /// 1x Cube Type, dictated by assign_1x_cube_name.
    pub fn map_1x_cube_bundles(&mut self, bundles: Vec<Vec<usize>>) {
        for bundle in bundles {
            self.assign_1x_cube_name(&bundle);
        }
    }

    /// Sets whether a 1x Brick in the provided Cube Bundle
    /// is the bottom or top half of the 1x Cube Type.
    fn assign_1x_cube_name(&mut self, cube_bundle: &Vec<usize>) {
        for (brick_number, brick_idx) in cube_bundle.iter().enumerate() {
            let brick = &mut self.bricks[*brick_idx];

            if brick_number == 0 || brick_number % 2 == 0 {
                brick.bottom_1x_cube = true;
            } else {
                brick.bottom_1x_cube = false;
            }
        }
    }

    pub fn build(&mut self) -> Vec<Brick> {
        let mut adjusted_coordinate_bricks = Vec::new();

        // 1x Cubes are a special case since a 1x Cube
        // actually comes in two parts for Blockland.
        let cubes_1x_idxes = self.get_1x_cube_bundles();
        self.map_1x_cube_bundles(cubes_1x_idxes);

        let min_size = self.bricks[0].size;

        for brick in &self.bricks {
            adjusted_coordinate_bricks.push(brick.calculate_right_offset(min_size));
        }

        adjusted_coordinate_bricks
    }
}

fn right_to_center_coord(right_coord: f32, size: u16) -> f32 {
    right_coord - (size as f32 / 2.0)
}

impl Brick {
    pub fn new(right_xyz_coord: (usize, usize, usize), size: u16) -> Brick {
        Brick {
            position: (
                right_xyz_coord.0 as f32,
                right_xyz_coord.1 as f32,
                right_xyz_coord.2 as f32,
            ),
            size,
            floored: true,
            bottom_1x_cube: false,
        }
    }

    fn calculate_right_offset(&self, min_size: u16) -> Brick {
        let x = right_to_center_coord(self.position.0, self.size);
        let y = right_to_center_coord(self.position.1, self.size);
        let floored = self.position.2 as usize == self.size as usize;

        let min_size_offset = if min_size > 1 {
            min_size as f32 / 4.0
        } else {
            0.0
        };

        let size_offset = if self.size > 1 {
            self.size as f32 / 4.0
        } else {
            0.3
        };

        Brick {
            position: (
                (x / 2.0) - min_size_offset,
                (y / 2.0) - min_size_offset,
                (self.position.2 / 2.0) - size_offset,
            ),
            size: self.size,
            floored,
            bottom_1x_cube: self.bottom_1x_cube,
        }
    }

    /// Returns whether this Brick is positioned
    /// above the other Brick.
    /// NOTE: This is only used before BrickBuilder
    /// runs the build function.
    fn is_above(&self, other: &Brick) -> bool {
        let is_x_aligned = self.position.0 == other.position.0;
        let is_y_aligned = self.position.1 == other.position.0;

        let other_below = self.position.2 - other.position.2 == 1.0;

        is_x_aligned && is_y_aligned && other_below
    }
}

#[cfg(test)]
mod tests {
    use crate::blockland::save_file::to_save_file_output;

    use super::*;

    #[test]
    fn place_one_4x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/4xCube.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((4, 4, 4), 4));
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4x_cube_tower() {
        let size = 4;

        let mut brick_builder = BrickBuilder::new();
        for i in 1..=size {
            brick_builder.with_brick(Brick::new(
                (size as usize, size as usize, size as usize * i as usize),
                size,
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/4xCubesTower.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_4x_cubes() {
        let size = 4;

        let desired_x_coordinate = 4;
        let desired_y_coordinates = vec![4, 8, 12, 16];
        let desired_z_coordinate = 4;

        let mut brick_builder = BrickBuilder::new();
        for y_coord in desired_y_coordinates {
            brick_builder.with_brick(Brick::new(
                (desired_x_coordinate, y_coord, desired_z_coordinate),
                size,
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/4xCubesLine.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_8_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((4, 4, 4), 4));
        brick_builder.with_brick(Brick::new((12, 8, 8), 8));

        let expected = include_str!("../../assets/brick_comparisons/4-8Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_4_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((8, 8, 8), 8));
        brick_builder.with_brick(Brick::new((12, 4, 4), 4));

        let expected = include_str!("../../assets/brick_comparisons/8-4Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_16_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((4, 4, 4), 4));
        brick_builder.with_brick(Brick::new((20, 16, 16), 16));

        let expected = include_str!("../../assets/brick_comparisons/4-16Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_32_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((4, 4, 4), 4));
        brick_builder.with_brick(Brick::new((36, 32, 32), 32));

        let expected = include_str!("../../assets/brick_comparisons/4-32Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_4_64_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((4, 4, 4), 4));
        brick_builder.with_brick(Brick::new((68, 64, 64), 64));

        let expected = include_str!("../../assets/brick_comparisons/4-64Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_8x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/8xCube.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((8, 8, 8), 8));
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_16_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((8, 8, 8), 8));
        brick_builder.with_brick(Brick::new((24, 16, 16), 16));

        let expected = include_str!("../../assets/brick_comparisons/8-16Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_32_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((8, 8, 8), 8));
        brick_builder.with_brick(Brick::new((40, 32, 32), 32));

        let expected = include_str!("../../assets/brick_comparisons/8-32Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8_64_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((8, 8, 8), 8));
        brick_builder.with_brick(Brick::new((72, 64, 64), 64));

        let expected = include_str!("../../assets/brick_comparisons/8-64Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16_8_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((16, 16, 16), 16));
        brick_builder.with_brick(Brick::new((24, 8, 8), 8));

        let expected = include_str!("../../assets/brick_comparisons/16-8Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16_4_8_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((16, 16, 16), 16));
        brick_builder.with_brick(Brick::new((20, 4, 4), 4));
        brick_builder.with_brick(Brick::new((28, 8, 8), 8));

        let expected = include_str!("../../assets/brick_comparisons/16-4-8Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16_32_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((16, 16, 16), 16));
        brick_builder.with_brick(Brick::new((48, 32, 32), 32));

        let expected = include_str!("../../assets/brick_comparisons/16-32Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16_64_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((16, 16, 16), 16));
        brick_builder.with_brick(Brick::new((80, 64, 64), 64));

        let expected = include_str!("../../assets/brick_comparisons/16-64Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_32_64_cube() {
        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((32, 32, 32), 32));
        brick_builder.with_brick(Brick::new((96, 64, 64), 64));

        let expected = include_str!("../../assets/brick_comparisons/32-64Cube.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_8x_cubes() {
        let size = 8;

        let desired_x_coordinate = 8;
        let desired_y_coordinates = vec![8, 16, 24, 32];
        let desired_z_coordinate = 8;

        let mut brick_builder = BrickBuilder::new();
        for y_coord in desired_y_coordinates {
            brick_builder.with_brick(Brick::new(
                (desired_x_coordinate, y_coord, desired_z_coordinate),
                size,
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/8xCubesLine.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_8x_cube_tower() {
        let size = 8;

        let mut brick_builder = BrickBuilder::new();
        for i in 1..=4 {
            brick_builder.with_brick(Brick::new(
                (size as usize, size as usize, size as usize * i as usize),
                size,
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/8xCubesTower.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_16x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/16xCube.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((16, 16, 16), 16));
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_16x_cubes() {
        let size = 16;

        let desired_x_coordinate = 16;
        let desired_y_coordinates = vec![16, 32, 48, 64];
        let desired_z_coordinate = 16;

        let mut brick_builder = BrickBuilder::new();
        for y_coord in desired_y_coordinates {
            brick_builder.with_brick(Brick::new(
                (desired_x_coordinate, y_coord, desired_z_coordinate),
                size,
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/16xCubesLine.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_16x_cube_tower() {
        let size = 16;

        let mut brick_builder = BrickBuilder::new();
        for i in 1..=4 {
            brick_builder.with_brick(Brick::new(
                (size as usize, size as usize, size as usize * i as usize),
                size,
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/16xCubesTower.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_32x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/32xCube.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((32, 32, 32), 32));
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_32x_cubes() {
        let size = 32;

        let desired_x_coordinate = 32;
        let desired_y_coordinates = vec![32, 64, 96, 128];
        let desired_z_coordinate = 32;

        let mut brick_builder = BrickBuilder::new();
        for y_coord in desired_y_coordinates {
            brick_builder.with_brick(Brick::new(
                (desired_x_coordinate, y_coord, desired_z_coordinate),
                size,
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/32xCubesLine.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_32x_cube_tower() {
        let size = 32;

        let mut brick_builder = BrickBuilder::new();
        for i in 1..=4 {
            brick_builder.with_brick(Brick::new(
                (size as usize, size as usize, size as usize * i as usize),
                size,
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/32xCubesTower.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_64x_cube() {
        let expected = include_str!("../../assets/brick_comparisons/64xCube.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((64, 64, 64), 64));
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x4_64x_cubes() {
        let size = 64;

        let desired_x_coordinate = 64;
        let desired_y_coordinates = vec![64, 128, 192, 256];
        let desired_z_coordinate = 64;

        let mut brick_builder = BrickBuilder::new();
        for y_coord in desired_y_coordinates {
            brick_builder.with_brick(Brick::new(
                (desired_x_coordinate, y_coord, desired_z_coordinate),
                size,
            ));
        }

        let expected = include_str!("../../assets/brick_comparisons/64xCubesLine.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_64x_cube_tower() {
        let size = 64;

        let mut brick_builder = BrickBuilder::new();
        for i in 1..=4 {
            brick_builder.with_brick(Brick::new(
                (size as usize, size as usize, size as usize * i as usize),
                size,
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/64xCubesTower.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_1x_cube_tower() {
        let size = 1;

        let mut brick_builder = BrickBuilder::new();
        for i in 1..=4 {
            brick_builder.with_brick(Brick::new(
                (size as usize, size as usize, size as usize * i as usize),
                size,
            ));
        }
        let expected = include_str!("../../assets/brick_comparisons/1xCubesTower.bls").to_string();
        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_increasing_cubes_scale() {
        let expected = include_str!("../../assets/brick_comparisons/CubeScale.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((4, 4, 4), 4));
        brick_builder.with_brick(Brick::new((12, 8, 8), 8));
        brick_builder.with_brick(Brick::new((28, 16, 16), 16));
        brick_builder.with_brick(Brick::new((60, 32, 32), 32));
        brick_builder.with_brick(Brick::new((124, 64, 64), 64));

        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_complete_increasing_cubes_scale() {
        let expected = include_str!("../../assets/brick_comparisons/CubeScaleNew.bls").to_string();

        let mut brick_builder = BrickBuilder::new();
        brick_builder.with_brick(Brick::new((1, 1, 1), 1));
        brick_builder.with_brick(Brick::new((3, 2, 2), 2));
        brick_builder.with_brick(Brick::new((7, 4, 4), 4));
        brick_builder.with_brick(Brick::new((15, 8, 8), 8));
        brick_builder.with_brick(Brick::new((31, 16, 16), 16));
        brick_builder.with_brick(Brick::new((63, 32, 32), 32));
        brick_builder.with_brick(Brick::new((127, 64, 64), 64));

        let actual = to_save_file_output(&brick_builder.build());

        assert_eq!(expected, actual);
    }
}
