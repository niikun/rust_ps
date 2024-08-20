fn split_string(s:String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    match result {
        Some(value) => return value.to_string(),
        None => return "Not Found".to_string(),
    }
    // result.to_string()
}

pub fn run(){
    let chunk: String = split_string("hello,world".to_string(), ',', 0);
    println!("chunk :{}",chunk);
}   