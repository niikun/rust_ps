fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

pub fn run() {
    let a = 10;
    let b = 0;

    let result = divide(a, b);

    match result {
        Some(x) => println!("Result:{}", x),
        None => println!("Error : division by 0"),
    }
}
