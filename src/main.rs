fn main() {
    let mut x: i32 = 4; // add mut so it can be resigned
    println!("x is: {}", x); // x is: 4
    x = 5;
    println!("x is: {}", x); // x is: 5

    let z:i32 = 7;
    println!("z is: {}", z); // 7
    let z:i32 = 9;
    println!("z is: {}", z); // 9

    let v:i32 = 7;
    println!("v is: {}", v); // 7
    let v:i32 = v + 1;
    println!("v is: {}", v) // 8
}
