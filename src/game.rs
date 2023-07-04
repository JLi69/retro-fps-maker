use crate::raycast::raycast;
use crate::Camera;
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
        let (hit, tile_type) = raycast(&camera.position, angle, 32.0, &level);
        let d = (hit.x - camera.position.x) * camera.rotation.cos()
            + (hit.y - camera.position.y) * camera.rotation.sin();

        canvas.set_draw_color(Color::GREEN);
        if hit.y.floor() == hit.y {
            canvas.set_draw_color(Color::RGB(0, 200, 0));
        }

        if tile_type != 0 {
            canvas
                .draw_line(
                    Point::new(i, (-300.0 / d + 300.0) as i32),
                    Point::new(i, (300.0 / d + 300.0) as i32),
                )
                .map_err(|e| e.to_string())?;
        }

        angle += camera.fov / 800.0;
    }

    Ok(())
}