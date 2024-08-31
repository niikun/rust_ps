#[derive(Debug, PartialEq)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Rhone,
    Nagano,
}

struct Wine {
    name: String,
    region: WineRegions,
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Bordeaux => println!("Bordeaux is supported"),
        WineRegions::Burgundy => println!("Burgundy is supported"),
        WineRegions::Champagne => println!("Champagne is supported"),
        WineRegions::Rhone => println!("Rhone is supported"),
        _ => println!("{:?} is not Supported", w),
    }
}

pub fn run() {
    let my_wine = Wine {
        name: String::from("niikun"),
        region: WineRegions::Nagano,
    };

    supported_regions(my_wine.region);
}
