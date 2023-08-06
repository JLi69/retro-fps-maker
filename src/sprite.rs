use crate::Camera;
use sdl2::{
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
};

pub struct Vector2f64 {
    pub x: f64,
    pub y: f64,
}

pub fn new_vector2(px: f64, py: f64) -> Vector2f64 {
    Vector2f64 { x: px, y: py }
}

pub fn dist(v1: &Vector2f64, v2: &Vector2f64) -> f64 {
    ((v1.x - v2.x).powi(2) + (v2.y - v1.y).powi(2)).sqrt()
}

pub struct Sprite {
    pub pos: Vector2f64,
    pub sprite_type: u8,
}

impl Sprite {
    pub fn new(x: f64, y: f64, spr_type: u8) -> Self {
        Self {
            pos: new_vector2(x, y),
            sprite_type: spr_type,
        }
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        depth_buffer: &[f64],
        cam: &Camera,
        sprite_image: &Texture,
    ) -> Result<(), String> {
        let sprite_trans_x = self.pos.x - cam.position.x;
        let sprite_trans_y = self.pos.y - cam.position.y;
        let sprite_rotated_y =
            sprite_trans_x * (-cam.rotation).cos() - sprite_trans_y * (-cam.rotation).sin();
        let sprite_rotated_x =
            sprite_trans_x * (-cam.rotation).sin() + sprite_trans_y * (-cam.rotation).cos();

        if sprite_rotated_y < 0.4 {
            return Ok(());
        }

        let sprite_sz = 400.0;

        let sprite_screen_size = (sprite_sz / sprite_rotated_y) as u32;
        let sprite_screen_y =
            (320.0 / sprite_rotated_y + 320.0 - sprite_screen_size as f64 / 2.0) as i32;
        let norm_x = (sprite_rotated_x / sprite_rotated_y).atan() / cam.fov + 0.5;

        let sprite_screen_x = (norm_x * 800.0) as i32;

        let fov_range = 2.0 * (cam.fov / 2.0).tan() * sprite_rotated_y;
        let sprite_start_x = ((sprite_rotated_x - sprite_sz / 640.0) / fov_range) + 0.5;
        let sprite_end_x = ((sprite_rotated_x + sprite_sz / 640.0) / fov_range) + 0.5;

        if ((sprite_start_x < 1.0 && sprite_end_x > 0.0)
            || (sprite_end_x > 1.0 && sprite_start_x < 0.0))
            && sprite_rotated_y > 0.0
        {
            let startx = (sprite_screen_x - sprite_screen_size as i32 / 2) / 2;
            let endx = (sprite_screen_x + sprite_screen_size as i32 / 2) / 2;
            let mut pixel_x = 0.0f64;
            for i in startx..endx {
                if i >= 0
                    && (i as usize) < depth_buffer.len()
                    && depth_buffer[i as usize] > sprite_rotated_y
                {
                    canvas.copy(
                        &sprite_image,
                        Rect::new(pixel_x as i32, 0, 1, 16),
                        Rect::from_center(
                            Point::new(i * 2 + 1 + 80, sprite_screen_y),
                            2,
                            sprite_screen_size,
                        ),
                    )?;
                }
                pixel_x += 16.0 / sprite_screen_size as f64 * 2.0;
            }
        }

        Ok(())
    }
}
