enum WineGrapes {
    CabernetSauvignon,
    Tanat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetSauvignon => println!("This is a Cabernet Sauvignon"),
        // WineGrapes::Tanat => println!("This is a Tanat"),
        // WineGrapes::Merlot => println!("This is a Merlot"),
        _ => println!("I don't know this wine"),
    }
}

pub fn run() {
    taste_wine(WineGrapes::Tanat);
}
