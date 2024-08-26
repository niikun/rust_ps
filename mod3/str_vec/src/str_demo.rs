pub fn run() {
    // let hello = "Hello, World!";
    // println!("String slice : {}", hello);

    // let mut greeting = String::from("hello");
    // println!("String : {:p}", greeting.as_ptr());
    // greeting = String::from("Hello");
    // println!("String : {:p}", greeting.as_ptr());
    // greeting.push_str(", World!");
    // println!("String : {}", greeting);
    // println!("String : {:p}", greeting.as_ptr());

    let s = "hello_world";
    print_str(s);

    let salutation = String::from("Hello, ");
    print_string(&salutation);
}

fn print_str(s: &str) {
    // let mut new_string = s.to_string();
    let mut new_string = format!("{}{}", s, "??");
    new_string.push_str("!!!!!!");
    println!("{}", &new_string);
}
fn print_string(s: &String) {
    println!("{}", s);
}
