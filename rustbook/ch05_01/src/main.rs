#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn len(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

fn main() {
    let mut p1 = Point {
        x: 3.,
        y: 4.,
        z: 5.,
    };

    p1.y = 10.0;

    println!("{} {} {}", p1.x, p1.y, p1.z);

    let p2 = Point { z: 15., ..p1 };

    println!("{} {} {}", p2.x, p2.y, p2.z);
    println!("p2 is {p2:?}");

    println!("len of p2 = {}", p2.len())
}
