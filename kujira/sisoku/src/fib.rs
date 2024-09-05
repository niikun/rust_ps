fn main() {
    //     let mut a = 1;
    //     let mut b = 1;
    //     println!("{}", a);
    //     println!("{}", b);
    //     for _ in 0..10 {
    //         let tmp = a;
    //         a = b;
    //         b = tmp + b;
    //         println!("{}", b);
    //     }

    // let pc_price = 98000.0;
    // let price_a = 1200.0 + 0.8 * pc_price;
    // let price_b = pc_price * 0.9;
    // if price_a > price_b {
    //     println!("Bのほうが {} 円安い", (price_a - price_b) as u32);
    // } else {
    //     println!("Aのほうが {} 円安い", (price_b - price_a) as u32);
    // }
    let int_str = "42";
    let float_str = "3.14";

    // 文字列を整数に変換
    let int_value: u32 = int_str.parse().expect("Not a valid integer!");
    println!("整数値: {}", int_value);

    // 文字列を浮動小数点数に変換
    let float_value: f64 = float_str.parse().expect("Not a valid float!");
    println!("浮動小数点数値: {}", float_value);
}
