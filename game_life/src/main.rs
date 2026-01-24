#[derive(Debug)]
struct Field<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Field<T> {
    fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Field {
            width,
            height,
            cells: vec![default; width * height],
        }
    }
}

fn main() {
    let field = Field::new(10, 10, false);

    println!("{field:?}");
}
