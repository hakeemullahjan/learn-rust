fn main() {
    let mut vecc: Vec<i64> = vec![10, 20, 30, 40, 50];

    println!("vector {:?}", vecc);
    println!("indexing {}", vecc[1]);
    println!("length {}", vecc.len());

    vecc.push(60);

    println!("after push {:?}", vecc);

    vecc.remove(2);

    println!("after removing {:?}", vecc);
}
