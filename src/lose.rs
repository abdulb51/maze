use macroquad::prelude::*;

// use crate::ui::still_image::StillImage;
pub async fn run() -> String {
    
    
    
    
    
    loop {
        clear_background(BLUE);
        draw_text("Press SPACE to restart, or ESC to exit", 400.0, 400.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "game".to_string();
        }
        
        if is_key_pressed(KeyCode::Escape) {
            return "main".to_string();
        }

        next_frame().await;
    }
}