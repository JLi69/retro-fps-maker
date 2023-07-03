use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::{KeyboardState, Scancode};
use sdl2::mouse::MouseState;
use std::time::Instant;
use sdl2::video::Window;
use sdl2::render::Canvas;

mod level;
mod raycast;
use level::Level;
use raycast::{raycast, Vector2f64};

struct Camera {
	pub position: Vector2f64,
	pub rotation: f64,
	pub speed: f64,
	pub rotation_speed: f64,
	pub fov: f64
}

impl Camera {
	pub fn new(x: f64, y: f64, cam_rotation: f64, cam_fov: f64) -> Self {
		Camera {
			position: raycast::new_vector2(x, y),
			rotation: cam_rotation,
			fov: cam_fov,
			speed: 0.0,
			rotation_speed: 0.0
		}
	}

	pub fn update(&mut self, dt: f64) {
		self.position.x += self.speed * self.rotation.cos() * dt;
		self.position.y += self.speed * self.rotation.sin() * dt;
		self.rotation += self.rotation_speed * dt;
	}

	pub fn handle_key_input(&mut self, keyboard_state: &KeyboardState) {
		if keyboard_state.is_scancode_pressed(Scancode::Up) {
			self.speed = 2.0;	
		} else if keyboard_state.is_scancode_pressed(Scancode::Down) {	
			self.speed = -2.0;	
		} else {
			self.speed = 0.0;	
		}

		if keyboard_state.is_scancode_pressed(Scancode::Left) {
			self.rotation_speed = -1.0;	
		} else if keyboard_state.is_scancode_pressed(Scancode::Right) {	
			self.rotation_speed = 1.0;	
		} else {
			self.rotation_speed = 0.0;	
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameMode {
	Editor,
	Game
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ButtonState {
	Released,
	Held
}

fn invert_u8(val: u8) -> u8 {
	if val == 0 {
		return 1;	
	}
	
	0
}

fn display_level(
	canvas: &mut Canvas<Window>,
	camera: &mut Camera,
	level: &Level
) -> Result<(), String> {
	let mut angle = -camera.fov / 2.0 + camera.rotation;
	for i in 0..800 {
		let (hit, tile_type) = raycast(&camera.position, angle, 32.0, &level);
		let d = (hit.x - camera.position.x) * camera.rotation.cos() +
			(hit.y - camera.position.y) * camera.rotation.sin();
					
		if tile_type != 0 {
			canvas.set_draw_color(Color::GREEN);
			if hit.y.floor() == hit.y {		
				canvas.set_draw_color(Color::RGB(0, 200, 0));
			}

			canvas.draw_line(Point::new(i, (-300.0 / d + 300.0) as i32), 
										 Point::new(i, (300.0 / d + 300.0) as i32))
				.map_err(|e| e.to_string())?;	
		}

		angle += camera.fov / 800.0;
	}

	Ok(())
}

fn display_level_editor(
	canvas: &mut Canvas<Window>,
	level: &Level
) -> Result<(), String> {
	for y in 0..level.height {
		for x in 0..level.width {
			canvas.set_draw_color(Color::WHITE);
			canvas.draw_rect(Rect::new(x as i32 * 32, y as i32 * 32, 32, 32))
				.map_err(|e| e.to_string())?;
	
			if level.get_tile(x as isize, y as isize) != 0 {
				canvas.fill_rect(Rect::new(x as i32 * 32, y as i32 * 32, 32, 32))
					.map_err(|e| e.to_string())?;	
			}
		}
	}
	
	canvas.set_draw_color(Color::RED);
	canvas.fill_rect(Rect::new(level.spawnx as i32 * 32, level.spawny as i32 * 32, 32, 32))
		.map_err(|e| e.to_string())?;
	Ok(())
}

fn handle_mouse_input_editor(
	level: &mut Level,
	mouse_state: &MouseState,
	left_state: &ButtonState
) -> ButtonState {
	let mousex = (mouse_state.x() / 32) as isize;
	let mousey = (mouse_state.y() / 32) as isize;

	let new_left_state;

	if mouse_state.left() {
		if let ButtonState::Released = left_state {
			level.set_tile(mousex, mousey, invert_u8(level.get_tile(mousex, mousey)));	
		}
	
		new_left_state = ButtonState::Held;
	} else {
		new_left_state = ButtonState::Released;
	}
	
	if mouse_state.right() {
		level.spawnx = mousex as f64 + 0.5;
		level.spawny = mousey as f64 + 0.5;
	}

	new_left_state
}

fn can_quit(event_pump: &mut EventPump) -> bool {
	for event in event_pump.poll_iter() {
		match event {
			Event::Quit { .. } => { return true; }	
			_ => {}	
		}
	}

	false
}

fn switch_modes(
	game_mode: &GameMode, 
	p_key_state: &ButtonState,
	keyboard_state: &KeyboardState,
	camera: &mut Camera,
	level: &Level
) -> (ButtonState, GameMode) {
	if keyboard_state.is_scancode_pressed(Scancode::P) &&
	   *p_key_state == ButtonState::Released &&
	   *game_mode == GameMode::Editor { 
	   camera.position.x = level.spawnx;
	   camera.position.y = level.spawny;
	   camera.rotation = 0.0;
	   return (ButtonState::Held, GameMode::Game);
	} else if keyboard_state.is_scancode_pressed(Scancode::P) &&
	   *p_key_state == ButtonState::Released &&
	   *game_mode == GameMode::Game {
	   return (ButtonState::Held, GameMode::Editor);
	} else if keyboard_state.is_scancode_pressed(Scancode::P) {	
	   return (ButtonState::Held, game_mode.clone());
	}

	(ButtonState::Released, game_mode.clone())
}

fn main() -> Result<(), String> {
	let ctx = sdl2::init().map_err(|e| e.to_string())?;
	let vid_subsystem = ctx.video().map_err(|e| e.to_string())?;
	let window = vid_subsystem.window("Retro FPS", 800, 600)
		.position_centered()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas = window.into_canvas()
		.present_vsync()
		.build()
		.map_err(|e| e.to_string())?;
	let mut event_pump = ctx.event_pump().map_err(|e| e.to_string())?;

	let mut level = Level::new(16, 16);
	let mut camera = Camera::new(0.5, 0.5, 0.0, 3.14159 / 3.0);
	let mut left_state = ButtonState::Released;	
	let mut p_key_state = ButtonState::Released;
	let mut game_mode = GameMode::Editor;
	let mut dt = 0.0f64;

	while !can_quit(&mut event_pump) {
		let frame_start = Instant::now(); 

		let mouse_state = event_pump.mouse_state();
		let keyboard_state = event_pump.keyboard_state();

		canvas.set_draw_color(Color::BLACK);
		canvas.clear();
		
		match game_mode {
			GameMode::Editor => {
				display_level_editor(&mut canvas, &level)
					.map_err(|e| e.to_string())?;
				canvas.set_draw_color(Color::YELLOW);
				canvas.draw_rect(Rect::new(mouse_state.x() / 32 * 32, mouse_state.y() / 32 * 32, 32, 32))
					.map_err(|e| e.to_string())?;	
				left_state = handle_mouse_input_editor(&mut level, &mouse_state, &left_state);
			}	
			GameMode::Game => {
				display_level(&mut canvas, &mut camera, &level)
					.map_err(|e| e.to_string())?;	
				camera.update(dt);
				camera.handle_key_input(&keyboard_state);
			}
		}

		(p_key_state, game_mode) = switch_modes(&game_mode,
												&p_key_state,
												&keyboard_state,
												&mut camera,
												&level);

		canvas.present();	

		dt = frame_start.elapsed().as_secs_f64();
	}

	Ok(())
}
