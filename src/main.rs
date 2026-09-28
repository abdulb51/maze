/*
By: Abdul, the G.O.A.T, Baig
Date: 2026-09-22
Program Details: <Program Description Here>
*/

mod ui;
mod utils;

use crate::ui::grid::draw_grid;
use crate::ui::still_image::StillImage;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::TextureManager;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "maze".to_string(),
        window_width: 1080,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut keys = 0;
    let mut draw_key1 = true;
    let mut draw_key2 = true;
    let mut draw_lock1 = true;
    let mut draw_lock2 = true;

    let tm = TextureManager::new();
    let all_assets = vec![
        "assets/maze.png",
        "assets/subaru.png",
        "assets/subaruflip.png",
        "assets/spike.png",
        "assets/spikeball.png",
        "assets/trophy.png",
        "assets/lock2.png",
        "assets/lock1.png",
        "assets/key1.png",
        "assets/key2.png",
    ];
    tm.preload_with_loading_screen(&all_assets, None, None).await;

    let img_maze = StillImage::from_preload(
        tm.get_preload("assets/maze.png").unwrap(),
        1080.0, // width
        1080.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    );

    let mut img_subaru = StillImage::from_preload(tm.get_preload("assets/subaru.png").unwrap(), 150.0, 150.0, 55.0, 405.0, true, 1.0);

    let mut img_key1 = StillImage::from_preload(tm.get_preload("assets/key1.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_key2 = StillImage::from_preload(tm.get_preload("assets/key2.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_lock1 = StillImage::from_preload(tm.get_preload("assets/lock1.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_lock2 = StillImage::from_preload(tm.get_preload("assets/lock2.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_spikeball = StillImage::from_preload(tm.get_preload("assets/spikeball.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_spike = StillImage::from_preload(tm.get_preload("assets/spike.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_trophy = StillImage::from_preload(tm.get_preload("assets/trophy.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    const MOVE_SPEED: f32 = 220.0;

    let mut spikeball_x = 0.0;
    let mut spikeball_y = 0.0;

    let mut spike_x = 0.0;
    let mut spike_y = 0.0;

    loop {
        clear_background(WHITE);

        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
            img_subaru.set_preload(tm.get_preload("assets/subaru.png").unwrap());
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
            img_subaru.set_preload(tm.get_preload("assets/subaruflip.png").unwrap());
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = img_subaru.pos();

        // Move X first
        if movement.x != 0.0 {
            img_subaru.set_x(img_subaru.get_x() + movement.x);
            if check_collision(&img_subaru, &img_maze, 1) {
                img_subaru.set_x(old_pos.x); // Undo if collision happens
            }
        }

        // Move Y next
        if movement.y != 0.0 {
            img_subaru.set_y(img_subaru.get_y() + movement.y);
            if check_collision(&img_subaru, &img_maze, 1) {
                img_subaru.set_y(old_pos.y); // Undo if collision happens
            }
        }

        if check_collision(&img_subaru, &img_key1, 1) {
            keys = 1;
        }

        if check_collision(&img_subaru, &img_key2, 1) {
            keys = 2;
        }
println! ("{}", keys);
        if keys >= 2 {
            draw_key1 = false;
            draw_key2 = false;
        } 
        
        else if keys == 1 {
            draw_key1 = true;
            draw_key2 = false;
        } 
        
        else {
            draw_key1 = true;
            draw_key2 = true;
        }

        if check_collision(&img_subaru, &img_lock1, 1) && keys >= 1 {
            img_lock1.clear();
        }

        if check_collision(&img_subaru, &img_lock1, 1) {
            img_subaru.set_x(old_pos.x); // Undo if collision happens
        }

        if check_collision(&img_subaru, &img_lock1, 1) {
            img_subaru.set_y(old_pos.y); // Undo if collision happens
        }

        if check_collision(&img_subaru, &img_lock2, 1) && keys >= 2 {
            img_lock2.clear();
        }

        if check_collision(&img_subaru, &img_lock2, 1) {
            img_subaru.set_x(old_pos.x); // Undo if collision happens
        }

        if check_collision(&img_subaru, &img_lock2, 1) {
            img_subaru.set_y(old_pos.y); // Undo if collision happens
        }

        let mut spikeball_pos = img_spikeball.pos();

        if spikeball_pos.x == 0.0 && spikeball_pos.y == 0.0 {
            spikeball_x = 0.0;
            spikeball_y = 1.0;
        }

        if spikeball_pos.x == 0.0 && spikeball_pos.y == 250.0 {
            spikeball_x = 0.0;
            spikeball_y = -1.0;
        }
        spikeball_pos.y += spikeball_y;
        spikeball_pos.x += spikeball_x;
        img_spikeball.set_position(spikeball_pos);

        let mut spike_pos = img_spike.pos();

        if spike_pos.x == 0.0 && spike_pos.y == 0.0 {
            spike_x = 0.0;
            spike_y = 2.0;
        }

        if spike_pos.x == 0.0 && spike_pos.y == 400.0 {
            spike_x = 0.0;
            spike_y = -2.0;
        }
        spike_pos.y += spike_y;
        spike_pos.x += spike_x;
        img_spike.set_position(spike_pos);

        //come back to this
        // if check_collision(&img_subaru, &img_spike, 1) {
        //     img_subaru.set_x(55.0); // KILL if collision happens
        // }

        // if check_collision(&img_subaru, &img_spike, 1) {
        //     img_subaru.set_y(405.0); // KILL if collision happens
        // }

        // //come back to this
        // if check_collision(&img_subaru, &img_spikeball, 1) {

        //     img_subaru.set_x( img_subaru.get_x() - 30.0); // KILL if collision happens
        // }

        if check_collision(&img_subaru, &img_trophy, 1) {
            img_subaru.set_x(55.0);
            img_subaru.set_y(405.0);

        keys = 0;
        }

        if draw_key1 {
            img_key1.draw();
        }
        if draw_key2 {
            img_key2.draw();
        }
        img_trophy.draw();
        img_spikeball.draw();
        img_spike.draw();
        img_maze.draw();
        img_lock1.draw();
        img_lock2.draw();
        img_subaru.draw();

        draw_grid(50.0, RED);
        next_frame().await;
    }
}
