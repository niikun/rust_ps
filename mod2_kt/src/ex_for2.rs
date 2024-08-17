pub fn run(){
    for i in 0..10{
        if i %2 == 0{
            continue;
        }
        println!("{}",i);
        if i == 7{
            break;
        }
    }
}