#[derive(Debug)]
struct Person {
    first_name:String,
    last_name:String,
    age:Option<u8>,
    contact:u8,
}

impl Person{
    fn new(first_name:String,last_name:String,age:Option<u8>)->Person{
        Person{
            first_name,
            last_name,
            age,
            contact : 1,
        }
    }
}


fn main(){
    let p1 = Person{
        first_name:String::from("John"),
        last_name:String::from("Doe"),
        age:Some(30),
        contact:1,
    };
    let mut p2 = Person::new(String::from("Jane"),String::from("Doe"),None);
    println!("{:?}",p1);
    println!("{:?}",p2);
    p2.contact = 2;
    println!("{:?}",p2);
    p2.first_name = String::from("Aki");
    println!("{:?}",p2);
}
