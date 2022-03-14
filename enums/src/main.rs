fn main() {
    let a: MyEnum = MyEnum::A;
    println!("a {:?}", a);
    let b: MyEnum = MyEnum::B(12);
    println!("b {:?}", b);
    let c: MyEnum = MyEnum::C { x: 10, y: -200000 };
    println!("c {:?}", c);

    if let MyEnum::B(ali) = b {
        println!("{}", ali);
    }

    if let MyEnum::C { x, y } = c {
        println!("x {} y {}", x, y);
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C { x: i32, y: i32 },
}
