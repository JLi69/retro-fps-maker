use crate::level::Level;
use crate::sprite::{dist, new_vector2, Vector2f64};

fn scan_vertical_positive(
    start: &Vector2f64,
    angle: f64,
    max_dist: f64,
    level: &Level,
) -> (Vector2f64, u8) {
    let mut rayx = start.x.ceil();
    let mut rayy = (rayx - start.x) * angle.tan() + start.y;
    while (start.x - rayx).abs() < max_dist {
        let xind = rayx as isize;
        let yind = rayy.floor() as isize;

        if !level.out_of_bounds(xind, yind) && level.get_tile(xind, yind) != 0 {
            return (new_vector2(rayx, rayy), level.get_tile(xind, yind));
        }

        rayx += 1.0;
        rayy += angle.tan();
    }

    (new_vector2(rayx, rayy), 0)
}

fn scan_vertical_negative(
    start: &Vector2f64,
    angle: f64,
    max_dist: f64,
    level: &Level,
) -> (Vector2f64, u8) {
    let mut rayx = start.x.floor();
    let mut rayy = (rayx - start.x) * angle.tan() + start.y;
    while (start.x - rayx).abs() < max_dist {
        let xind = rayx as isize - 1;
        let yind = rayy.floor() as isize;

        if !level.out_of_bounds(xind, yind) && level.get_tile(xind, yind) != 0 {
            return (new_vector2(rayx, rayy), level.get_tile(xind, yind));
        }

        rayx -= 1.0;
        rayy -= angle.tan();
    }

    (new_vector2(rayx, rayy), 0)
}

fn scan_horizontal_positive(
    start: &Vector2f64,
    angle: f64,
    max_dist: f64,
    level: &Level,
) -> (Vector2f64, u8) {
    let mut rayy = start.y.ceil();
    let mut rayx = (rayy - start.y) * 1.0 / angle.tan() + start.x;
    while (start.y - rayy).abs() < max_dist {
        let xind = rayx.floor() as isize;
        let yind = rayy as isize;

        if !level.out_of_bounds(xind, yind) && level.get_tile(xind, yind) != 0 {
            return (new_vector2(rayx, rayy), level.get_tile(xind, yind));
        }

        rayy += 1.0;
        rayx += 1.0 / angle.tan();
    }

    (new_vector2(rayx, rayy), 0)
}

fn scan_horizontal_negative(
    start: &Vector2f64,
    angle: f64,
    max_dist: f64,
    level: &Level,
) -> (Vector2f64, u8) {
    let mut rayy = start.y.floor();
    let mut rayx = (rayy - start.y) * 1.0 / angle.tan() + start.x;
    while (start.y - rayy).abs() < max_dist {
        let xind = rayx.floor() as isize;
        let yind = rayy as isize - 1;

        if !level.out_of_bounds(xind, yind) && level.get_tile(xind, yind) != 0 {
            return (new_vector2(rayx, rayy), level.get_tile(xind, yind));
        }

        rayy -= 1.0;
        rayx -= 1.0 / angle.tan();
    }

    (new_vector2(rayx, rayy), 0)
}

//Returns x, y, and tile type
pub fn raycast(start: &Vector2f64, angle: f64, max_dist: f64, level: &Level) -> (Vector2f64, u8) {
    //Check vertical lines
    let vert = if angle.cos() > 0.0 {
        scan_vertical_positive(start, angle, max_dist, level)
    } else {
        scan_vertical_negative(start, angle, max_dist, level)
    };

    //Check horizontal lines
    let horiz = if angle.sin() > 0.0 {
        scan_horizontal_positive(start, angle, max_dist, level)
    } else {
        scan_horizontal_negative(start, angle, max_dist, level)
    };

    //Return the value that is closest
    if (dist(&horiz.0, start) < dist(&vert.0, start) && horiz.1 != 0) || vert.1 == 0 {
        horiz
    } else {
        vert
    }
}
