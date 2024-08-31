enum Shape {
    Circle(f64),
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(x) => std::f64::consts::PI * x * x,
            Shape::Square(x) => x * x,
        }
    }
}

pub fn run() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0)];

    let total_area: f64 = shapes.iter().map(|shape| shape.area()).sum();

    // let total_area: f64 = shapes
    //     .iter()
    //     .map(|shape| match shape {
    //         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
    //         Shape::Square(side) => side * side,
    //     })
    //     .sum();

    // println!("Total area: {:3}", total_area);
    // let mut total_area: f64 = 0.0;

    // for shape in shapes {
    //     match shape {
    //         Shape::Circle(x) => total_area += std::f64::consts::PI * x * x,
    //         Shape::Square(x) => total_area += x * x,
    //     }
    // }

    println!("Total area: {:.3}", total_area);
}
