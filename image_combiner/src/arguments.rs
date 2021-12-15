pub fn nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
    // here we are calling std::env::args() everytime, so the iterator
    // is not advanced everytime nth() is done
}

#[derive(Debug)]
pub struct Args {
    // the paths of the images to be combined and the output
    pub image_1: String,
    pub image_2: String,
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args {
            image_1: nth_arg(1),
            image_2: nth_arg(2),
            output: nth_arg(3),
        }
    }
}
