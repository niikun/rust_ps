use std::collections::HashMap;

pub fn run(){
    let mut reviews : HashMap<String,String> = HashMap::new();

    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    let book: &str = "Programming in Rust";
    let res = reviews.get(book);

    match res {
        Some(review) => println!("Review for \'{}\': {:?}", book, review),
        None => println!("No review for \'{}\'", book),
    }

    for (book, review) in &reviews {
        println!("Review for \'{}\': {:?}", book, review);
    }
    let rf_book : &str = "Ancient Roman History";
    println!("Review for \'{}\': {:?}", rf_book, reviews.get(rf_book).unwrap());
    reviews.remove(rf_book);
    println!("Review for \'{}\': {:?}", rf_book, reviews.get(rf_book));
}