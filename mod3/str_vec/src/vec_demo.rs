pub fn run() {
    // let mut numbers: Vec<i32> = Vec::new();
    // numbers.push(1);
    // numbers.push(2);
    // numbers.insert(1, 3);
    // numbers.pop();
    // println!("Vec: {:?}", numbers);

    ownership();

    modifiable()

    // let my_list = [1, 2, 3, 4, 5];
    // println!("{:?}", my_list);
}

fn ownership() {
    let numbers = vec![1, 2, 3];
    let slice_num = &numbers[..];
    println!("{:?}", slice_num);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    let mut slice = &mut numbers[..];
    slice[0] = 10;
    // slice.push(4);
    println!("{:?}", slice);
}
