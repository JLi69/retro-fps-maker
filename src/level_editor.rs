use crate::InputState;
use crate::Level;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn display_level_editor(
    canvas: &mut Canvas<Window>,
    level: &Level,
    input_state: &InputState,
) -> Result<(), String> {
    for y in 0..level.height {
        for x in 0..level.width {
            canvas.set_draw_color(Color::WHITE);
            canvas.draw_rect(Rect::new(x as i32 * 32, y as i32 * 32, 32, 32))?;

            if level.get_tile(x as isize, y as isize) != 0 {
                canvas.set_draw_color(Color::GREEN);
                canvas.fill_rect(Rect::new(x as i32 * 32, y as i32 * 32, 32, 32))?
            }
        }
    }

    canvas.set_draw_color(Color::RED);
    canvas.fill_rect(Rect::new(
        level.spawnx as i32 * 32,
        level.spawny as i32 * 32,
        32,
        32,
    ))?;

    let (mousex, mousey) = input_state.mouse_pos();
    canvas.set_draw_color(Color::YELLOW);
    canvas.draw_rect(Rect::new(mousex / 32 * 32, mousey / 32 * 32, 32, 32))?;

    Ok(())
}

pub fn handle_mouse_input_editor(level: &mut Level, input_state: &InputState) {
    let (mousex, mousey) = input_state.mouse_pos();
    let (mousex, mousey) = (mousex as isize / 32, mousey as isize / 32);

    if input_state.mouse_button_is_clicked(MouseButton::Left) {
        level.set_tile(mousex, mousey, level.get_tile(mousex, mousey) ^ 1);
    }

    if input_state.mouse_button_is_clicked(MouseButton::Right) {
        level.spawnx = mousex as f64 + 0.5;
        level.spawny = mousey as f64 + 0.5;
    }
}
