pub struct Level {
    pub width: usize,
    pub height: usize,
    level_data: Vec<u8>,
    pub spawnx: f64,
    pub spawny: f64,
}

impl Level {
    pub fn new(w: usize, h: usize) -> Self {
        Level {
            width: w,
            height: h,
            level_data: vec![0u8; w * h],
            spawnx: 0.0,
            spawny: 0.0,
        }
    }

    pub fn out_of_bounds(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize
    }

    pub fn get_tile(&self, x: isize, y: isize) -> u8 {
        if self.out_of_bounds(x, y) {
            return 0;
        }

        self.level_data[self.width * y as usize + x as usize]
    }

    pub fn set_tile(&mut self, x: isize, y: isize, tile: u8) {
        if self.out_of_bounds(x, y) {
            return;
        }

        self.level_data[self.width * y as usize + x as usize] = tile;
    }

    pub fn level_data_bytes(&self) -> &[u8] {
        &self.level_data
    }
}
