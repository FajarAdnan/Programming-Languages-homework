use std::collections::HashMap;

use interpreter::expression::*;
use interpreter::environment::*;
use interpreter::evaluator::*;

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];
        // act
        let sum = evaluate_addition(&Environment::new(), &Expression::Add(values));
        // assert
        assert_eq!(4.0, sum);
    }
    #[test]
    fn basic_subtraction() {
        // arrange
        let values = vec![Expression::Number(4.0), Expression::Number(2.0)];
        // act
        let remainder = evaluate_subtraction(&Environment::new(), &Expression::Subtract(values));
        // assert
        assert_eq!(2.0, remainder);
    }
    #[test]
    fn basic_multiplication() {
        // arrange
        let values = vec![Expression::Number(2.0), Expression::Number(2.0)];
        // act
        let product = evaluate_multiplication(&Environment::new(), &Expression::Multiply(values));
        // assert
        assert_eq!(4.0, product);
    }

    #[test]
    fn basic_environment_variable() {
        // arrange
        let mut environment = Environment::new();
        environment.define(String::from("x"), Expression::Number(5.0));
        let values = vec![Expression::Variable(String::from("x")),
                            Expression::Number(2.0)];
        let addition = Expression::Add(values);
        // act
        let sum = evaluate(&environment, &addition);
        // assert
        assert_eq!(7.0, sum);
    }
    #[test]
    fn two_environment_variables() {
        // arrange
        let mut environment = Environment::new();
        environment.define(String::from("x"), Expression::Number(5.0));
        environment.define(String::from("y"), Expression::Number(5.0));
        let values = vec![Expression::Variable(String::from("x")),
                            Expression::Variable(String::from("y"))];
        let addition = Expression::Add(values);
        // act
        let sum = evaluate(&environment, &addition);
        // assert
        assert_eq!(10.0, sum);
    }
    #[test]
    fn two_environment_variables_mult_impl_on_expression() {
        // arrange
        let mut environment = Environment::new();
        environment.define(String::from("x"), Expression::Number(5.0));
        environment.define(String::from("y"), Expression::Number(5.0));
        let values = vec![Expression::Variable(String::from("x")),
                            Expression::Variable(String::from("y"))];
        let addition = Expression::Multiply(values);
        // act
        let sum = evaluate(&environment, &addition);
        // assert
        assert_eq!(25.0, sum);
    }
}
