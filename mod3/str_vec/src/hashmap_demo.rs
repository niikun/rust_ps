use std::collections::HashMap;

pub fn run() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipies."),
    );
    reviews.insert(
        String::from("Programming with Rust"),
        String::from("Great book for beginners."),
    );

    println!("Reviews: {:?}", reviews);
    let roman_review = reviews.get("Ancient Roman History").unwrap();
    println!("roman_review: {:?}", &roman_review);

    let obsolete: &str = "Ancient Roman History";
    reviews.remove(obsolete);
    println!("Reviews: {:?}", reviews);
}
