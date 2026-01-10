fn main() {
    let s1 = String::from("Test1");

    let l1 = calculate_length(&s1);

    println!("s=\"{s1}\" l={l1}");
}

fn calculate_length(line: &String) -> usize {
    line.len()
}
