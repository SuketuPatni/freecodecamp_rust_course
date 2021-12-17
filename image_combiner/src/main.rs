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
    let (image_1, image_2) = resize_image(image_1, image_2);
    let output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

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


fn combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec(); // returns a vector of pixels as their rgba values
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2) // the main process of this image combiner
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let length = vec_1.len();
    let mut combined_data = vec![0u8, length as u8];
    let mut i = 0;

    while i < length {
        if i % 8 == 0 {
            combined_data.splice(i..(i+3), set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..(i+3), set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = vec![];
    for i in start..end {
        rgba.push(
            match vec.get(i) {
                Some(d) => *d,
                None => panic!("Index out of bounds")
            }
        )
    }
    rgba
}
