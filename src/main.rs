use display_info::DisplayInfo;
use image::{ImageReader,imageops::FilterType,GenericImageView};
use std::fs;

fn main() {

    let path = match std::env::args().nth(1) {
        Some(v) => v,
        None => { 
            eprintln!("The path must be provided! \n ex. img-resize -- [path to folder] [max width, 0=screen] [max height, 0=screen] [speed]");
            return;
        }
    };

    let max_width = get_dimension(2, "width".to_string());
    let max_height = get_dimension(3, "height".to_string()); 

    let filter_type = match std::env::args().nth(4) {
        Some(v) => if v=="fastest" {
            FilterType::Nearest
        } else if v=="fast" {
            FilterType::Triangle
        } else if v=="medium" {
            FilterType::CatmullRom
        } else if v=="slow" {
            FilterType::Gaussian
        } else {
            FilterType::Lanczos3
        },
        None => FilterType::Lanczos3
    };


    let entries = match fs::read_dir(&path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Given path, {} couldn't be read because \n {}", &path, e);
            return;
        }
    };

    let new_path = std::path::Path::new(&path).join("resized-images");

    match fs::exists(&path) {
        Ok(path_exists) => if path_exists {
            match fs::exists(&new_path) {
                Ok(new_path_exists) => if !new_path_exists {
                    match fs::create_dir(&new_path) {
                        Err(e) => {
                            eprintln!("Could not create new folder to store resized images due to \n{}", e);
                            return;
                        },
                        Ok(_) => println!("New folder created to store resized images on {}", &path)
                    }
                },
                Err(e) => {
                    eprintln!("Couldn't check if new location exists or not due to, \n{}", e);
                    return;
                }
            }
        },
        Err(e) => {
            eprintln!("Couldn't check if given path exists or not due to, \n{}", e);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not read item in folder because of the following error(s)\n{}",e);
                return;
            }
        };
        let entry_path = entry.path();
        if !entry_path.is_dir() {
            let ext = match &entry_path.extension() {
                None => "none".to_string(),
                Some(str) => str.to_str().unwrap_or_else(||"none").to_string()
            };
            if is_image(&ext) {
                println!("Resizing file {:?}", &entry_path);

                let mut img = match ImageReader::open(&entry_path) {
                    Ok(opened_img) => match opened_img.decode() {
                        Ok(decoded_img) => decoded_img,
                        Err(e) => {
                            eprintln!("Couldn't decode image, {:?}, for the following errors\n{}", &entry_path, e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Couldn't open image, {:?}, for the following errors\n{}", &entry_path, e);
                        return;
                    }
                };

                img = img.resize(
                        if max_width==0 {img.dimensions().0} else {max_width},
                        if max_height==0 {img.dimensions().0} else {max_height},
                        filter_type);


                let save_file = &new_path.join(match &entry_path.file_name() {
                    Some(file_name) => match file_name.to_str() {
                        Some(file_name_as_string) => {file_name_as_string},
                        None => {
                            eprintln!("New file name path couldn't be converted to a string");
                            return;
                        }
                    },
                    None => {
                        eprintln!("Couldn't get new file name");
                        return;
                    }
                });

                match img.save(save_file) {
                    Err(e) => {
                        eprintln!("New image couldn't be saved for the following error(s)\n{}",e);
                        return;
                    },
                    Ok(_)=>{}
                };

            } else {
                println!("Skipping file {} since it's not an image", entry_path.display());
            }
        }
    }

}

fn get_dimension(nth: usize, prop: String) -> u32 {
    match std::env::args().nth(nth) {
        Some(v) => match v.parse() {
            Err(_) => {
                eprintln!("max height is either default (use 0 for screen size) or a number, eg. 900",);
                return 0;
            },
            Ok(u) => u
        },
        None => {
            let data = &DisplayInfo::all().unwrap()[0];
            if prop=="height" { data.height }
            else { data.width }
        }
    }
}

fn is_image(str: &str) -> bool {
    let image_exts = ["bmp", "gif", "hdr", "ico", "jpeg", "jpg", "exr", "png", "pnm", "qoi", "tga", "tiff", "webp"];
    image_exts.contains(&str)
}
