use std::{collections::{BTreeMap, HashMap}};
use std::path::Path;

use image::{self, DynamicImage, GenericImage, GenericImageView, ImageError, imageops::FilterType};

use rectangle_pack as rp;

use crate::error::AtlasError;


/// Holds the created textures and pairs of image_index, atlas_index, Rect describing where each image sits in the created atlasses.
pub struct AtlasLayout {
    /// The list of atlas textures, default format is rgba16
    pub atlantes: Vec<DynamicImage>,
    /// Specifies the positions of the textures inside the atlantes
    pub rects: Vec<(u32, u32, Rect)>
}

#[derive(Debug)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}

/// Options passed to 
pub struct AtlasOptions {
    pub width: u32,
    pub height: u32,
    pub margin: u32,
    pub max_atlantes: u32,
}

pub fn atlas_paths(paths: &[&Path], options: AtlasOptions) -> Result<AtlasLayout, AtlasError> {
    let buffers = load_textures(paths)?;
    atlas_buffers(buffers, options)
}

pub fn atlas_buffers(mut buffers: Vec<DynamicImage>, options: AtlasOptions) -> Result<AtlasLayout, AtlasError> {

    let mut rects = rp::GroupedRectsToPlace::<_, usize>::new();
    let mut bins = BTreeMap::new();


    let mut max_width = 0;
    let mut max_height = 0;
    
    add_margins(&mut buffers, &options);

    for (id, buffer) in buffers.iter().enumerate() {
        let (width, height) = buffer.dimensions();
        max_width = width.max(max_width);
        max_height = height.max(max_height);

        rects.push_rect(id, None, rp::RectToInsert::new(width, height, 1));
    }

    if max_width > options.width || max_height > options.height {
        return Err(AtlasError::PackingError("There is an image larger than the atlas size! This includes margins!"));
    }

    let bin_count = options.max_atlantes.min(2*(buffers.len() as f32 * (max_height as f32 / options.height as f32).max(max_width as f32 / options.width as f32)).ceil() as u32);

    //let (current_width, current_height) = if options.

    //while options.width 
    for i in 0..bin_count {
        bins.insert(i, rp::TargetBin::new(options.width, options.height, 1));
    }

    let placement = match rp::pack_rects(&rects, &mut bins, &rp::volume_heuristic, &rp::contains_smallest_box) {
        Ok(pack) => {pack}
        Err(err) => {return Err(AtlasError::PackingError("Can't fit the images into the amount of bins specified!"));}
    };

    let mut rects = Vec::new();

    let mut locations: Vec<(&usize, &(u32, rp::PackedLocation))> = placement.packed_locations().iter().collect();
    locations.sort_by(|a, b| {a.0.cmp(b.0)});

    let mut index_map = HashMap::new();
    let mut atlantes = Vec::new();

    for (image_index, (bin_index, location)) in locations {
        let image = buffers.remove(0);
        if !index_map.contains_key(bin_index) {
            let empty_image = DynamicImage::new_rgba16(options.width, options.height);
            index_map.insert(*bin_index, atlantes.len());
            atlantes.push(empty_image);
        }

        // can unwrap here, as we gurantee the key to be there
        let atlas = &mut atlantes[*index_map.get(bin_index).unwrap()];
        // can unwrap here, as we already know that the image will fit
        atlas.copy_from(&image, location.x(), location.y()).unwrap();

        rects.push((
            *image_index as u32, 
            *bin_index, 
            Rect {
                width: image.width(),
                height: image.height(),
                x: location.x() + options.margin,
                y: location.y() + options.margin,
            }
        ));
    }

    let atlas = AtlasLayout {
        atlantes: atlantes,
        rects: rects,
    };
    
    Ok(atlas)
}

/// Loads all specified textures and converts them to the format of the first image
fn load_textures(paths: &[&Path])  -> Result<Vec<DynamicImage>, ImageError> {
    let mut buffers = Vec::with_capacity(paths.len());
    for path in paths {
        buffers.push(image::open(path)?);
    }
    Ok(buffers)
}

/// Adds a margin specified by options to every image in buffers
fn add_margins(buffers: &mut [DynamicImage], options: &AtlasOptions) {
    if options.margin == 0 {
        return;
    }
    for image in buffers {
        let mut new_image = DynamicImage::new_rgba16(image.width() + 2*options.margin, image.height() + 2*options.margin);
        // can unwrap here and in the following copy_froms as the size is guranteed to fit
        new_image.copy_from(image, options.margin, options.margin).unwrap();

        // left margin
        new_image.copy_from(&image.crop_imm(0, 0, 1, image.height()).resize_exact(options.margin, image.height(), FilterType::Nearest), 0, options.margin).unwrap();
        // right margin
        new_image.copy_from(&image.crop_imm(image.width()-1, 0, 1, image.height()).resize_exact(options.margin, image.height(), FilterType::Nearest), image.width()+options.margin, options.margin).unwrap();
        // top 
        new_image.copy_from(&image.crop_imm(0, 0, image.width(), 1).resize_exact(image.width(), options.margin, FilterType::Nearest), options.margin, 0).unwrap();
        // bottom margin
        new_image.copy_from(&image.crop_imm(0, image.height()-1, image.width(), 1).resize_exact(image.width(), options.margin, FilterType::Nearest), options.margin, image.height()+options.margin).unwrap();
        
        // create corner margin based on the corner pixel
        for (dx, dy) in vec!((0, 0), (0, 1), (1, 1), (1, 0)) {
            let pixel = image.get_pixel(dx*(image.width()-1), dy*(image.height()-1));
            for x in 0..options.margin {
                for y in 0..options.margin {
                    new_image.put_pixel(dx*(image.width()+options.margin) + x, dy*(image.height()+options.margin) + y, pixel);
                }
            }
        }

        std::mem::swap(&mut new_image, image);
    }
}