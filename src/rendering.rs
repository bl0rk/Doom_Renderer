use crate::{Vec2, GAME_MAP};

pub fn cast_ray3(pos_vec: &Vec2, dir_vec: &Vec2) -> (f32, i32) {
    let mut map_x: i32 = pos_vec.x.floor() as i32;
    let mut map_y: i32 = pos_vec.y.floor() as i32;

    let step_x: i32 = if dir_vec.x > 0.0 {1} else {-1};
    let step_y: i32 = if dir_vec.y > 0.0 {1} else {-1};

    // Get the length of the vector that collides with the next x or y axis.
    // Note: These make NaN if the vector lies directly on an axis.
    let mut side_dist_x: f32 = {
        // Get scalar to multiply the direction vector by.
        let c = (if dir_vec.x > 0.0 {pos_vec.x.ceil()/dir_vec.x} else {pos_vec.x.floor()/dir_vec.x});

        // Calculate the length of the vector.
        ((dir_vec.x*c).powi(2) + (dir_vec.y*c).powi(2)).sqrt()
    };

    let mut side_dist_y: f32 = {
        let c = (if dir_vec.y > 0.0 {pos_vec.y.ceil()/dir_vec.y} else {pos_vec.y.floor()/dir_vec.y});

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
        if GAME_MAP[map_x as usize][map_y as usize] != 0 {
            return (perp_wall_dist, GAME_MAP[map_x as usize][map_y as usize]);
            // return if hit_x {(side_dist_x, hit_x)} else {(side_dist_y, hit_x)};
        }
    }
}

pub fn cast_ray4(pos_vec: &Vec2, dir_vec: &Vec2, plane_vec: &Vec2, camera_x: f32) -> (f32, bool) {
    let mut ray_dir_x = dir_vec.x + plane_vec.x * camera_x;
    let mut ray_dir_y = dir_vec.x + plane_vec.x * camera_x;

    let mut map_x = pos_vec.x.floor() as i32;
    let mut map_y = pos_vec.y.floor() as i32;

    let mut side_dist_x: f32 = 0.0;
    let mut side_dist_y: f32 = 0.0;

    let delta_dist_x = if ray_dir_x == 0.0 { f32::MAX } else { (1.0/ray_dir_x).abs() };
    let delta_dist_y = if ray_dir_y == 0.0 { f32::MAX } else { (1.0/ray_dir_y).abs() };

    let mut perp_wall_dist: f32 = 0.0;

    let mut step_x: i32 = 0;
    let mut step_y: i32 = 0;

    let mut hit = 0;

    let mut side = 0;

    if ray_dir_x < 0.0 {
        step_x = -1;
        side_dist_x = (pos_vec.x - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - pos_vec.x) * delta_dist_x;
    }

    if ray_dir_y < 0.0 {
        step_y = -1;
        side_dist_y = (pos_vec.y - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - pos_vec.y) * delta_dist_y;
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

        if GAME_MAP[map_x as usize][map_y as usize] > 0 {
            hit = 1;
        }
    }

    if side == 0 {
        perp_wall_dist = side_dist_x - delta_dist_x;
    } else {
        perp_wall_dist = side_dist_y - delta_dist_y;
    }

    return (perp_wall_dist, if side == 0 {true} else {false});
}
