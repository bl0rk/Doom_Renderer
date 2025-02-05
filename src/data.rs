use std::ops;

pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn len(self: &Self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn refmul(&self, rhs: f32) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }

    pub fn refadd(&self, rhs: Vec2) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

    pub fn rotate_right(&mut self, degrees: f32) {
        let old_x = self.x;
        self.x = self.x * (-degrees).cos() - self.y * (-degrees).sin();
        self.y = old_x * (-degrees).sin() + self.y * (-degrees).cos();
    }

    pub fn rotate_left(&mut self, degrees: f32) {
        let old_x = self.x;
        self.x = self.x * (degrees).cos() - self.y * (degrees).sin();
        self.y = old_x * (degrees).sin() + self.y * (degrees).cos();
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self: Self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

pub struct PlayerData {
    pub pos: Vec2,
    pub dir: Vec2,
    pub cam: Vec2
}

impl PlayerData {
    pub fn new(pos: Vec2, dir: Vec2, cam: Vec2) -> PlayerData {
        PlayerData { pos, dir, cam }
    }
}
