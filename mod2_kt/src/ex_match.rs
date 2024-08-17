use std::io;

pub fn run(){
    println!("Say hello or good bye");
    let mut greeting =String::new();
     io::stdin().read_line(&mut greeting).expect("Failed to read line");

    match greeting.trim(){
        "hello" => println!("{:?}",greeting),
        "good bye" => println!("So long!"),
        _ => println!("I don't know what you said..."),
    }

}