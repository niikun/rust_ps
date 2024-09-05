use std::io::{self, Write};

fn main() {
    let gengo = ["昭和", "平成", "令和"];

    println!("西暦を入力してください");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failde to read line");
    let mut year: u32 = input.trim().parse().expect("Please type a number");
    // for i in 1926..2027 {
    if year < 1989 {
        if year == 1926 {
            println!("{}年 => {} 元年", year, gengo[0]);
        }
        println!("{}年 => {} {}年", year, gengo[0], year - 1926);
    } else if year < 2019 {
        if year == 1989 {
            println!("{}年 => {} 元年", year, gengo[1]);
        }
        println!("{}年 => {} {}年", year, gengo[1], year - 1989);
    } else {
        if year == 2019 {
            println!("{}年 => {} 元年", year, gengo[2]);
        }
        println!("{}年 => {} {}年", year, gengo[2], year - 2019);
    }
}

// fn main() {
//     let gengo = vec!["昭和", "平成", "令和"];

//     for i in 1926..=2026 {
//         if i < 1989 {
//             if i == 1926 {
//                 println!("{}年 => {} 元年", i, gengo[0]);
//                 continue;
//             }
//             println!("{}年 => {} {}年", i, gengo[0], i - 1926);
//         } else if i < 2019 {
//             if i == 1989 {
//                 println!("{}年 => {} 元年", i, gengo[1]);
//                 continue;
//             }
//             println!("{}年 => {} {}年", i, gengo[1], i - 1989);
//         } else {
//             if i == 2019 {
//                 println!("{}年 => {} 元年", i, gengo[2]);
//                 continue;
//             }
//             println!("{}年 => {} {}年", i, gengo[2], i - 2019);
//         }
//     }
// }
