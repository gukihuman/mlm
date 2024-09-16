use image::{GenericImageView, ImageBuffer};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let forge_dir = Path::new("forge");
    let assets_dir = Path::new("assets");

    for entry in WalkDir::new(forge_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            let relative_path = entry.path().strip_prefix(forge_dir)?;
            let output_dir = assets_dir.join(relative_path);
            fs::create_dir_all(&output_dir)?;

            let mut spritesheets: HashMap<
                String,
                Vec<(PathBuf, image::DynamicImage)>,
            > = HashMap::new();

            for file in WalkDir::new(entry.path())
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if file.path().extension().map_or(false, |ext| ext == "png") {
                    let file_stem =
                        file.path().file_stem().unwrap().to_str().unwrap();
                    let parts: Vec<&str> = file_stem.split('_').collect();
                    if parts.len() >= 2 {
                        let sprite_name = parts[0].to_string();
                        let img = image::open(file.path())?;
                        spritesheets
                            .entry(sprite_name)
                            .or_default()
                            .push((file.path().to_path_buf(), img));
                    }
                }
            }

            for (sprite_name, mut images) in spritesheets {
                images.sort_by_key(|(path, _)| path.clone());
                let (width, height) = images[0].1.dimensions();
                let total_width = width * images.len() as u32;
                let mut spritesheet = ImageBuffer::new(total_width, height);

                for (i, (_, img)) in images.iter().enumerate() {
                    image::imageops::replace(
                        &mut spritesheet,
                        img,
                        (i as u32 * width) as i64,
                        0,
                    );
                }

                let output_path =
                    output_dir.join(format!("{}.png", sprite_name));
                spritesheet.save(&output_path)?;
                println!("Generated spritesheet: {}", output_path.display());
            }
        }
    }

    println!("Sprite sheet generation completed successfully.");
    Ok(())
}
