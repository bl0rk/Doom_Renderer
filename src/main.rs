extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::TimerSubsystem;
use std::time::Duration;

mod data;
use data::{Vec2, PlayerData};

// TODO: Fix the fisheye effect.

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const GAME_MAP: [[i32; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];

pub fn ver_line(x: i32, y: i32, length: i32, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let _ = canvas.draw_line(Point::new(x, y), Point::new(x, y+length));
}

pub fn cast_ray3(pos_vec: &Vec2, dir_vec: &Vec2) -> (f32, bool) {
    let mut map_x: i32 = pos_vec.x.floor() as i32;
    let mut map_y: i32 = pos_vec.y.floor() as i32;

    let step_x: i32 = if dir_vec.x > 0.0 {1} else {-1};
    let step_y: i32 = if dir_vec.y > 0.0 {1} else {-1};

    // These need to be the vector lengths.
    
    // Get the length of the vector that collides with the next x or y axis.
    // Note: These make NaN if the vector lies directly on an axis.
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

    // The distance between each new collission on either the x or y axis. If zero it will always
    // collide so make it impossible for it to be checked.
    let delta_x = if dir_vec.x == 0.0 {side_dist_x = f32::MAX; 0.0} else {(1.0 + (dir_vec.y.powi(2)/dir_vec.x.powi(2))).sqrt()};
    let delta_y = if dir_vec.y == 0.0 {side_dist_y = f32::MAX; 0.0} else {(1.0 + (dir_vec.x.powi(2)/dir_vec.y.powi(2))).sqrt()};

    let mut hit_x = true;

    loop {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_x;
            map_x += step_x;
            hit_x = true;
        } else {
            side_dist_y += delta_y;
            map_y += step_y;
            hit_x = false;
        }

        // TODO: Find out why this is important.
        let perp_wall_dist = if hit_x {
            side_dist_x - delta_x
        } else {
            side_dist_y - delta_y
        };

        // Check for collision.
        if GAME_MAP[map_y as usize][map_x as usize] != 0 {
            return (perp_wall_dist, hit_x);
            // return if hit_x {(side_dist_x, hit_x)} else {(side_dist_y, hit_x)};
        }
    }
} pub fn main() -> Result<(), String> {
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
    let timer = sdl_context.timer()?;

    let mut time_now = timer.performance_counter();
    let mut time_last: u64 = 0;
    let mut delta_time: f64 = 0.0;

    // Game Setup
    let mut player = PlayerData::new(
        Vec2::new(1.7, 3.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 0.5)
    );

    'running: loop {
        time_last = time_now;
        time_now = timer.performance_counter();
        delta_time = (((time_now-time_last)) as f64 / timer.performance_frequency() as f64) as f64;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player.dir.rotate(1.0 * delta_time as f32);
                    player.cam.rotate(1.0 * delta_time as f32);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player.dir.rotate(-1.0 * delta_time as f32);
                    player.cam.rotate(-1.0 * delta_time as f32);
                },
                _ => {}
            }
        }

        //canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::MAGENTA);
        let half_height: u32 = HEIGHT/2;
        canvas.fill_rect(Rect::new(0, 0, WIDTH, half_height))?;
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, half_height as i32, WIDTH, half_height))?;

        // Draw Walls
        for x in 0..WIDTH as i32 {
            let cam_pos: f32 = (2.0*x as f32)/(WIDTH as f32)-1.0;
            let (hit_distance, hit_x) = cast_ray3(
                &player.pos,
                // Multiply the camera by the cam_pos and add it to the direction vector.
                &player.dir.refadd(player.cam.refmul(cam_pos))
            );

            let line_height = (HEIGHT as f32 / hit_distance) as i32;

            ver_line(x, (HEIGHT as i32/2 - line_height/2), line_height, if hit_x {Color::WHITE} else {Color::GRAY}, &mut canvas);

        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
