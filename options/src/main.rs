fn main() {
    println!("H ello, world!");
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    //Unwrapping the 'Some' variant will extract the value wrapped
    // println!("{:?} wraps to {}", divide1, divide1.unwrap());

    //Unwrapping the 'None' variant will extract the value wrapped
    println!("{:?} wraps to {}", divide1, divide2.unwrap())
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}
