pub fn run(){
    // let maybe_number:Option<Option<()>> = Some(None);
    let maybe_number = Some("42");
    if let Some(num) = maybe_number{
        println!("The number is:{:?}",num);
    } else { 
        println!("There is no number");
    }

}

// pub fn maybe_square(x:i32) ->Option<i32>{
//     if x > 0 {
//         Some(x*x)
//     } else {
//         None
//     }
// }