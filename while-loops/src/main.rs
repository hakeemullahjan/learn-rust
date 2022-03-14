fn main() {
    let mut i = 0;
    while i < 6 {
        println!("i->{}", i);
        i += 1;
        if i == 3 {
            println!("EXIT");
            break;
        }
    }
}
