use std::fmt::Display;

pub fn save_header() -> String {
    let warning_text = "This is a Blockland save file.  You probably shouldn't modify it cause you'll screw it up.";
    let mut header_body = "1\r\n\r\n".to_string();

    let data_lines = vec![
        "0.898039 0.000000 0.000000 1.000000",
        "0.898039 0.898039 0.000000 1.000000",
        "0.000000 0.498039 0.247059 1.000000",
        "0.200000 0.000000 0.800000 1.000000",
        "0.898039 0.898039 0.898039 1.000000",
        "0.749020 0.749020 0.749020 1.000000",
        "0.498039 0.498039 0.498039 1.000000",
        "0.200000 0.200000 0.200000 1.000000",
        "0.392157 0.192157 0.000000 1.000000",
        "0.901961 0.337255 0.078431 1.000000",
        "0.749020 0.176471 0.482353 1.000000",
        "0.384314 0.000000 0.113725 1.000000",
        "0.129412 0.266667 0.266667 1.000000",
        "0.000000 0.137255 0.329412 1.000000",
        "0.101961 0.458824 0.764706 1.000000",
        "1.000000 1.000000 1.000000 1.000000",
        "0.078431 0.078431 0.078431 1.000000",
        "1.000000 1.000000 1.000000 0.247059",
        "0.921569 0.513726 0.674510 1.000000",
        "1.000000 0.603922 0.419608 1.000000",
        "1.000000 0.874510 0.611765 1.000000",
        "0.956863 0.874510 0.784314 1.000000",
        "0.784314 0.921569 0.486275 1.000000",
        "0.537255 0.694118 0.549020 1.000000",
        "0.556863 0.929412 0.956863 1.000000",
        "0.694118 0.658824 0.901961 1.000000",
        "0.874510 0.556863 0.956863 1.000000",
        "0.666667 0.000000 0.000000 0.698039",
        "1.000000 0.498039 0.000000 0.698039",
        "0.988235 0.956863 0.000000 0.698039",
        "0.000000 0.470588 0.192157 0.698039",
        "0.000000 0.200000 0.639216 0.698039",
        "0.592157 0.156863 0.392157 0.694118",
        "0.549020 0.698039 1.000000 0.698039",
        "0.847059 0.847059 0.847059 0.698039",
        "0.098039 0.098039 0.098039 0.698039",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
        "1.000000 0.000000 1.000000 0.000000",
    ];

    for data in data_lines {
        header_body.push_str(&format!("{}\r\n", data));
    }

    format!("{}\r\n{}", warning_text, header_body)
}

pub fn save_bricks(bricks: &Vec<Brick>) -> String {
    let linecount_line = format!("Linecount {}", bricks.len());

    let mut brick_contents = String::new();
    for brick in bricks {
        brick_contents.push_str(&format!("{}\r\n", brick));
        brick_contents.push_str("+-OWNER 999999\r\n");
    }

    format!("{}\r\n{}", linecount_line, brick_contents)
}

pub fn to_save_file_output(bricks: &Vec<Brick>) -> String {
    let save_header = save_header();
    let save_bricks = save_bricks(bricks);

    format!("{}{}", save_header, save_bricks)
}

#[derive(Clone)]
pub struct Brick {
    pub position: (f32, f32, f32),
    pub size: u32,
    floored: bool,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

    pub fn next_to(&mut self, direction: Direction, other_brick: &Brick) {
        let square_radius = self.size as f32 / 2.0;
        self.position = other_brick.position;

        match direction {
            Direction::Up => self.position.1 = other_brick.position.1 + square_radius,
            Direction::Down => self.position.1 = other_brick.position.1 - square_radius,
            Direction::Left => self.position.0 = other_brick.position.0 - square_radius,
            Direction::Right => self.position.0 = other_brick.position.0 + square_radius,
        }
    }

    pub fn from_right_coordinate(size: u32, right_position: (usize, usize, usize)) -> Brick {
        let x = right_to_center_coord(right_position.0, size);
        let y = right_to_center_coord(right_position.1, size);
        let floored = right_position.2 == size as usize;

        Brick {
            position: (
                (x / 2.0) - (size as f32 / 4.0),
                (y / 2.0) - (size as f32 / 4.0),
                (right_position.2 as f32 / 2.0) - (size as f32 / 4.0),
            ),
            size,
            floored,
        }
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x Cube\" {} {} {} 0 {} 0  0 0 1 1 1",
            self.size, self.position.0, self.position.1, self.position.2, self.floored as usize
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_one_4x_cube() {
        let expected = include_str!("../assets/brick_comparisons/4xCube.bls").to_string();
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
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../assets/brick_comparisons/4xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_3x3_4x_cubes() {
        let mut bricks = vec![Brick::new(4)];
        let cross_direction_order = vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];

        for direction in cross_direction_order {
            let mut neighboring_brick = Brick::new(4);
            neighboring_brick.next_to(direction, &bricks[0]);

            bricks.push(neighboring_brick);
        }

        let corner_direction_order = vec![Direction::Left, Direction::Right];
        let top_and_bottom_bricks = bricks.iter().skip(3).cloned().collect::<Vec<Brick>>();
        for brick in top_and_bottom_bricks {
            for direction in &corner_direction_order {
                let mut neighboring_brick = Brick::new(4);
                neighboring_brick.next_to(*direction, &brick);

                bricks.push(neighboring_brick);
            }
        }

        let expected = include_str!("../assets/brick_comparisons/4xCubes.bls").to_string();
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
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../assets/brick_comparisons/4xCubesLine.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_8x_cube() {
        let expected = include_str!("../assets/brick_comparisons/8xCube.bls").to_string();
        let actual = to_save_file_output(&vec![Brick::new(8)]);

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
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../assets/brick_comparisons/8xCubesLine.bls").to_string();
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
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../assets/brick_comparisons/8xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_16x_cube() {
        let expected = include_str!("../assets/brick_comparisons/16xCube.bls").to_string();
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
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../assets/brick_comparisons/16xCubesLine.bls").to_string();
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
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../assets/brick_comparisons/16xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_32x_cube() {
        let expected = include_str!("../assets/brick_comparisons/32xCube.bls").to_string();
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
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../assets/brick_comparisons/32xCubesLine.bls").to_string();
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
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../assets/brick_comparisons/32xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_one_64x_cube() {
        let expected = include_str!("../assets/brick_comparisons/64xCube.bls").to_string();
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
                (desired_x_coordinate, y_coord, desired_z_coordinate),
            ));
        }

        let expected = include_str!("../assets/brick_comparisons/64xCubesLine.bls").to_string();
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
                (size as usize, size as usize, size as usize * i as usize),
            ));
        }
        let expected = include_str!("../assets/brick_comparisons/64xCubesTower.bls").to_string();
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }

    #[test]
    fn place_increasing_cubes_scale() {
        let expected = include_str!("../assets/brick_comparisons/CubeScale.bls").to_string();

        let bricks = vec![
            Brick::from_right_coordinate(4, (4, 4, 4)),
            Brick::from_right_coordinate(8, (12, 8, 8)),
            Brick::from_right_coordinate(16, (28, 16, 16)),
            Brick::from_right_coordinate(32, (60, 32, 32)),
            Brick::from_right_coordinate(64, (124, 64, 64)),
        ];
        let actual = to_save_file_output(&bricks);

        assert_eq!(expected, actual);
    }
}
