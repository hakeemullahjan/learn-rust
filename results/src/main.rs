
#[derive(Debug)]
enum MyError {
    Error1,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let divide1 = divide(21, 31);
    // let res = divide1.expect("we crashed");
    // println!("res {}", res);

    // match divide1 {
    //     Ok(v) => println!("v {}", v),
    //     Err(e) => println!("err {:?}", e),
    // }

    // if divide1.is_ok(){
    //     println!("is OKK {}", divide1.unwrap())
    // }

    // println!("{}", divide1.unwrap())
    println!("{}", divide1.unwrap_or(100))
}
