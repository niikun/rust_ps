fn process_numbers(numbers: &[i32]){
    let mut sum = 0;
    for &num in numbers{
        sum += num;
    }
    println!("Sum :{}",sum);

    if sum %2 == 0{
        println!("Sum is even");
    } else {
        println!("Sum is odd");
    }
}

pub fn run(){
    let numbers = [1,2,3,4,5];
    process_numbers(&numbers);
}