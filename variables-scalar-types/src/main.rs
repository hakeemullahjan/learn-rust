fn main() {
    // unsigned integer
    // u8, u16, u32, u64, u128
    let _unsigned_integer: u8 = 242 + 13; //0-255

    // signed integer
    // i8, i16, i32, i64, i128
    let _signed_integer: i8 = -12; //-128-127

    // float is used for decimals
    // f32, f64
    let mut _float: f32 = -2242424.234;
    _float = 123.343 + 100.2;

    println!("unsigned integer: {}", _unsigned_integer);
    println!("signed integer: {}", _signed_integer);
    println!("float: {}", _float);

    // char - can only be
    let _letter = "Hakeemullah";
    let _emoji = "\u{1F600}";

    println!("letter {}", _letter);
    println!("emoji {}", _emoji);

    //bool
    let _is_true: bool = true && false || false;

    println!("isTrue {}", _is_true);
}
