use texture_atlasser as ta;
use std::{error::Error, fs, path::{Path, PathBuf}};

// Creates a new atlas from a directory of images
fn main() -> Result<(), Box<dyn Error>>{
    let directory = Path::new("./examples/assets");

    // Find all files inside the directory. We're assuming there are only images or directories, no other files.
    let path_buffs: Vec<PathBuf> = fs::read_dir(directory).unwrap().filter_map(|entry| {
        let entry = entry.ok()?;
        if entry.file_type().unwrap().is_file() {
            return Some(entry.path());
        } else {
            return None;
        }
    }).collect();
    let paths: Vec<&Path> = path_buffs.iter().map(|path_buf| {path_buf.as_path()}).collect();
    //let paths: Vec<&Path> = vec!(Path::new("./examples/assets/grass.png"));
    let options = ta::AtlasOptions { 
        width: 512,
        height: 512,
        margin: 3,
        max_atlantes: 1,
        try_smaller: Some(1.1),
        cut_down: true,
    };

    let atlas = ta::atlas_paths(&paths, options)?;
    
    // Save the atlantes
    for (i, image) in atlas.atlantes.iter().enumerate() {
        image.save(format!("./examples/assets/atlantes/dir_atlas{}.png", i))?;
    }
    // Log the positions of the single textures inside the atlantes into the console
    for rect in atlas.rects.iter() {
        println!("{:?}", rect);
    }
    Ok(())
}
