pub fn run() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.insert(1, 3);
    numbers.pop();
    println!("Vec: {:?}", numbers);
}
