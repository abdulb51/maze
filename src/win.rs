use macroquad::prelude::*;



pub async fn run() -> String {
   
   
    loop {
        clear_background(BLUE);
        draw_text("You Win!
        
        Press SPACE to start again.
        Press ESC to End
        ", 500.0, 500.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "start".to_string();
        }
if is_key_pressed(KeyCode::Escape) {
            return "main".to_string();
        }

        next_frame().await;
    }
}