use sdl2::{pixels::Color, rect::Point, render::{Canvas, Texture}, video::Window};

use crate::{data::{GameData, PlayerData}, Vec2, WORLD_MAP};

pub fn ver_line(x: i32, draw_start: i32, draw_end: i32, color: Color, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(color);
    let _ = canvas.draw_line(Point::new(x, draw_start), Point::new(x, draw_end));
}


pub fn render_view(player: &PlayerData, buffer: &mut Texture, game_data: &GameData) {
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

        let mut x_side = false;

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
                x_side = true;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                x_side = false;
            }

            if WORLD_MAP[map_x as usize][map_y as usize] != 0 {
                break;
            }
        }

        if x_side {
            perp_wall_dist = (side_dist_x - delta_dist_x);
        } else {
            perp_wall_dist = (side_dist_y - delta_dist_y);
        }

        let lineheight: i32 = ((game_data.height as f64)/perp_wall_dist) as i32;

        let mut draw_start: i32 = -lineheight / 2 + (game_data.height as i32) / 2;
        if draw_start < 0 {
            draw_start = 0;
        }
        let mut draw_end: i32 = lineheight / 2 + (game_data.height as i32) / 2;
        if draw_end >= (game_data.height as i32) {
            draw_end = (game_data.height as i32) - 1;
        }

        // Texture Handling
        let mut texture = match WORLD_MAP[map_x as usize][map_y as usize] {
            1 => game_data.textures.get(0).unwrap(),
            2 => game_data.textures.get(1).unwrap(),
            3 => game_data.textures.get(2).unwrap(),
            4 => game_data.textures.get(3).unwrap(),
            _ => game_data.textures.get(4).unwrap()
        };

        // The coordinate on the wall which corresponds to the x-coordinate on the texture.
        let mut wall_x: f64;
        if x_side {
            wall_x = pos_y + perp_wall_dist * ray_dir_y;
        } else {
            wall_x = pos_x + perp_wall_dist * ray_dir_x;
        }
        wall_x -= wall_x.floor();

        // The x-coordinate on the texture.
        let mut tex_x = (wall_x * texture.width as f64) as u32;

        let step = texture.height as f64 / lineheight as f64;
        let mut tex_pos = (draw_start as f64 - game_data.height as f64/2.0 + lineheight as f64 / 2.0);

        let _ = buffer.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in draw_start..draw_end {
                let tex_y = tex_pos as u32 & (texture.height - 1);
                tex_pos += step;

                let (mut r, mut g, mut b, a) = texture.get_pixel(tex_x as usize, tex_y as usize).unwrap_or((255, 255, 255, 255));

                if x_side {
                    r /= 2;
                    g /= 2;
                    b /= 2;
                }

                let offset: usize = y as usize*pitch + x as usize*4;

                buffer[offset] = r;
                buffer[offset+1] = g;
                buffer[offset+2] = b;
                buffer[offset+3] = a;
            }
        });
    }
}

pub fn render_view_canvas(player: &PlayerData, canvas: &mut Canvas<Window>, game_data: &GameData, textured: bool) {
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

        let mut x_side = false;

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
                x_side = true;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                x_side = false;
            }

            if WORLD_MAP[map_x as usize][map_y as usize] != 0 {
                break;
            }
        }

        if x_side {
            perp_wall_dist = (side_dist_x - delta_dist_x);
        } else {
            perp_wall_dist = (side_dist_y - delta_dist_y);
        }

        let lineheight: i32 = ((game_data.height as f64)/perp_wall_dist) as i32;

        let mut draw_start: i32 = -lineheight / 2 + (game_data.height as i32) / 2;
        if draw_start < 0 && textured {
            draw_start = 0;
        }
        let mut draw_end: i32 = lineheight / 2 + (game_data.height as i32) / 2;
        if draw_end >= (game_data.height as i32) && textured {
            draw_end = (game_data.height as i32) - 1;
        }

        if !textured {
            let mut color = match WORLD_MAP[map_x as usize][map_y as usize] {
                1 => Color::RED,
                2 => Color::GREEN,
                3 => Color::BLUE,
                4 => Color::GRAY,
                _ => Color::CYAN
            };

            if x_side {
                let (r, g, b, a) = color.rgba();
                color = Color::RGBA(r/2, g/2, b/2, a/2);
            }

            ver_line(x, draw_start - player.ver, draw_end - player.ver, color, canvas);

            continue;
        }

        // Texture Handling
        let texture = match WORLD_MAP[map_x as usize][map_y as usize] {
            1 => game_data.textures.get(0).unwrap(),
            2 => game_data.textures.get(1).unwrap(),
            3 => game_data.textures.get(2).unwrap(),
            4 => game_data.textures.get(3).unwrap(),
            _ => game_data.textures.get(4).unwrap()
        };

        // The coordinate on the wall which corresponds to the x-coordinate on the texture.
        let mut wall_x: f64;
        if x_side {
            wall_x = pos_y + perp_wall_dist * ray_dir_y;
        } else {
            wall_x = pos_x + perp_wall_dist * ray_dir_x;
        }
        wall_x -= wall_x.floor();

        // The x-coordinate on the texture.
        let tex_x = (wall_x * texture.width as f64) as u32;

        let step = texture.height as f64 / lineheight as f64;
        let mut tex_pos = (draw_start as f64 - game_data.height as f64/2.0 + lineheight as f64 / 2.0);

        for y in draw_start..draw_end {
            let tex_y = tex_pos as u32 & (texture.height - 1);
            tex_pos += step;

            let (r, g, b, a) = texture.get_pixel(tex_x as usize, tex_y as usize).unwrap_or((255, 255, 255, 255));

            let color = match x_side {
                true => Color::RGBA(r/2, g/2, b/2, a),
                false => Color::RGBA(r, g, b, a)
            };

            canvas.set_draw_color(color);
            let _ = canvas.draw_point(Point::new(x, y));
        }
    }
}
