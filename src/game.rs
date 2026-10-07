use macroquad::prelude::*;

use crate::custom::player::Player;
use crate::ui::still_image::StillImage;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::TextureManager;

pub async fn run() -> String {
    let mut keys = 0;

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

    let mut player = Player::new("assets/subaru.png", 55.0, 405.0, 150.0, 150.0, true, 1.0).await;

    let mut img_key1 = StillImage::from_preload(tm.get_preload("assets/key1.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_key2 = StillImage::from_preload(tm.get_preload("assets/key2.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_lock1 = StillImage::from_preload(tm.get_preload("assets/lock1.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_lock2 = StillImage::from_preload(tm.get_preload("assets/lock2.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_spikeball = StillImage::from_preload(tm.get_preload("assets/spikeball.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut img_spike = StillImage::from_preload(tm.get_preload("assets/spike.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let img_trophy = StillImage::from_preload(tm.get_preload("assets/trophy.png").unwrap(), 1080.0, 1080.0, 0.0, 0.0, true, 1.0);

    let mut spikeball_x = 0.0;
    let mut spikeball_y = 0.0;

    let mut spike_x = 0.0;
    let mut spike_y = 0.0;

    loop {
        clear_background(WHITE);

        player.keypress();
        player.move_player();

        if player.move_dir.x != 0.0 {
            if check_collision(player.get_image(), &img_maze, 1) {
                player.move_back_x();
            }
        }
        if player.move_dir.y != 0.0 {
            if check_collision(player.get_image(), &img_maze, 1) {
                player.move_back_y();
            }
        }

        if check_collision(player.get_image(), &img_key1, 1) {
            keys = 1;
        }

        if check_collision(player.get_image(), &img_key2, 1) {
            keys = 2;
        }

        if keys >= 2 {
            img_key1.clear();
            img_key2.clear();
        } else if keys == 1 {
            img_key1.clear();
            img_key2.set_preload(tm.get_preload("assets/key2.png").unwrap());
        } else {
            img_key1.set_preload(tm.get_preload("assets/key1.png").unwrap());
            img_key2.set_preload(tm.get_preload("assets/key2.png").unwrap());
        }

        if check_collision(player.get_image(), &img_lock1, 1) {
            if keys == 1 {
                img_lock1.clear(); // unlocked
            } else {
                player.move_back_x();
            } // Undo if collision happens
        };

        if check_collision(player.get_image(), &img_lock2, 1) {
            if keys == 2 {
                img_lock2.clear(); // unlocked
            } else {
                player.move_back_y();
            } // Undo if collision happens
        };

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

        if check_collision(player.get_image(), &img_spike, 1) {
            return "lose".to_string();
        }

        if check_collision(player.get_image(), &img_spikeball, 1) {
            return "lose".to_string(); //sends to loss screen 
        }

        if check_collision(player.get_image(), &img_trophy, 1) {
            return "win".to_string();
        }

        img_key1.draw();
        img_key2.draw();
        img_lock1.draw();
        img_lock2.draw();
        img_trophy.draw();
        img_spikeball.draw();
        img_spike.draw();
        img_maze.draw();
        img_lock1.draw();
        img_lock2.draw();
        player.draw();
        next_frame().await;
    }
}
