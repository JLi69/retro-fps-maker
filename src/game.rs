use crate::raycast::raycast;
use crate::Camera;
use crate::InputState;
use crate::Level;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub fn display_level(
    canvas: &mut Canvas<Window>,
    camera: &Camera,
    level: &Level,
    textures: &mut [Texture],
    line_width: u32,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.fill_rect(Rect::new(80, 0, 800, 320))?;
    canvas.set_draw_color(Color::RGB(64, 64, 64));
    canvas.fill_rect(Rect::new(80, 320, 800, 320))?;

    let mut angle = -camera.fov / 2.0 + camera.rotation;
    for i in 0..(800 / line_width) {
        let (hit, tile_type) = raycast(&camera.position, angle, 128.0, level);
        let d = (hit.x - camera.position.x) * camera.rotation.cos()
            + (hit.y - camera.position.y) * camera.rotation.sin();

        if tile_type > 0 && (tile_type as usize) <= textures.len() && hit.x == hit.x.floor() {
            let texture_properties = textures[tile_type as usize - 1].query();

            textures[tile_type as usize - 1].set_color_mod(255, 255, 255);

            let sample_rect = Rect::new(
                (hit.y.fract() * texture_properties.width as f64) as i32,
                0,
                1,
                texture_properties.height,
            );

            let dst_rect = Rect::from_center(
                Point::new((i * line_width) as i32 + line_width as i32 / 2 + 80, 320),
                line_width,
                (600.0 / d) as u32,
            );

            canvas.copy(&textures[tile_type as usize - 1], sample_rect, dst_rect)?;
        } else if tile_type > 0 && (tile_type as usize) <= textures.len() && hit.y == hit.y.floor()
        {
            let texture_properties = textures[tile_type as usize - 1].query();

            textures[tile_type as usize - 1].set_color_mod(180, 180, 180);

            let sample_rect = Rect::new(
                (hit.x.fract() * texture_properties.width as f64) as i32,
                0,
                1,
                texture_properties.height,
            );

            let dst_rect = Rect::from_center(
                Point::new((i * line_width) as i32 + line_width as i32 / 2 + 80, 320),
                line_width,
                (600.0 / d) as u32,
            );

            canvas.copy(&textures[tile_type as usize - 1], sample_rect, dst_rect)?;
        }

        angle += camera.fov / 800.0 * line_width as f64;
    }

    Ok(())
}

pub fn game_update(level: &mut Level, camera: &mut Camera, input_state: &InputState, dt: f64) {
    //Handle player collision with any tiles
    let dist_travelled = camera.speed.abs() * dt;
    //Cast the ray in the direction the player is moving
    let ray_angle = if camera.speed > 0.0 {
        camera.rotation //Player is moving forward
    } else {
        camera.rotation + std::f64::consts::PI //Player is moving backwards
    };

    let (hit_pos, tile) = raycast(&camera.position, ray_angle, dist_travelled, level);
    if tile == 0 {
        //If the player doesn't hit any tile, just move it as normal
        camera.update(dt);
    } else {
        //Otherwise, only move it very close to the tile they hit
        camera.position.x = hit_pos.x - ray_angle.cos() * 0.01;
        camera.position.y = hit_pos.y - ray_angle.sin() * 0.01;
    }

    //Get key input to move the camera
    camera.handle_key_input(input_state);
}
