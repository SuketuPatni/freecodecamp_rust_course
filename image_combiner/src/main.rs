mod arguments;
use image::{io::Reader,
            DynamicImage, 
            ImageFormat, 
            imageops::FilterType::Triangle,
            GenericImageView,
            ImageError};
use std::convert::TryInto;

#[derive(Debug)]
enum ImageDataError {
    DifferentImageFormats,
    BufferTooSmall,
    UnableToReadImageFromPath(std::io::Error),
    UnableToFormatImage(String),
    UnableToDecodeImage(ImageError),
    UnableToSaveImage(ImageError)
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
            width,
            height,
            data: buffer,
            image_name, // name of image along with its path
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataError> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataError::BufferTooSmall);
        } 
        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataError> {
    let args = arguments::Args::new();
    let (image_1, image_1_format) = find_image(args.image_1)?; // propagates ImageDataError from find_image function into main  
    let (image_2, image_2_format) = find_image(args.image_2)?;

    if image_1_format != image_2_format {
        return Err(ImageDataError::DifferentImageFormats);
    } 
    let (image_1, image_2) = resize_image(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?; // the "?" propagates the error from set_data into main,
                                     // as both main and set_data return Result<(), ImageDataError>

    if let Err(e) = image::save_buffer_with_format(output.image_name,
                                    &output.data,
                                    output.width,
                                    output.height,
                                    image::ColorType::Rgba8,
                                    image_1_format) {
        Err(ImageDataError::UnableToSaveImage(e)) 
    } else {
        Ok(())
    }
}

// lots of error handling done here in this function
fn find_image(path:String) -> Result<(DynamicImage, ImageFormat), ImageDataError> {
    match Reader::open(&path) { // Reader::open() returns a Result type
        Ok(image_reader) => {
            if let Some(image_format) = image_reader.format() { // image_reader.format() returns an Option Type
                match image_reader.decode() {
                    Ok(image) => Ok((image, image_format)),
                    Err(e) => Err(ImageDataError::UnableToDecodeImage(e))
                }
            } else {
                Err(ImageDataError::UnableToFormatImage(path))
            }
        }, 
        Err(e) => Err(ImageDataError::UnableToReadImageFromPath(e))
    }
}

fn get_smallest_dimensions(dim1: (u32,u32), dim2: (u32,u32)) -> (u32,u32) {
    match dim1.0 * dim1.1 <= dim2.0 * dim2.1 {
        true => dim1,
        false => dim2
    }
}

fn resize_image(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    // println!("width: {}, \nheight: {}", width, height);

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
    let mut combined_data = vec![0u8; vec_1.len()];
    let mut i = 0;

    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &[u8], start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds")
        };
        rgba.push(val);
    }
    rgba
}
