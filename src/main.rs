use sdl2::image::LoadTexture;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas, Texture};
use sdl2::video::Window;
use std::time::Instant;

mod camera;
mod events;
mod game;
mod level;
mod level_editor;
mod menu;
mod raycast;
mod sprite;
use camera::Camera;
use events::{can_quit, InputState};
use game::{display_level, game_update};
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

    *game_mode
}

fn display(
    canvas: &mut Canvas<Window>,
    game_mode: &GameMode,
    camera: &Camera,
    level: &Level,
    textures: &mut [Texture],
    input_state: &InputState,
) -> Result<(), String> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    match game_mode {
        GameMode::Editor => {
            display_level_editor(canvas, level, input_state)?;
        }
        GameMode::Game => {
            display_level(canvas, camera, level, textures, 4)?;
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
            handle_mouse_input_editor(level, input_state);
        }
        GameMode::Game => {
            game_update(level, camera, input_state, dt);
        }
    }
}

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let vid_subsystem = ctx.video()?;
    let window = vid_subsystem
        .window("Retro FPS", 800, 640)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);
    let mut event_pump = ctx.event_pump()?;

    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // Load textures
    let mut textures = vec![texture_creator.load_texture("assets/images/test-texture.png")?];

    let mut level = Level::new(25, 20);
    let mut camera = Camera::new(0.5, 0.5, 0.0, std::f64::consts::PI / 3.0);
    let mut input_state = InputState::new();
    let mut game_mode = GameMode::Editor;
    let mut dt = 0.0f64;

    let font_8_bit_operator =
        ttf_ctx.load_font("assets/fonts/8BitOperator/8bitOperatorPlus-Regular.ttf", 64)?;

    //Main loop
    while !can_quit(&mut event_pump) {
        let frame_start = Instant::now();

        display(
            &mut canvas,
            &game_mode,
            &camera,
            &level,
            &mut textures,
            &input_state,
        )?;
        canvas.present();

        update(&game_mode, &mut level, &mut camera, &input_state, dt);
        game_mode = switch_modes(&game_mode, &input_state, &mut camera, &level);
        input_state.update(&event_pump);

        //Calculate how much time has elapsed in the frame
        dt = frame_start.elapsed().as_secs_f64();
    }

    Ok(())
}
