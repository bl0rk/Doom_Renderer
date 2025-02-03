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

pub fn cast_ray(posx: f32, posy: f32, dirx: f32, diry: f32) {
    // nextx; nexty; deltax; deltay;
    let mut next_x = Vec2::new(dirsx, diry);
    let mut next_y = Vec2::new(dirx, diry);

    let dir_vec2 = Vec2::new(dirx, diry);

    next_x *= if dirx > 0.0 {posx.ceil()/dirx} else {posx.floor()/dirx};
    next_y *= if diry > 0.0 {posy.ceil()/diry} else {posy.floor()/diry};

    let mut delta_x: f32 = 0.0;
    let mut delta_y: f32 = 0.0;

    delta_x = if dirx > 0.0 {(posx+next_x.x+1)/dirx} else {(posx+next_x.x-1)/dirx};
    delta_y = if diry > 0.0 {(posy+next_y.y+1)/dirx} else {(posy+next_y.y-1)/dirx};

    // Check both next, then when checking the delta ones go along one axis until the distance from
    // the player exceeds the distance from the next one on the other axis.

    loop {
        if (next_x.len() < next_y.len()) {
            // Check for collission

            next_x *= delta_x;
        }
    }
}

pub fn main() -> Result<(), String> {
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
