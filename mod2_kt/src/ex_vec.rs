pub fn run(){
    let three_nums = vec![15,3,46];
    let names: Vec<&str> = vec!["bob","frank"];
    println!("first vec :{:?}",three_nums);
    println!("first name :{}",names[0]);
    let mut fruit = Vec::new();
    fruit.push("apple");
    fruit.push("orange");
    fruit.push("banana");
    println!("fruit address :{:p}",fruit.as_ptr());
    println!("fruit :{}",fruit.len());
    print!("fruit :{}",fruit.capacity());
    println!("fruit :{:?}",fruit);
    fruit.pop();
    println!("fruit address :{:p}",fruit.as_ptr());
    println!("fruit :{}",fruit.len());
    print!("fruit :{}",fruit.capacity());
    println!("fruit :{:?}",fruit);
}