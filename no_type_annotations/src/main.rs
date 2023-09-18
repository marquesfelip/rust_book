fn main() {
    let guess: u32 = "42".parse().expect("Not a number");

    // Floating-pointing type
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //Char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let tupla: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tupla.0;

    let six_point_four = tupla.1;

    let one = tupla.2;

    // Array
    let a = [1, 2, 3, 4, 5];

    let array = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1]; 
}
