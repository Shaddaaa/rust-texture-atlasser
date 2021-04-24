Easy texture atlass creation with various options to fit different needs.

Provides a way to create texture atlasses from lists of [Paths](https://doc.rust-lang.org/stable/std/path/struct.Path.html) or [DynamicImages](https://docs.rs/image/).

# Examples #

Loading different images into atlantes and saving them afterwards:
```rust
use std::path::Path;
use texture_atlasser as ta;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let paths: Vec<&Path> = vec!(
        Path::new("image1.png"), 
        Path::new("image2.png"), 
        Path::new("image3.jpg"),
    );
    let options = ta::AtlasOptions { 
        width: 64,
        height: 64,
        margin: 5,
        max_atlantes: 2,
        try_smaller: None,
        cut_down: true,
    };
    // create the atlas
    let atlas = ta::atlas_paths(&paths, options)?;
    
    // save the atlantes
    for (i, image) in atlas.atlantes.iter().enumerate() {
        image.save(format!("atlas{}.png", i))?;
    }
    // Log the positions of the single textures inside the atlantes into the console
    for rect in atlas.rects.iter() {
        println!("{:?}", rect);
    }
    Ok(())
}
```
For an example loading an entire directory of images, see the examples folders.
