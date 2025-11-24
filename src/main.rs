fn add(x: i32, y: i32) -> i32 {
     return x + y;
}


fn main() {

    let x = 5;
    let mut y = 6;
    y = add(x, y);
    println!("x: {}", x);
    println!("y: {}", y);
}
