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
    println!("v is: {}", v); // 8

    //scope

    { // x is only 2 inside this scope
        let x: i32 = 2;
        println!("x is {}", x ); // 2
        let v: i32 = v + 2; // can get variables from outside the scope
        println!("x is {}", v ); //10
    }
    println!("x is {}", x ); // 5

    {
        let x = 4;
        let x = "hello";
        // work because its redefined

        //x = "hello"; does not work is not redefined only value and this is nit a number
    }

    {
        // const syntax big letters
        // const cannot be reassigned
        const SECONDS_IN_MINUTES: u32 = 60;
        println!("{}", SECONDS_IN_MINUTES)
    }
}
