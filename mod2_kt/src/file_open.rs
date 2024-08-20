use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn run(){
    let file = File::open("src/sample.txt");
    let file = match file{
        Ok(file) => file,
        Err(error) =>{
            match error.kind(){
                std::io::ErrorKind::NotFound => {
                    panic!("File not found:{}",error);
                }
                _ =>{
                panic!("Error opening the file:{}",error);
                }
            }
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines(){
        println!("{}",line.unwrap());
    }
}