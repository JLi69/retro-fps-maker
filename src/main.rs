use sdl2::image::LoadTexture;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{BlendMode, Canvas, Texture};
use sdl2::video::Window;
use std::time::Instant;
use sdl2::rect::Rect;

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
use level_editor::{
    display_level_editor, 
    handle_mouse_input_editor, 
    level_editor_menu::LevelEditorMenu,
    level_editor_menu::load_default_assets,
    level_editor_menu::load_default_sprites,
    level_editor_menu::EditorMode,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameMode {
    Editor,
    Game,
}

fn switch_modes(
    game_mode: &GameMode,
    camera: &mut Camera,
    level: &Level,
) -> GameMode {
    if *game_mode == GameMode::Editor {
        camera.position.x = level.spawnx;
        camera.position.y = level.spawny;
        camera.rotation = 0.0;
        return GameMode::Game;
    } else if *game_mode == GameMode::Game {
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
    sprite_images: &[Texture],
    input_state: &InputState,
) -> Result<(), String> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    match game_mode {
        GameMode::Editor => {
            display_level_editor(canvas, level, input_state, &*textures, sprite_images)?;
        }
        GameMode::Game => {
            display_level(canvas, camera, level, textures, 2)?;
        }
    }

    for texture in textures {
        texture.set_color_mod(255, 255, 255);
    }

    Ok(())
}

fn update(
    game_mode: &GameMode,
    level: &mut Level,
    camera: &mut Camera,
    input_state: &InputState,
    selected_tile: u8,
    editor_mode: &EditorMode,
    dt: f64,
) {
    match game_mode {
        GameMode::Editor => {
            handle_mouse_input_editor(level, input_state, selected_tile, editor_mode);
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
        .window("Retro FPS", 960, 640)
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

    let mut level_editor_menu = LevelEditorMenu::new();
    let mut textures = load_default_assets(&texture_creator);
    let sprite_images = load_default_sprites(&texture_creator);
    let player_spawn_icon = texture_creator.load_texture("assets/images/player_spawn_icon.png")?;

    let mut level = Level::new(40, 40);
    let mut camera = Camera::new(0.5, 0.5, 0.0, std::f64::consts::PI / 12.0 * 5.0);
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
            &sprite_images,
            &input_state,
        )?;

        if game_mode == GameMode::Editor {
            match level_editor_menu.editor_mode {
                EditorMode::Tiles => {
                    level_editor_menu.display(
                        &mut canvas,
                        &input_state,
                        &texture_creator,
                        &font_8_bit_operator,
                        &textures,
                    )?;
                    level_editor_menu.handle_mouse_input(&input_state, textures.len() as u8);
                }
                EditorMode::Sprites => {
                    level_editor_menu.display(
                        &mut canvas,
                        &input_state,
                        &texture_creator,
                        &font_8_bit_operator,
                        &sprite_images,
                    )?;
                    level_editor_menu.handle_mouse_input(&input_state, sprite_images.len() as u8);
                }
            }

            canvas.copy(
                &player_spawn_icon, 
                None, 
                Rect::new(level.spawnx as i32 * 16, level.spawny as i32 * 16, 16, 16)
            )?;
        
            let clicked = 
                level_editor_menu.menu.get_clicked(&input_state, MouseButton::Left)
                    .unwrap_or("".to_owned());

            if clicked == "play_button" {
                game_mode = switch_modes(&game_mode, &mut camera, &level);
            } else if clicked == "save_button" {
                level_editor::level_file::write_level_file(&level, "saved_level")?; 
            } else if clicked == "load_button" {
                level = level_editor::level_file::read_level_file("saved_level")?; 
            } else if clicked == "sprite_button" {
                level_editor_menu.selected = 1;
                level_editor_menu.editor_mode = EditorMode::Sprites;
            } else if clicked == "tile_button" {   
                level_editor_menu.selected = 1;
                level_editor_menu.editor_mode = EditorMode::Tiles;
            }
        }

        canvas.present();

        update(
            &game_mode,
            &mut level,
            &mut camera,
            &input_state,
            level_editor_menu.selected,
            &level_editor_menu.editor_mode,
            dt,
        );
        
        game_mode = if input_state.key_is_clicked(Scancode::P) {
            switch_modes(&game_mode, &mut camera, &level) 
        } else {
            game_mode 
        };

        input_state.update(&event_pump);

        //Calculate how much time has elapsed in the frame
        dt = frame_start.elapsed().as_secs_f64();
    }

    Ok(())
}
