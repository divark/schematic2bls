pub mod mapping;
pub mod save_file;

use std::fmt::Display;

#[derive(Clone)]
pub struct Brick {
    pub position: (f32, f32, f32),
    pub size: u16,
    floored: bool,
    pub bottom_1x_cube: bool,
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cube_name = if self.size > 1 {
            format!("{}x Cube", self.size)
        } else if self.bottom_1x_cube {
            String::from("1x Cube Bottom")
        } else {
            String::from("1x Cube Top")
        };

        let z_idx = if self.size > 1 {
            format!("{}", self.position.2)
        } else {
            format!("{:.1}", self.position.2)
        };

        write!(
            f,
            "{}\" {} {} {} 0 {} 0  0 0 1 1 1",
            cube_name, self.position.0, self.position.1, z_idx, self.floored as usize
        )
    }
}
