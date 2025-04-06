use std::fs;

#[derive(Debug)]
pub struct CatArgs {
    pub files: Vec<String>,
    pub arguments: Vec<String>,
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

pub fn parse_args(args: &Vec<String>) -> CatArgs {
    let mut arguments: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for a in args {
        if a.starts_with('-') {
            arguments.push(String::from(a));
        } else {
            files.push(String::from(a));
        }
    }

    CatArgs {files, arguments}
}
