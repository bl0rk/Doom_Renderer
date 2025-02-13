use std::ops;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::image::{LoadSurface, LoadTexture};

pub struct Vec2<T> {
    pub x: T,
    pub y: T
}
impl<T> Vec2<T> { pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

impl Vec2<f64> {
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
    pub textured: bool,
    pub width: u32,
    pub height: u32,
    pub last_mouse_pos: Vec2<i32>,
    pub textures: Vec<LoadedTexture>
}

impl GameData {
    pub fn new(mouse_set: bool, map_view: bool, width: u32, height: u32) -> Self {
        GameData {
            mouse_set,
            map_view,
            textured: false,
            width,
            height,
            last_mouse_pos: Vec2::new(0, 0),
            textures: Vec::new()
        }
    }

    pub fn update_sizes(&mut self, sizes: (u32, u32)) {
        self.width = sizes.0;
        self.height= sizes.1;
    }

    pub fn add_texture_from_file(&mut self, filename: &str) -> Result<(), String> {
        let sur = Surface::from_file(filename)?.into_canvas()?;
        let (sur_width, sur_height) = {(sur.surface().width(), sur.surface().height())};

        self.textures.push(
            LoadedTexture::new(
                sur_width,
                sur_height,
                sur.read_pixels(Rect::new(0, 0, sur_width, sur_height), sdl2::pixels::PixelFormatEnum::RGBA32)?
            )
        );

        Ok(())
    }
}

pub struct EndGame(pub bool);

pub struct LoadedTexture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>
}

impl LoadedTexture {
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        LoadedTexture { width, height, pixels }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8, u8, u8)>  {
        if x >= self.width as usize || y >= self.height as usize {
            return Option::None;
        }

        let pos = (x + y*self.height as usize) * 4;

        if pos+3 >= self.pixels.len() {
            return None;
        }

        Some((
            self.pixels.get(pos).unwrap().clone(),
            self.pixels.get(pos+1).unwrap().clone(),
            self.pixels.get(pos+2).unwrap().clone(),
            self.pixels.get(pos+3).unwrap().clone()
        ))
    }
}
