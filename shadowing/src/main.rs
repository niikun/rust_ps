fn main() {
    
    let mut height = 180;
    height = height -20;
    let result = if height >180{
        "Tall"
    } else if height > 170{
        "Average"
    } else {
        "Short"
    };
    println!("The result is :{}",result);

    let health = if height < 180 {
        "Good"
    } else {
        "Bad"
    };
    println!("The health is :{}",health);

    let health = if height < 180 {true} else {false};

    println!("The health is :{}",health);

}

