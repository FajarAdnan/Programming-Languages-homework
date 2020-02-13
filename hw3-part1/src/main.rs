use std::io;

// Main function that calls the calculating functions
fn main() {
    // Get user input for a
    let mut a = String::new(); // make a mutable string variable
    println!("Enter a.");
    io::stdin().read_line(&mut a) // Get a from the user
               .ok()
               .expect("Incorrect input"); // Error handling
    let a: i32 = a.trim().parse().unwrap(); // Convert string into int

    // Get user input for b
    let mut b = String::new(); // make a mutable string variable
    println!("Enter b.");
    io::stdin().read_line(&mut b) // Get b from the user
               .ok()
               .expect("Incorrect input"); // Error handling
    let b: i32 = b.trim().parse().unwrap(); // Convert string into int

    // Call the function
    sum_of_squares(a, b);
}

/*
// Function to square a number ... Changed into lambda notation within the sum_of_squares function
fn square(x: i32) -> i32{
    let x = x * x;
    return x;
}
*/

// Main function with all the code in it. Takes two inputs: a, b
fn sum_of_squares(a: i32, b: i32) {

    // ****************Square a number (LAMBDA NOTATION)*****************
    let squared = |x: i32| x * x;

    // If a is greater than b, the function ends
    if a > b{
        println!("There is no sum of squares available for your selection of {} and {}.", a, b);
    }
    else{
        // Used to record the sum as we calculate
        let mut sum_of_squares_answer = 0;
        // Sets the beginning number. We are calculating between a and b.
        let mut counter = a+1;
        while counter < b{
            sum_of_squares_answer = sum_of_squares_answer + squared(counter);
            counter = counter + 1;
        }
        println!("The sum of squares between {} and {} is {}.", a, b, sum_of_squares_answer);
    }
}
