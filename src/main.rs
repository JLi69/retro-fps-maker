use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use std::time::Instant;

enum Mode {
	Editor,
	Game
}

enum ButtonState {
	Released,
	Held
}

struct Level {
	width: usize,
	height: usize,
	level_data: Vec<u8>
}

impl Level {
	fn new(w: usize, h: usize) -> Self {
		Level {
			width: w,
			height: h,
			level_data: vec![0u8; w * h]
		}
	}

	fn out_of_bounds(&self, x: isize, y: isize) -> bool {
		x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize	
	}

	fn get_tile(&self, x: isize, y: isize) -> u8 {
		if self.out_of_bounds(x, y) {
			return 0;
		}

		self.level_data[self.width * y as usize + x as usize]
	}

	fn set_tile(&mut self, x: isize, y: isize, tile: u8) {
		if self.out_of_bounds(x, y) {
			return;	
		}

		self.level_data[self.width * y as usize + x as usize] = tile;	
	}
}

struct Raycast {
	//Coordinates the ray hit
    x: f64,
    y: f64,
	//Tile that was hit
    tile_type: u8,
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn raycast(startx: f64,
		   starty: f64,
		   angle: f64,
		   max_dist: f64,
		   level: &Level) -> Raycast {
    let mut vert = Raycast {
        x: 0.0,
        y: 0.0,
        tile_type: 0,
    };

    //Check vertical lines
    if angle.cos() > 0.0 {
        let mut rayx = startx.ceil();
        let mut rayy = (rayx - startx) * angle.tan() + starty;
        while (startx - rayx).abs() < max_dist {
            let xind = rayx as isize;
            let yind = rayy.floor() as isize;

            if !level.out_of_bounds(xind, yind) && 
				level.get_tile(xind, yind) != 0
            {
                vert = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: level.get_tile(xind, yind),
                };

                break;
            }

            rayx += 1.0;
            rayy += angle.tan();
        }
    } else if angle.cos() < 0.0 {
        let mut rayx = startx.floor();
        let mut rayy = (rayx - startx) * angle.tan() + starty;
        while (startx - rayx).abs() < max_dist {
            let xind = rayx as isize - 1;
            let yind = rayy.floor() as isize;

            if !level.out_of_bounds(xind, yind) &&
				level.get_tile(xind, yind) != 0
            {
                vert = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: level.get_tile(xind, yind),
                };
                break;
            }

            rayx -= 1.0;
            rayy -= angle.tan();
        }
    }

    let mut horiz = Raycast {
        x: 0.0,
        y: 0.0,
        tile_type: 0,
    };

    //Check horizontal lines
    if angle.sin() > 0.0 {
        let mut rayy = starty.ceil();
        let mut rayx = (rayy - starty) * 1.0 / angle.tan() + startx;
        while (starty - rayy).abs() < max_dist {
            let xind = rayx.floor() as isize;
            let yind = rayy as isize;

            if !level.out_of_bounds(xind, yind) &&
				level.get_tile(xind, yind) != 0
            {
                horiz = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: level.get_tile(xind, yind),
                };
                break;
            }

            rayy += 1.0;
            rayx += 1.0 / angle.tan();
        }
    } else if angle.sin() < 0.0 {
        let mut rayy = starty.floor();
        let mut rayx = (rayy - starty) * 1.0 / angle.tan() + startx;
        while (starty - rayy).abs() < max_dist {
            let xind = rayx.floor() as isize;
            let yind = rayy as isize - 1;

            if !level.out_of_bounds(xind, yind) &&
				level.get_tile(xind, yind) != 0
            {
                horiz = Raycast {
                    x: rayx,
                    y: rayy,
                    tile_type: level.get_tile(xind, yind),
                };
                break;
            }

            rayy -= 1.0;
            rayx -= 1.0 / angle.tan();
        }
    }

    //Return the value that is closest
    if (dist(horiz.x, horiz.y, startx, starty) < dist(vert.x, vert.y, startx, starty)
        && horiz.tile_type != 0)
        || vert.tile_type == 0
    {
        return horiz;
    } else {
        return vert;
    }
}

fn invert_u8(val: u8) -> u8 {
	if val == 0 {
		return 1;	
	}
	
	0
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
	
	let mut left_state = ButtonState::Released;

	let mut camx = 0.5f64;
	let mut camy = 0.5f64;
	let mut cam_rotation = 0.0f64;
	let mut cam_rotation_speed = 0.0f64;
	let mut cam_speed = 0.0f64;
	const FOV: f64 = 3.14159 / 3.0;
	
	let mut mode = Mode::Editor;

	let mut dt = 0.0f64;

	'running: loop {
		let frame_start = Instant::now(); 

		let mouse_state = event_pump.mouse_state();

		canvas.set_draw_color(Color::BLACK);
		canvas.clear();
		
		match mode {
			Mode::Editor => {
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
				canvas.fill_rect(Rect::new(camx as i32 * 32, camy as i32 * 32, 32, 32))
					.map_err(|e| e.to_string())?;
	
				canvas.set_draw_color(Color::YELLOW);
				canvas.draw_rect(Rect::new(mouse_state.x() / 32 * 32, mouse_state.y() / 32 * 32, 32, 32))
					.map_err(|e| e.to_string())?;
	
				let mousex = (mouse_state.x() / 32) as isize;
				let mousey = (mouse_state.y() / 32) as isize;
				
				if mouse_state.left() {
					if let ButtonState::Released = left_state {
						level.set_tile(mousex, mousey, invert_u8(level.get_tile(mousex, mousey)));	
					}
	
					left_state = ButtonState::Held;
				} else {
					left_state = ButtonState::Released;
				}
	
				if mouse_state.right() {
					camx = mousex as f64 + 0.5;
					camy = mousey as f64 + 0.5;
				}
			}	
			Mode::Game => {
				let mut angle = -FOV / 2.0 + cam_rotation;
				for i in 0..800 {
					let hit = raycast(camx, camy, angle, 32.0, &level);
					let d = (hit.x - camx) * cam_rotation.cos() + (hit.y - camy) * cam_rotation.sin();
					
					if hit.tile_type != 0 {
						canvas.set_draw_color(Color::GREEN);
						if hit.y.floor() == hit.y {		
							canvas.set_draw_color(Color::RGB(0, 200, 0));
						}

						canvas.draw_line(Point::new(i, (-300.0 / d + 300.0) as i32), 
										 Point::new(i, (300.0 / d + 300.0) as i32))
							.map_err(|e| e.to_string())?;
	
					}

					angle += FOV / 800.0;
				}
			}
		}

		canvas.present();	

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => { break 'running; }
				Event::KeyDown { keycode: Some(Keycode::P), repeat: false, .. } => {
					if let Mode::Editor = mode {
						mode = Mode::Game;	
					} else {
						mode = Mode::Editor;	
					}
				}
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
					cam_speed = 2.0f64;	
				}
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
					cam_speed = -2.0f64;	
				}
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
					cam_rotation_speed = -1.0f64;	
				}
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
					cam_rotation_speed = 1.0f64;	
				}
				Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
					cam_speed = 0.0f64;	
				}
				Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
					cam_speed = 0.0f64;	
				}
				Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
					cam_rotation_speed = 0.0f64;	
				}
				Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
					cam_rotation_speed = 0.0f64;	
				}
				_ => {}	
			}
		}

		if let Mode::Game = mode {
			camx += cam_speed * cam_rotation.cos() * dt;
			camy += cam_speed * cam_rotation.sin() * dt;
			cam_rotation += cam_rotation_speed * dt;
		}

		dt = frame_start.elapsed().as_secs_f64();
	}

	Ok(())
}
