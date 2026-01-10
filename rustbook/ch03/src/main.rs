fn main() {
    'l1: for number in (1..4).rev() {
        println!("{number}!");
        if number < 3 {
            break 'l1;
        }
    }
    println!("LIFTOFF!!!");
}
