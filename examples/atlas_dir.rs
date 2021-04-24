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
    // Convert PathBufs into &Paths
    let paths: Vec<&Path> = path_buffs.iter().map(|path_buf| {path_buf.as_path()}).collect();

    // We want all our images inside a single (small) texture atlas with a 3 pixel margin, so we set max_atlantes to 1, enable cut_down & try_smaller. 
    // width & height just need to be enough here, as the atlasser will try to minimize the dimensions anyways
    let options = ta::AtlasOptions { 
        width: 512,
        height: 512,
        margin: 3,
        max_atlantes: 1,
        try_smaller: Some(1.1),
        cut_down: true,
    };

    // Create the atlas
    let atlas = ta::atlas_paths(&paths, options)?;
    
    // Save the atlas
    atlas.atlantes[0].save(format!("./examples/assets/atlantes/dir_atlas.png"))?;
    
    // Log the positions of the single textures inside the atlantes into the console
    for rect in atlas.rects.iter() {
        println!("{:?}", rect);
    }
    Ok(())
}