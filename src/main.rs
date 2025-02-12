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
use data::{GameData, PlayerData, Vec2};
use rendering::render_view;
use config::{WIDTH, HEIGHT, WORLD_MAP};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let _ = window.set_title("Software Renderer");
    window.set_mouse_grab(true); // Prevent cursor from leaving window.

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let timer = sdl_context.timer()?;
    let mouse = sdl_context.mouse();

    mouse.show_cursor(false);

    // Game Setup
    let mut game_data = GameData::new(
        false,
        false,
        WIDTH,
        HEIGHT
    );

    let mut player = PlayerData::new(
        Vec2::new(1.5, 1.5),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 0.66)
    );

    let mut time = 0.0;
    let mut old_time = 0.0;

    let mut last_mouse_pos: Vec2<i32> = Vec2::new(0, 0);

    'running: loop {
        game_data.update_sizes(canvas.window().size());

        canvas.set_draw_color(Color::MAGENTA);
        canvas.clear();

        old_time = time;
        time = timer.ticks() as f64;
        let delta_time = (time - old_time) / 1000.0;

        let move_speed = 3.0 * delta_time;

        // Rotate Left/Right
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            // The number 67.5 (45+(45/2)) came through trail and error, 90 and 45 led the player
            // to strafe at an angle, for some reason. Do note that it appears that either 90 or 45
            // at least move in the correct direction but 67.5 requires a sign switch.
            let dir_x: f64 = player.dir.x * (-67.5f64).cos() - player.dir.y * (-67.5f64).sin();
            let dir_y: f64 = player.dir.x * (-67.5f64).sin() + player.dir.y * (-67.5f64).cos();

            let new_player_x = player.pos.x + dir_x * move_speed;
            let new_player_y = player.pos.y + dir_y * move_speed;

            if WORLD_MAP[new_player_x as usize][new_player_y as usize] == 0 {
                player.pos.x = new_player_x;
                player.pos.y = new_player_y;
            }
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            let dir_x: f64 = player.dir.x * (67.5f64).cos() - player.dir.y * (67.5f64).sin();
            let dir_y: f64 = player.dir.x * (67.5f64).sin() + player.dir.y * (67.5f64).cos();

            let new_player_x = player.pos.x + dir_x * move_speed;
            let new_player_y = player.pos.y + dir_y * move_speed;

            if WORLD_MAP[new_player_x as usize][new_player_y as usize] == 0 {
                player.pos.x = new_player_x;
                player.pos.y = new_player_y;
            }
        }

        // Move Forward/Backward
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            if WORLD_MAP[(player.pos.x + player.dir.x * move_speed) as usize][player.pos.y as usize] == 0 {
                player.pos.x += player.dir.x * move_speed;
            }

            if WORLD_MAP[player.pos.x as usize][(player.pos.y + player.dir.y * move_speed) as usize] == 0 {
                player.pos.y += player.dir.y * move_speed;
            }
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            if WORLD_MAP[(player.pos.x - player.dir.x * move_speed) as usize][player.pos.y as usize] == 0 {
                player.pos.x -= player.dir.x * move_speed;
            }

            if WORLD_MAP[player.pos.x as usize][(player.pos.y - player.dir.y * move_speed) as usize] == 0 {
                player.pos.y -= player.dir.y * move_speed;
            }
        }

        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Q) {
            if player.ver != 80 {
                player.ver += 4;
            }
        } else if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::E) {
            if player.ver != -80 {
                player.ver -= 4;
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
                Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
                } => {
                    game_data.map_view = !game_data.map_view;
                },
                Event::MouseButtonDown {
                    mouse_btn,
                    ..
                } => {
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Right => {
                            mouse.show_cursor(!mouse.is_cursor_showing());
                            let grabbed = canvas.window().mouse_grab();
                            canvas.window_mut().set_mouse_grab(!grabbed);
                        },
                        _ => {}
                    }
                },
                Event::MouseMotion {
                    x, y, ..
                } => {
                    if mouse.is_cursor_showing() {
                        continue;
                    }

                    // Once the mouse gets set back to the center, the MouseMotion event triggers
                    // and has to be ignored.
                    if game_data.mouse_set {
                        game_data.mouse_set = false;
                        continue;
                    }

                    let delta_x_norm = (last_mouse_pos.x - x).signum(); // 1 = left; -1 = right;
                    let rotate_by = delta_x_norm as f64 * delta_time;

                    player.dir.rotate(rotate_by/2.0);
                    player.cam.rotate(rotate_by/2.0);

                    let delta_y_norm = (y - last_mouse_pos.y).signum();
                    player.ver += (delta_y_norm as f64 * delta_time*100.0) as i32;
                    if player.ver.abs() == 100 {
                        player.ver = player.ver.signum() * 100;
                    }

                    // Warp mouse back to the center of the window.
                    let half_width = (game_data.width/2) as i32;
                    let half_height = (game_data.height/2) as i32;
                    mouse.warp_mouse_in_window(&canvas.window(), half_width, half_height as i32);
                    last_mouse_pos.set(half_width as i32, half_height as i32);
                    game_data.mouse_set = true;
                },
                _ => {}
            }
        }

        render_view(&player, &mut canvas, &game_data);

        if game_data.map_view {
            canvas.set_draw_color(Color::WHITE);

            for x in WORLD_MAP.iter().enumerate() {
                for y in x.1.iter().enumerate() {
                    let set_x = (x.0 as i32) * 20;
                    let set_y = (9 - y.0 as i32) * 20;

                    if *y.1 == 0 {
                        canvas.draw_rect(Rect::new(set_x, set_y, 20, 20));
                    } else {
                        canvas.fill_rect(Rect::new(set_x, set_y, 20, 20));
                    }
                }
            }

            canvas.set_draw_color(Color::BLACK);
            let player_x = (player.pos.x) * 20.0;
            let player_y = (10.0-player.pos.y) * 20.0;

            let _ = canvas.fill_rect(Rect::new(player_x as i32, player_y as i32, 5, 5));

            let _ = canvas.draw_line(
                Point::new(player_x as i32, player_y as i32),
                Point::new((player_x + player.dir.x * 20.0) as i32, (player_y - player.dir.y * 20.0) as i32)
            );

        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
