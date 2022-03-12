fn main() {
    let _str: &str = "hello world!";
    println!("_strr-> {}", _str);

    let mut _string: String = String::from("Hello ");

    println!("_string-> {}", _string);
    print!("length {}", _string.len());

    _string.push_str("hakeemullah");
    _string.push('!');
    _string = _string.replace("Hello", "Bye");
    println!("_string-> {}", _string);
}
