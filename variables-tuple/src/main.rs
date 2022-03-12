fn main() {
    let tuple: (u8, bool, f32) = (5, true, 3.142);
    //print structure of array and other object
    println!("first:{}, second:{}, third:{}", tuple.0, tuple.1, tuple.2);

    let tuple_2 = ("hello", 31, "xord", false);
    println!("tuple_2 {:?}", tuple_2);

    //destructing
    let (greeting, id, org, is_bad) = tuple_2;

    println!(
        "greeting:{}, id:{}, organization:{}, isBad:{}",
        greeting, id, org, is_bad
    );
}
