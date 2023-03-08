fn main() {
    let mut x = 5;
    println!("The values of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// -------
// Chapter 3 Takeaways

// -------
// Numeric Operations
// support for:
// addition (+)
// subtraction (-)
// multiplication (*)
// division (/)
// remainder (%)

// -------
// Boolean
// let t = true;
// let f: bool = false; // explicit type annotation

// -------
// Character Type
// Important bit of info here that character type is single quotes
// let c = 'c';
// let z = 'z';

// -------
// Tuple Type
// General way to group together values with a variety of types
// let tup: (i32, f64, u8) = (500, 6.4, 1);

// can also destructure tuple
// let tup = (500, 6.4, 1);
// let (x, y, z) = tup;
// println!("The value of y is: {}", y);

// can also access tuple element by using a period (.) with index
// let x: (i32, f64, u8) = (500, 6.4, 1);
// let five_hundred = x.0;
// let six_point_four = x.1;
// let one = x.2;

// -------
// Array Type
// Arrays should only be used when you have a fixed number of elements
// let a = [1, 2, 3, 4, 5];
// let months = ["Jan", "Feb", "Mar", "etc"];
// otherwise use a vector type

// array's type in square brackets when needed
// let a: [i32; 5] = [1, 2, 3, 4, 5];
// 5 after semi-colon indicates that the element contains five items

// also can init array with
// let a = [3; 5];
// which is the same as
// let a = [3, 3, 3, 3, 3];

