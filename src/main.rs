extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod data;
mod rendering;
mod config;
use data::{Vec2, PlayerData};
use rendering::{cast_ray3, cast_ray4};
use config::{WIDTH, HEIGHT, GAME_MAP};

// TODO: Fix the fisheye effect.

pub fn ver_line(x: i32, y: i32, length: i32, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let _ = canvas.draw_line(Point::new(x, y), Point::new(x, y+length));
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
    let timer = sdl_context.timer()?;

    let mut time_now = timer.performance_counter();
    let mut time_last: u64 = 0;
    let mut delta_time: f64 = 0.0;

    let mut move_speed: f32 = 0.0;

    let mut map_view = true;

    // Game Setup
    let mut player = PlayerData::new(
        Vec2::new(1.5, 1.5),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 0.66)
    );

    'running: loop {
        time_last = time_now;
        time_now = timer.performance_counter();
        delta_time = (((time_now-time_last)) as f64 / timer.performance_frequency() as f64) as f64;

        move_speed = 3.0 * delta_time as f32;

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
                    player.dir.rotate_left(1.0 * delta_time as f32);
                    player.cam.rotate_left(1.0 * delta_time as f32);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player.dir.rotate_right(1.0 * delta_time as f32);
                    player.cam.rotate_right(1.0 * delta_time as f32);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    if GAME_MAP[(player.pos.x + player.dir.x * move_speed) as usize][player.pos.y as usize] == 0 {
                        player.pos.x += player.dir.x * move_speed;
                    }

                    if GAME_MAP[player.pos.x as usize][(player.pos.y + player.dir.y * move_speed) as usize] == 0 {
                        player.pos.y += player.dir.y * move_speed;
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    if GAME_MAP[(player.pos.x - player.dir.x * move_speed) as usize][player.pos.y as usize] == 0 {
                        player.pos.x -= player.dir.x * move_speed;
                    }

                    if GAME_MAP[player.pos.x as usize][(player.pos.y - player.dir.y * move_speed) as usize] == 0 {
                        player.pos.y -= player.dir.y * move_speed;
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    map_view = !map_view;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        if map_view {
        canvas.set_draw_color(Color::WHITE);

        for y in GAME_MAP.iter().enumerate() {
            for x in y.1.iter().enumerate() {
                if *x.1 == 0 {
                    canvas.draw_rect(Rect::new(x.0 as i32*20, y.0 as i32*20, 20, 20));
                } else {
                    canvas.fill_rect(Rect::new(x.0 as i32*20, y.0 as i32*20, 20, 20));
                }
            }
        }

        canvas.set_draw_color(Color::BLACK);
        //canvas.draw_point(Point::new((player.pos.x*20.0) as i32, (player.pos.y*20.0) as i32));
        canvas.fill_rect(Rect::new((player.pos.x*20.0) as i32, (player.pos.y*20.0) as i32, 5, 5));

        canvas.draw_line(Point::new((player.pos.x*20.0) as i32, (player.pos.y*20.0) as i32),
            Point::new(((player.pos.x*20.0) + player.dir.x*20.0) as i32, ((player.pos.y*20.0) + player.dir.y*20.0) as i32)
        );

        } else {

        canvas.set_draw_color(Color::MAGENTA);
        let half_height: u32 = HEIGHT/2;
        canvas.fill_rect(Rect::new(0, 0, WIDTH, half_height))?;
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new(0, half_height as i32, WIDTH, half_height))?;

        // Draw Walls
        for x in 0..WIDTH as i32 {
            let cam_pos: f32 = (2.0*x as f32)/(WIDTH as f32)-1.0;
            let (hit_distance, hit_num) = cast_ray3(
                &player.pos,
                // Multiply the camera by the cam_pos and add it to the direction vector.
                &player.dir.refadd(player.cam.refmul(cam_pos))
            );
            
            //let (hit_distance, hit_x) = cast_ray4(&player.pos, &player.dir, &player.cam, cam_pos);

            let line_height = (HEIGHT as f32 / hit_distance) as i32;

            let col = match hit_num {
                1 => Color::WHITE,
                2 => Color::GRAY,
                3 => Color::RED,
                4 => Color::BLUE,
                _ => Color::CYAN
            };

            ver_line(x, HEIGHT as i32/2 - line_height/2, line_height, col, &mut canvas);

        }

        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
