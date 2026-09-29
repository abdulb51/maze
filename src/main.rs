/*
By: Abdul, the G.O.A.T, Baig
Date: 2026-09-22
Program Details: <maze game>
*/

mod ui;
mod utils;

mod game;
mod win;
mod start;
mod lose;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "mazegame".to_owned(),
        window_width: 1080,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "start".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "game" => game::run().await,
                "win" => win::run().await,
                "start" => start::run().await,
                "lose" => lose::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
