extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::ops;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const GAME_MAP: [[i32; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1]
];

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

pub fn ver_line(x: i32, y: i32, length: i32, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let _ = canvas.draw_line(Point::new(x, y), Point::new(x, y+length));
}

pub fn cast_ray3(pos_vec: Vec2, dir_vec: Vec2) {
    let mut map_x: i32 = pos_vec.x.floor() as i32;
    let mut map_y: i32 = pos_vec.y.floor() as i32;

    let step_x: i32 = if dir_vec.x > 0.0 {1} else {-1};
    let step_y: i32 = if dir_vec.y > 0.0 {1} else {-1};

    // These need to be the vector lengths.
    
    // Get the length of the vector that collides with the next x or y axis.
    let mut side_dist_x: f32 = {
        // Get scalar to multiply the direction vector by.
        let c = (if dir_vec.x > 0.0 {pos_vec.x.ceil()} else {pos_vec.x.floor()}/dir_vec.x);
        // Calculate the length of the vector.
        ((dir_vec.x*c).powi(2) + (dir_vec.y*c).powi(2)).sqrt()
    };

    let mut side_dist_y: f32 = {
        let c = (if dir_vec.y > 0.0 {pos_vec.y.ceil()} else {pos_vec.y.floor()}/dir_vec.y);
        ((dir_vec.x*c).powi(2) + (dir_vec.y*c).powi(2)).sqrt()
    };

    // let mut side_dist_x: f32 = if dir_vec.x > 0.0 {pos_vec.x.ceil()-pos_vec.x} else {pos_vec.x-pos_vec.x.floor()};
    // let mut side_dist_y:f32 = if dir_vec.y > 0.0 {pos_vec.y.ceil()-pos_vec.y} else {pos_vec.y-pos_vec.y.floor()};
    
    // The distance between each new collission on either the x or y axis. If zero it will always
    // collide so make it impossible for it to be checked.
    let delta_x = if dir_vec.x == 0.0 {side_dist_x += f32::MAX; 0.0} else {(1.0 + (dir_vec.y.powi(2)/dir_vec.x.powi(2))).sqrt()};
    let delta_y = if dir_vec.y == 0.0 {side_dist_y += f32::MAX; 0.0} else {(1.0 + (dir_vec.x.powi(2)/dir_vec.y.powi(2))).sqrt()};

    loop {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_x;
            map_x += step_x;
        } else {
            side_dist_y += delta_y;
            map_y += step_y;
        }

        // Check for collision.
        if GAME_MAP[map_y as usize][map_x as usize] != 0 {
            return;
        }
    }
}

pub fn main() -> Result<(), String> {
    cast_ray3(Vec2::new(1.7, 3.0), Vec2::new(1.0, -1.0));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let _ = window.set_title("bweh");

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut cube_size = 20;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => cube_size += 5,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    if cube_size > 5 {
                        cube_size -= 5
                    }
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        let mut vertical_start = true;
        let mut current = true;

        for x in 0..WIDTH as i32 {
            if x%cube_size as i32 == 0  {
                vertical_start = !vertical_start;
            }

            current = vertical_start;

            for y in 0..(HEIGHT as i32/cube_size)+1 {
                ver_line(x, cube_size*y, cube_size, if current {Color::RGB(255, 255, 255)} else {Color::RGB(0, 0, 0)}, &mut canvas);
                current = !current;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
