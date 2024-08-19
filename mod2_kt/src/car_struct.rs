#[derive(PartialEq,Debug)]
struct Car{
    color : String,
    motor : Transmission,
    roof : bool,
    age : (Age, u32),
}

#[derive(PartialEq,Debug)]
enum Transmission{
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq,Debug)]
enum Age{
    New,
    Used,
}

fn car_quality(miles:u32)->(Age,u32){
    let quality :(Age,u32) = (Age::New,miles);
    quality
}

fn car_factory(color:String,motor:Transmission,roof:bool,mileage:u32)->Car{
    let age = car_quality(mileage);
    Car{
        color,
        motor,
        roof,
        age,
    }
}
pub fn run(){
    let colors = ["Bule","Green","Red","Silver"];
    let mut car:Car = car_factory(colors[0].to_string(),Transmission::Manual,true,1000);
    println!("Car :{:?}",car);
}


