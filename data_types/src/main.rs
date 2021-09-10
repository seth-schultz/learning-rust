fn main() {

    let x = 2.0; // f64, Rust defaults to f64 because it is roughly the same speed on modern CPU

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuples
    // data residing in tuples can have different data types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // tuple index 0

    let six_point_four = x.1; // tuple index 1

    let one = x.2; // tuple index 2

    // arrays
    // unlike tuples, all data in an array must have the same data type
    // arrays are allocated on the stack rather than the heap
    // arrays have a fixed size compared to a vector which can grow and shrink
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // every item in the array will have the value of 3 (i.e. let a = [3, 3, 3, 3, 3];)

    let first = a[0]; //get first entry
    let second = a[1]; //get second entry
}
