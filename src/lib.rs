use std::fs;

#[derive(Debug)]
pub struct Args {
    pub number: bool,
    pub show_ends: bool,
}

impl Args {
    fn new() -> Args {
        Args {number: false, show_ends: false}
    }
}

#[derive(Debug)]
pub struct Catr {
    pub files: Vec<String>,
    pub arguments: Args,
}

pub fn execute(args: &Vec<String>) {
    let catr = parse_args(args);

    for file in &catr.files {
        match fs::read_to_string(file) {
            Ok(contents) => println!("{}", contents),
            Err(err) => println!("No such file [{}]: {}", file, err)
        }
    }
}

pub fn parse_args(args: &Vec<String>) -> Catr {
    let mut files: Vec<String> = Vec::new();
    let mut arguments = Args::new();

    for a in args {
        if a.starts_with('-') {
            match a.as_str() {
                "--number" => arguments.number = true,
                "--show-ends" => arguments.show_ends = true,
                _ => println!("Invalid argument [{}]", a),
            }
        } else {
            files.push(String::from(a));
        }
    }

    Catr {files, arguments}
}
