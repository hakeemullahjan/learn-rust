use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // start from zero and 6 is excluded!
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
    for i in 0..100000000 {
        // println!("i->  {}", i);
    }
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);
}
