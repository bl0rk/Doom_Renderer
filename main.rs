extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// x = !^ | y = ->
const WORLD_MAP: [[i32; 10]; 10] = [
    [1, 1, 1, 1, 1, 1 ,1 ,1 ,1 ,1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 3, 3, 3, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 4, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 4, 0, 0, 0, 2, 2, 0, 1],
    [1, 0, 4, 0, 0, 0, 0, 2, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];

pub fn ver_line(x: i32, draw_start: i32, draw_end: i32, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let _ = canvas.draw_line(Point::new(x, draw_start), Point::new(x, draw_end));
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Bweh 2", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    
    
    let mut event_pump = sdl_context.event_pump()?;

    let mut pos_x = 1.5;
    let mut pos_y = 1.5;
    let mut dir_x = -1.0;
    let mut dir_y = 0.0;
    let mut plane_x = 0.0;
    let mut plane_y = 0.66;

    let mut time = 0.0;
    let mut old_time = 0.0;

    let timer = sdl_context.timer()?;

    'running: loop {
        canvas.set_draw_color(Color::MAGENTA);
        canvas.clear();

        old_time = time;
        time = timer.ticks() as f64;
        let frame_time = (time - old_time) / 1000.0;

        let move_speed: f64 = frame_time * 5.0;
        let rotate_speed: f64 = frame_time * 3.0;

        // Rotate Left/Right
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            let old_dir_x = dir_x;
            dir_x = dir_x * (rotate_speed).cos() - dir_y * (rotate_speed).sin();
            dir_y = old_dir_x * (rotate_speed).sin() + dir_y * (rotate_speed).cos();

            let old_plane_x = plane_x;
            plane_x = plane_x * (rotate_speed).cos() - plane_y * (rotate_speed).sin();
            plane_y = old_plane_x * (rotate_speed).sin() + plane_y * (rotate_speed).cos();
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            let old_dir_x = dir_x;
            dir_x = dir_x * (-rotate_speed).cos() - dir_y * (-rotate_speed).sin();
            dir_y = old_dir_x * (-rotate_speed).sin() + dir_y * (-rotate_speed).cos();

            let old_plane_x = plane_x;
            plane_x = plane_x * (-rotate_speed).cos() - plane_y * (-rotate_speed).sin();
            plane_y = old_plane_x * (-rotate_speed).sin() + plane_y * (-rotate_speed).cos();

        }

        // Move Forward/Backward
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            if WORLD_MAP[(pos_x + dir_x * move_speed) as usize][pos_y as usize] == 0 {
                pos_x += dir_x * move_speed;
            }

            if WORLD_MAP[pos_x as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                pos_y += dir_y * move_speed;
            }
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            if WORLD_MAP[(pos_x - dir_x * move_speed) as usize][pos_y as usize] == 0 {
                pos_x -= dir_x * move_speed;
            }

            if WORLD_MAP[pos_x as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                pos_y -= dir_y * move_speed;
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Rendering
        
        for x in 0..WIDTH as i32 {
            let mut camera_x = 2.0 * (x as f64) / (WIDTH as f64) - 1.0;
            let mut ray_dir_x = dir_x + plane_x * camera_x;
            let mut ray_dir_y = dir_y + plane_y * camera_x;

            let mut map_x: i32 = pos_x as i32;
            let mut map_y: i32 = pos_y as i32;

            let mut side_dist_x: f64;
            let mut side_dist_y: f64;

            let mut delta_dist_x: f64 = if ray_dir_x == 0.0 {1e30} else {(1.0/ray_dir_x).abs()};
            let mut delta_dist_y: f64 = if ray_dir_y == 0.0 {1e30} else {(1.0/ray_dir_y).abs()};

            let mut perp_wall_dist: f64;

            let mut step_x: i32;
            let mut step_y: i32;

            let mut hit: i32 = 0;
            let mut side: i32 = 0;

            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (pos_x - (map_x as f64)) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = ((map_x as f64) + 1.0 - pos_x) * delta_dist_x;
            }

            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (pos_y - (map_y as f64)) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = ((map_y as f64) + 1.0 - pos_y) * delta_dist_y;
            }

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if WORLD_MAP[map_x as usize][map_y as usize] != 0 {
                    hit = 1;
                }
            }

            if side == 0 {
                perp_wall_dist = (side_dist_x - delta_dist_x);
            } else {
                perp_wall_dist = (side_dist_y - delta_dist_y);
            }

            let lineheight: i32 = ((HEIGHT as f64)/perp_wall_dist) as i32;

            let mut draw_start: i32 = -lineheight / 2 + (HEIGHT as i32) / 2;
            if draw_start < 0 { draw_start = 0; }
            let mut draw_end: i32 = lineheight / 2 + (HEIGHT as i32) / 2;
            if draw_end >= (HEIGHT as i32) { draw_end = (HEIGHT as i32) - 1; }

            let mut color: Color;

            color = match WORLD_MAP[map_x as usize][map_y as usize] {
                1 => Color::CYAN,
                2 => Color::BLUE,
                3 => Color::GREEN,
                4 => Color::WHITE,
                _ => Color::GREY
            };

            if side == 1 {
                let (r, g, b, a) = color.rgba();
                color = Color::RGBA(r/2, g/2, b/2, a/2);
            }

            ver_line(x, draw_start, draw_end, color, &mut canvas);
        }

        // End Rendering

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
