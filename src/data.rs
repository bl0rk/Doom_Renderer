use std::ops;

pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

impl Vec2<f64> {
    pub fn len(self: &Self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn refmul(&self, rhs: f64) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }

    pub fn refadd(&self, rhs: Vec2<f64>) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

    /// Rotates the vector by some degrees. >0 => Left; <0 => Right;
    pub fn rotate(&mut self, degrees: f64) {
        let old_x = self.x;
        self.x = self.x * (degrees).cos() - self.y * (degrees).sin();
        self.y = old_x * (degrees).sin() + self.y * (degrees).cos();
    }
}

impl ops::Mul<f64> for Vec2<f64> {
    type Output = Self;

    fn mul(self: Self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::MulAssign<f64> for Vec2<f64> {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

pub struct PlayerData {
    pub pos: Vec2<f64>,
    pub dir: Vec2<f64>,
    pub cam: Vec2<f64>,
    pub ver: i32
}

impl PlayerData {
    pub fn new(pos: Vec2<f64>, dir: Vec2<f64>, cam: Vec2<f64>) -> PlayerData {
        PlayerData { pos, dir, cam, ver: 0 }
    }
}

pub struct GameData {
    pub mouse_set: bool,
    pub map_view: bool,
    pub width: u32,
    pub height: u32
}

impl GameData {
    pub fn new(mouse_set: bool, map_view: bool, width: u32, height: u32) -> Self {
        GameData {
            mouse_set,
            map_view,
            width,
            height
        }
    }

    pub fn update_sizes(&mut self, sizes: (u32, u32)) {
        self.width = sizes.0;
        self.height= sizes.1;
    }
}
