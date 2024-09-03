use std::io;

pub fn run() {
    let mut input = String::new();

    loop {
        println!("Enter a string: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();
        if trimmed_input == "stop" {
            break;
        }

        println!("You entered {}", trimmed_input);
        input.clear();
    }
    println!("Goodbye!");
}
