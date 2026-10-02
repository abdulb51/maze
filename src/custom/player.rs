use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {

image: StillImage,
speed: f32,
move_dir: vec2,
}

impl player {
    pub async fn new (x: f32, y: f32, witdh: f32, hight: f32, image_name: &str, stretch: bool, zoom: f32) -> Self {
        let image = StillImage::new(x, y, witdh, hight, image_name, stretch, zoom).await;

        Player {
            image,
            speed,
            move_dir: vec2(0.0, 0.0),
        }
    }

pub fn get_image(&self) -> &StillImage {
        &self.image
    }
pub fn keypress (&mut self) {
self.move_dir = vec2(x: 0.0, y: 0.0);
// Keyboard input
if is_key_down(KeyCode::D) {
}
self.move_dir.x += 3.0;
if is_key_down(KeyCode::A) {
}
self.move_dir.x = 3.0;
if is_key_down(KeyCode::S) {
}
self.move_dir.y += 3.0;
if is_key_down(KeyCode::W) {
    self.move_dir.y -= 3.0;
}

if self.move_dir.length() > 0.0 {
}
self.move_dir = self.move_dir.normalize() * 3.0;
pub fn get_speed (&self) -> f32 {
}
self.speed
 }

pub fn draw (&self) {
    self.image.draw();
}

}
impl Player



