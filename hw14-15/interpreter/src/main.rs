use std::collections::HashMap;

use interpreter::expression::*;
use interpreter::environment::*;
use interpreter::evaluator::*;
use interpreter::lexer::*;
use interpreter::parser::*;

fn main() {

    let environment = Environment::new();
    let expression_string = "(* 2 2 (+ 2 2 (* 1 2)))"; // Expected output: 24
    let value = evaluate(&environment, &generate_expression(&mut tokenize(expression_string)));
    println!("We computed {}", value)
}
