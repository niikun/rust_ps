pub fn run() {
    // let vec = vec![1, 2, 3, 4, 5];
    // let vec = vec!["a", "b", "c", "d", "e"];
    // // let vec: Vec<i32> = Vec::new();
    // // let third_val: i32 = vec[2];
    // // println!("Third value: {}", third_val);

    // // let last_val = vec.last().unwrap();
    // // println!("Last value: {:?}", last_val);

    // let second_val = vec.get(1);
    // println!("Second value: {:?}", second_val);

    // let first_val = vec.first();
    // println!("First value: {:?}", first_val);
    // match first_val {
    //     Some(val) => println!("first_val : {}", val),
    //     None => println!("No value"),
    // }

    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("{:?}", vec);

    let more_num = vec![5, 6];
    vec.extend(&more_num);
    println!("{:?}", vec);
    println!("{:?}", more_num);

    let mut other_nums = vec![7, 8, 9];
    vec.append(&mut other_nums);
    println!("{:?}", vec);
    println!("{:?}", other_nums);
    vec.insert(0, 0);
    println!("{:?}", vec);
}
