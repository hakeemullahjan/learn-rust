fn main() {
    let _is_even: bool = is_even(65535);
    println!("is even: {}", _is_even);
}

fn is_even(num: u16) -> bool {
    let digit: u16 = num % 2;
    digit == 0 //return bool (not include semicolon)
}
