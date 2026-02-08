fn main() {
    // scope for the vector and other object
    {
        let mut v: Vec<i32> = Vec::new();

        v.push(1);
        v.push(2);
        v.push(3);

        println!("{v:?}");
    }
    // v deallocated when it goes out of scope

    let v2 = vec![4, 5, 6];

    // unsafe
    // let third: &i32 = &v2[7];
    // println!("{third}");

    // safe
    let elem: Option<&i32> = v2.get(2);
    match elem {
        Some(elem) => println!("{elem}"),
        None => println!("No item!"),
    }
}
