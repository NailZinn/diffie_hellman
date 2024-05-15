#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const NONE: Self = Self { x: 0, y: 0 };
}