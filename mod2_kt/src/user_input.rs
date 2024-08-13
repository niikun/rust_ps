use std::io;

pub fn run(){
    let mut input =  String::new();
    while input.trim() != "stop"{
        println!("Please enter a number or 'stop' to exit:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");  
        println!("You entered :{}", input) ;
    }
    println!("Goodbye!");
}