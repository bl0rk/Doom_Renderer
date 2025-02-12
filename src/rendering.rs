use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{data::{GameData, PlayerData}, Vec2, WORLD_MAP};

pub fn ver_line(x: i32, draw_start: i32, draw_end: i32, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let _ = canvas.draw_line(Point::new(x, draw_start), Point::new(x, draw_end));
}


pub fn render_view(player: &PlayerData, canvas: &mut Canvas<Window>, game_data: &GameData) {
    let mut pos_x = player.pos.x;
    let mut pos_y = player.pos.y;
    
    let mut dir_x = player.dir.x;
    let mut dir_y = player.dir.y;

    let mut plane_x = player.cam.x;
    let mut plane_y = player.cam.y;


    for x in 0..game_data.width as i32 {
        let mut camera_x = 2.0 * (x as f64) / (game_data.width as f64) - 1.0;
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

        loop {
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
                break;
            }
        }

        if side == 0 {
            perp_wall_dist = (side_dist_x - delta_dist_x);
        } else {
            perp_wall_dist = (side_dist_y - delta_dist_y);
        }

        let lineheight: i32 = ((game_data.height as f64)/perp_wall_dist) as i32;

        let mut draw_start: i32 = -lineheight / 2 + (game_data.height as i32) / 2;
        let mut draw_end: i32 = lineheight / 2 + (game_data.height as i32) / 2;

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

        ver_line(x, draw_start - player.ver, draw_end - player.ver, color, canvas);
    }
}
