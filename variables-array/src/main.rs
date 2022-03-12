fn main() {
    let arr: [u8; 3] = [10, 20, 30];
    println!("index {}", arr[0]);

    let other_array: [u8; 5] = [100; 5];
    print!(
        "other_array length {} \nother_array {:?}",
        other_array.len(),
        other_array
    );

}
