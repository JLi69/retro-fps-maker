pub struct Vector2f64 {
    pub x: f64,
    pub y: f64,
}

pub fn new_vector2(px: f64, py: f64) -> Vector2f64 {
    Vector2f64 { x: px, y: py }
}

pub fn dist(v1: &Vector2f64, v2: &Vector2f64) -> f64 {
    ((v1.x - v2.x).powi(2) + (v2.y - v1.y).powi(2)).sqrt()
}

pub struct Sprite {
    pub pos: Vector2f64,
    pub sprite_type: u8
}

impl Sprite {
    pub fn new(x: f64, y: f64, spr_type: u8) -> Self {
        Self {
            pos: new_vector2(x, y),
            sprite_type: spr_type
        }
    }
}
