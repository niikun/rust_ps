use std::io;

// pub fn run(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     let num:i32 = input.trim().parse().expect("Not a number");
//     for _ in 0..num {
//         println!("Hello, world!");
//     }
// }

pub fn rev_num(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num = input.trim().parse().expect("Not a number");
    for i in (0..num).rev(){
        println!("{}!",i);
    }


}