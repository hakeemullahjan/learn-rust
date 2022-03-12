fn main() {
    let _arr = [10, 20, 30, 40];
    let _slice = &_arr[1..3]; //index 1 included and 3 excluded

    // println!("array {:?}", _arr);
    // println!("slice {:?}", _slice);

    borrowing_slice(_arr, _slice);
}

fn borrowing_slice(_arr: [u8; 4], _slice: &[u8]) {
    println!("{:?}", _arr);
    println!("{:?}", _slice);
    println!("slice length {}", _slice.len());
    println!("{} {}", _slice[0], _slice[1]);
}
