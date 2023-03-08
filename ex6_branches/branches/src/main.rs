// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

fn main() {
    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

// note that if the values of both the if arm and else arm expression
// are mismatched (ie not i32 integers like the above example), we'll
// get an error. in short, variables must have a single type.
