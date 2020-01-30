fn main() {
    sum_of_squares(5);
}

fn sum_of_squares(x: i32) {
    let left = x + 1;
    let right = x * 2;

    let square_left = left * left;
    let square_right = right * right;

    let sum_of_squares_answer = square_left + square_right;

    println!("The sum of squares is {}.", sum_of_squares_answer);
}
