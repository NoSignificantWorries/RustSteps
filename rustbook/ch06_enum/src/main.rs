#[derive(Debug)]
enum Point {
    P2 { x: f64, y: f64 },
    P3 { x: f64, y: f64, z: f64 },
}

impl Point {
    fn len(&self) -> f64 {
        match self {
            Point::P2 { x, y } => x * x + y * y,
            Point::P3 { x, y, z } => x * x + y * y + z * z,
        }
    }
}

fn main() {
    let p1 = Point::P2 { x: 12.0, y: -4.0 };
    let p2 = Point::P3 {
        x: 12.0,
        y: -4.0,
        z: 5.1,
    };

    println!("{}", p1.len());
    println!("{p1:?}");
    println!("{}", p2.len());
    println!("{p2:?}");
}
