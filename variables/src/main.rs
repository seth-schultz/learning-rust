fn main() {
    // indicate x is mutable (changable)
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1; // shadow x and add 1 to the original value, for a value of 6

    let x = x * 2; // shadow x again and multiply the shadowed version by 2, for a value of 12

    println!("The value of x is: {}", x);

    let spaces = "   ";

    // the difference between mut and shadowing is that because we're effectively creating a new variable when we use the let keyword
    // we can change the type of the value but reuse the same name
    let spaces = spaces.len();

    println!("Spaces has {} spaces", spaces);
}