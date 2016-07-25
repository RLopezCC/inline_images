extern crate regex;
extern crate rustc_serialize;

use std::env;
use std::io::{self, Read, Write};
use std::fs::File;
//use std::marker::Copy;
use regex::Regex;

use rustc_serialize::base64::{ToBase64, STANDARD};


fn replace_contents(line: &str) -> String { 
    let args: Vec<String> = env::args().collect();
    let assets_dir = args[1].clone();

    let re_path = Regex::new(r"url\('(?P<path>.+?)'\);").unwrap();
    let cap = re_path.captures(line).unwrap();
    let path = cap.name("path").unwrap();
    let mut src_path = assets_dir;
    let extension = path.split('.').last().unwrap();


    src_path.push_str(path);

    let mut image_file = match File::open(&src_path) {
        Ok(f) => {f},
        Err(_) => {
            let mut stderr = std::io::stderr();
            writeln!(&mut stderr, "Cannot open {} Skipping.", src_path).unwrap();
            return line.to_string();
        }
    };

    let mut file_contents = Vec::new();
    match image_file.read_to_end(&mut file_contents) {
        Ok(_) => {},
        Err(_) => {
            println!("Cannot read {}. Skipping.", src_path);
            return line.to_string();
        }
    }

    let mut new_property = String::new();
    new_property.push_str("data:image/");
    new_property.push_str(extension);
    new_property.push_str(";base64,");
    new_property.push_str(&file_contents.to_base64(STANDARD));

    let new_line: String;
    new_line = line.replace(path, &new_property);
    return new_line;
}


fn main() {
    let mut contents = String::new();

    match io::stdin().read_to_string(&mut contents) {
      Ok(_) => {},
      Err(_) => panic!("Cannot read stdin, what")
    };

    let lines_split = contents.split('\n');
    for line in lines_split {
        if line.contains("background-image: url") {
            println!("{}", replace_contents(line));
        } else {
            println!("{}", line);
        }
    }
}
