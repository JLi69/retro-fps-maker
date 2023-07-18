use crate::raycast::raycast;
use crate::Camera;
use crate::InputState;
use crate::Level;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn display_level(
    canvas: &mut Canvas<Window>,
    camera: &Camera,
    level: &Level,
) -> Result<(), String> {
    let mut angle = -camera.fov / 2.0 + camera.rotation;
    for i in 0..800 {
        let (hit, tile_type) = raycast(&camera.position, angle, 32.0, level);
        let d = (hit.x - camera.position.x) * camera.rotation.cos()
            + (hit.y - camera.position.y) * camera.rotation.sin();

        canvas.set_draw_color(Color::GREEN);
        if hit.y.floor() == hit.y {
            canvas.set_draw_color(Color::RGB(0, 200, 0));
        }

        if tile_type != 0 {
            canvas.draw_line(
                Point::new(i, (-300.0 / d + 300.0) as i32),
                Point::new(i, (300.0 / d + 300.0) as i32),
            )?;
        }

        angle += camera.fov / 800.0;
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
