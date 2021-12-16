mod arguments;
use image::{io::Reader, DynamicImage, ImageFormat};
use std::{io::BufReader, fs::File};

enum ImageDataError {
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataError> {
    let args = arguments::Args::new();
    let (image_1, image_format_1) = find_image(args.image_1);
    let (image_2, image_format_2) = find_image(args.image_2);

    if image_1_format != image_2_format {
        Err(ImageDataError::DifferentImageFormats)
    } 

    Ok(())
}

fn find_image(path:String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap(); 
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimensions(dim1: (u32,u32), dim2: (u32,u32)) -> (u32,u32) {
    match dim1.0 * dim1.1 <= dim2.0 * dim2.1 {
        true => dim1,
        false => dim2
    }
}
