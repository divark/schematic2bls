pub mod mapping;
pub mod save_file;

use std::fmt::Display;

#[derive(Clone)]
pub struct Brick {
    pub position: (f32, f32, f32),
    pub size: u32,
    floored: bool,
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
