use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{keyboard::Scancode, render::Canvas};
use std::time::Instant;

mod camera;
mod events;
mod game;
mod level;
mod level_editor;
mod raycast;
mod sprite;
use camera::Camera;
use events::{can_quit, InputState};
use game::display_level;
use level::Level;
use level_editor::{display_level_editor, handle_mouse_input_editor};

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameMode {
    Editor,
    Game,
}

fn switch_modes(
    game_mode: &GameMode,
    input_state: &InputState,
    camera: &mut Camera,
    level: &Level,
) -> GameMode {
    if input_state.key_is_clicked(Scancode::P) && *game_mode == GameMode::Editor {
        camera.position.x = level.spawnx;
        camera.position.y = level.spawny;
        camera.rotation = 0.0;
        return GameMode::Game;
    } else if input_state.key_is_clicked(Scancode::P) && *game_mode == GameMode::Game {
        return GameMode::Editor;
    }

    game_mode.clone()
}

fn display(
    canvas: &mut Canvas<Window>,
    game_mode: &GameMode,
    camera: &Camera,
    level: &Level,
    input_state: &InputState,
) -> Result<(), String> {
    match game_mode {
        GameMode::Editor => {
            display_level_editor(canvas, &level, &input_state).map_err(|e| e.to_string())?;
        }
        GameMode::Game => {
            display_level(canvas, &camera, &level).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn update(
    game_mode: &GameMode,
    level: &mut Level,
    camera: &mut Camera,
    input_state: &InputState,
    dt: f64,
) {
    match game_mode {
        GameMode::Editor => {
            handle_mouse_input_editor(level, &input_state);
        }
        GameMode::Game => {
            camera.update(dt);
            camera.handle_key_input(&input_state);
        }
    }
}

fn main() -> Result<(), String> {
    let ctx = sdl2::init().map_err(|e| e.to_string())?;
    let vid_subsystem = ctx.video().map_err(|e| e.to_string())?;
    let window = vid_subsystem
        .window("Retro FPS", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = ctx.event_pump().map_err(|e| e.to_string())?;

    let mut level = Level::new(16, 16);
    let mut camera = Camera::new(0.5, 0.5, 0.0, 3.14159 / 3.0);
    let mut input_state = InputState::new();
    let mut game_mode = GameMode::Editor;
    let mut dt = 0.0f64;

    //Main loop
    while !can_quit(&mut event_pump) {
        let frame_start = Instant::now();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        display(&mut canvas, &game_mode, &camera, &level, &input_state)
            .map_err(|e| e.to_string())?;
        update(&game_mode, &mut level, &mut camera, &input_state, dt);

        game_mode = switch_modes(&game_mode, &input_state, &mut camera, &level);

        canvas.present();
        input_state.update(&event_pump);
        dt = frame_start.elapsed().as_secs_f64();
    }

    Ok(())
}
