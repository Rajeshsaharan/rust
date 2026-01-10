fn main() {
    assert_eq!(4, pow(2,2));
    assert_eq!(27,pow(3,3));
}

// i32 , i32 -> i32
// take two input parameter and produce x power of y
fn pow(x: i32, y: i32)->i32 {
    if y == 0 {
        1
    } else {
        x * pow(x, y - 1)
    }
}
