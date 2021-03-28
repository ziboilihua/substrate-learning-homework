pub enum Shape {
    Circle{ r: f64 },
    Rectangle{ width: f64, height: f64},
    Triangle{ a: f64, b: f64, c:f64 },
}

pub trait Area {
    fn get_area (&self) -> f64;
}

impl Area for Shape {
    fn get_area(&self) -> f64 {
        match *self {
            Shape::Circle{r} => r * r * 3.14,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                let p = (a + b + c) / 2.0;
                (p * (p - a) * (p - b) * (p - c)).sqrt()
            }
        }
    }
}