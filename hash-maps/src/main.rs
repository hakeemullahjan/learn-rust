use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "hi");
    map.insert(1, "hello");
    map.insert(3, "how");

    println!("map {:?}", map);

    match map.get(&1) {
        Some(strr) => println!("{}", strr),
        _ => println!("Does not exist in map"),
    }

    match map.get(&4) {
        Some(strr) => println!("{}", strr),
        None => println!("Does not exist in map"),
    }

    map.remove(&1);

    println!("{:?}", map)
}
