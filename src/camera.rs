use crate::{
    events::InputState,
    sprite::{new_vector2, Vector2f64},
};
use sdl2::keyboard::Scancode;

pub struct Camera {
    pub position: Vector2f64,
    pub rotation: f64,
    pub speed: f64,
    pub rotation_speed: f64,
    pub fov: f64,
}

impl Camera {
    pub fn new(x: f64, y: f64, cam_rotation: f64, cam_fov: f64) -> Self {
        Camera {
            position: new_vector2(x, y),
            rotation: cam_rotation,
            fov: cam_fov,
            speed: 0.0,
            rotation_speed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position.x += self.speed * self.rotation.cos() * dt;
        self.position.y += self.speed * self.rotation.sin() * dt;
        self.rotation += self.rotation_speed * dt;
    }

    pub fn handle_key_input(&mut self, input: &InputState) {
        if input.key_is_clicked(Scancode::Up) {
            self.speed = 2.0;
        } else if input.key_is_clicked(Scancode::Down) {
            self.speed = -2.0;
        } else if !input.key_is_held(Scancode::Up) && !input.key_is_held(Scancode::Down) {
            self.speed = 0.0;
        }

        if input.key_is_clicked(Scancode::Left) {
            self.rotation_speed = -1.0;
        } else if input.key_is_clicked(Scancode::Right) {
            self.rotation_speed = 1.0;
        } else if !input.key_is_held(Scancode::Left) && !input.key_is_held(Scancode::Right) {
            self.rotation_speed = 0.0;
        }
    }
}
