use std::env;
use catr;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    catr::execute(&args);
}
