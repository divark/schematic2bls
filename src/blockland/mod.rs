pub mod mapping;
pub mod save_file;

use std::fmt::Display;

#[derive(Clone)]
pub struct Brick {
    pub position: (f32, f32, f32),
    pub size: u16,
    floored: bool,
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cube_name = format!("{}x Cube", self.size);
        let z_idx = format!("{}", self.position.2);

        write!(
            f,
            "{}\" {} {} {} 0 {} 6  0 0 1 1 1",
            cube_name, self.position.0, self.position.1, z_idx, self.floored as usize
        )
    }
}
