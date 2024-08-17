enum Toy{
    Car,
    Doll,
    Ball,
}


pub fn run(){
    let toy = Toy::Car;
    match toy {
        Toy::Car => println!("Caaaaaar"),
        Toy::Doll => println!("Doooool"),
        Toy::Ball => println!("Baaaaall"),
    }
}