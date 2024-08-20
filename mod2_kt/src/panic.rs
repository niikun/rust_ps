fn loop_and_panic(numbers: Vec<i32>){
    for num in numbers{
        if num <0{
            panic!("Negative number found:{}",num);
        }
        println!("Number :{}",num)
    }
}

pub fn run(){
    // panic!("error we are crashing the program");
    loop_and_panic(vec![1,2,3,4,-5]);
}