use crate::InputState;
use crate::Level;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub mod level_editor_menu;
pub mod level_file;

pub fn display_level_editor(
    canvas: &mut Canvas<Window>,
    level: &Level,
    input_state: &InputState,
    textures: &[Texture],
) -> Result<(), String> {
    for y in 0..level.height {
        for x in 0..level.width {
            canvas.set_draw_color(Color::WHITE);
            canvas.draw_rect(Rect::new(x as i32 * 16, y as i32 * 16, 16, 16))?;

            if level.get_tile(x as isize, y as isize) != 0 {
                canvas.copy(
                    &textures[level.get_tile(x as isize, y as isize) as usize - 1],
                    None,
                    Rect::new(x as i32 * 16, y as i32 * 16, 16, 16),
                )?;
            }
        }
    }

    canvas.set_draw_color(Color::RED);
    canvas.fill_rect(Rect::new(
        level.spawnx as i32 * 16,
        level.spawny as i32 * 16,
        16,
        16,
    ))?;

    let (mousex, mousey) = input_state.mouse_pos();
    canvas.set_draw_color(Color::YELLOW);
    canvas.draw_rect(Rect::new(mousex / 16 * 16, mousey / 16 * 16, 16, 16))?;

    Ok(())
}

fn invert_tile(current: u8, selected: u8) -> u8 {
    if current == 0 {
        selected
    } else {
        0
    }
}

pub fn handle_mouse_input_editor(level: &mut Level, input_state: &InputState, selected: u8) {
    let (mousex, mousey) = input_state.mouse_pos();
    let (mousex, mousey) = (mousex as isize / 16, mousey as isize / 16);

    if input_state.mouse_button_is_clicked(MouseButton::Left) {
        level.set_tile(
            mousex,
            mousey,
            invert_tile(level.get_tile(mousex, mousey), selected),
        );
    }

    if input_state.mouse_button_is_clicked(MouseButton::Right) {
        level.spawnx = mousex as f64 + 0.5;
        level.spawny = mousey as f64 + 0.5;
    }
}
