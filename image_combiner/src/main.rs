mod arguments;
use image::{io::Reader,
            DynamicImage, 
            ImageFormat, 
            imageops::FilterType::Triangle,
            GenericImageView};
use std::{io::BufReader, fs::File};
use std::convert::TryInto;

#[derive(Debug)]
enum ImageDataError {
    DifferentImageFormats,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    image_name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, image_name: String) -> Self {
        let buffer = Vec::with_capacity((height * width * 4).try_into().unwrap()); // 4 for the r,g,b and alpha values per pixel
        FloatingImage {
            width: width,
            height: height,
            data: buffer,
            image_name: image_name // name of image along with its path
        }
    }
}

fn main() -> Result<(), ImageDataError> {
    let args = arguments::Args::new();
    let (image_1, image_1_format) = find_image(args.image_1);
    let (image_2, image_2_format) = find_image(args.image_2);

    if image_1_format != image_2_format {
        return Err(ImageDataError::DifferentImageFormats);
    } 
    let output = FloatingImage::new(image_1.width, image_1.height, args.output)

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

fn resize_image(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, \nheight: {}", width, height);

    match (width, height) == image_2.dimensions() { // .dimensions() returns (width, height) both u32
        true => (image_1.resize_exact(width, height, Triangle), image_2), // for most cases Triangle is the best option
                                                                          // among FilterTypes
        false => (image_1, image_2.resize_exact(width, height, Triangle))
    }
}





















