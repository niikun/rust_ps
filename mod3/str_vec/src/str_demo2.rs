pub fn run() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // println!("Sentence : {}", &sentence[0..=5]);

    // let description = format!("Title: Quick story\n{}", &sentence);
    // println!("{}", description);

    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("{} is a vowel", c),
    //         _ => continue,
    //     }
    // }

    let words: Vec<&str> = sentence.split_whitespace().collect();
    let words2: Vec<&str> = sentence.split(" ").collect();
    println!("{:?}", words);
    println!("{:?}", words2);
    for word in words {
        println!("{}", word);
    }
    for word in words2 {
        println!("{}", word);
    }
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{:?}", reversed);
}
