use crate::Level;
use std::{
    fs::File,
    io::{Write, Read}
};

pub fn write_level_file(level: &Level, path: &str) -> Result<(), String> {
    let mut level_file = File::create(path).map_err(|e| e.to_string())?;
    // Write the width and height as bytes to the file
    level_file.write(&level.width.to_be_bytes()).map_err(|e| e.to_string())?;
    level_file.write(&level.height.to_be_bytes()).map_err(|e| e.to_string())?;
    // Write the starting position of the player
    level_file.write(&level.spawnx.to_be_bytes()).map_err(|e| e.to_string())?;
    level_file.write(&level.spawny.to_be_bytes()).map_err(|e| e.to_string())?;
    // Write the level data to the file
    level_file.write_all(level.level_data_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_level_file(path: &str) -> Result<Level, String> {
    let mut level_file = File::open(path).map_err(|e| e.to_string())?;
    
    let mut level = {
        // read the dimensions from the file
        let mut width = [0u8; std::mem::size_of::<usize>()];  
        level_file.read_exact(&mut width).map_err(|e| e.to_string())?;
        let mut height = [0u8; std::mem::size_of::<usize>()];
        level_file.read_exact(&mut height).map_err(|e| e.to_string())?;

        // read the starting position from the file
        let mut startx = [0u8; std::mem::size_of::<f64>()];  
        level_file.read_exact(&mut startx).map_err(|e| e.to_string())?;
        let mut starty = [0u8; std::mem::size_of::<f64>()];
        level_file.read_exact(&mut starty).map_err(|e| e.to_string())?; 

        let mut empty_level = Level::new(
            usize::from_be_bytes(width),
            usize::from_be_bytes(height)
        );
        empty_level.spawnx = f64::from_be_bytes(startx);
        empty_level.spawny = f64::from_be_bytes(starty);

        empty_level
    };

    let mut level_data = vec![0u8; level.width * level.height]; 
    level_file.read(&mut level_data).map_err(|e| e.to_string())?;

    for (i, tile) in level_data.iter().enumerate() {
        let (x, y) = (i % level.width, i / level.width);
        level.set_tile(x as isize, y as isize, *tile);
    }

    Ok(level)
}
