fn sum(numbers: &[i32]) -> i32 {
    let mut sum_num :i32 = 0;
    for num in numbers {
        sum_num += num;
    }
    sum_num
}

pub fn run(){
    let numbers = vec! [1,2,3,4,5];
    let ans = sum(&numbers);
    println!("sum of {:?} is {}",&numbers,ans);
}