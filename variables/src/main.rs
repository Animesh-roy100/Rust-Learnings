fn main() {
    // println!("Hello, world!");
    // let bunnies = 2;
    // let bunnies: i32 = 4;
    // let (bunnies, carrots) = (8, 50);
    // println!("{} {}",bunnies, carrots)
    let mut x: i8 = 2;
    println!("the value of x: {}", x);

    x = 3;
    println!("the value of x: {}", x);

    let y = (10, 20, 30);
    println!("the value of first num in y: {}", y.0);

    const WRAP_FACTOR: f64 = 9.9;
    println!("the value of wrap_factor: {}", WRAP_FACTOR);
}