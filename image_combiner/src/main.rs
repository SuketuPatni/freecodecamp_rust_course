mod arguments;
use image::{io::Reader, DynamicImage, ImageFormat};
use std::{io::BufReader, fs::File};

fn main() {
    let args = arguments::Args::new();
    let (image_1, image_format_1) = find_image(args.image_1);
    let (image_2, image_format_2) = find_image(args.image_2);
}

fn find_image(path:String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap(); 
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
    (image, image_format)
}
