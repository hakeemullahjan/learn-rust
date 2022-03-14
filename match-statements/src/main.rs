fn main() {
    let i = 3;

    match i {
        0 => println!("0"),
        1 | 2 | 3 => println!("1,2,3"),
        4..=9 => println!("4..9"),
        _ => println!("default"),
    }
}
