#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Comparator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl Comparator {
    pub fn format(&self) -> &'static str {
        match self {
            Comparator::Equal => "==",
            Comparator::NotEqual => "!=",
            Comparator::Less => "<",
            Comparator::LessEqual => "<=",
            Comparator::Greater => ">",
            Comparator::GreaterEqual => ">=",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    Comparison {
        left: Expression,
        comparator: Comparator,
        right: Expression,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Number(i32),
    Dice {
        count: i32,
        sides: i32,
    },
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Parens(Box<Expression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvaluatedValue {
    Number(i32),
    Comparison(i32, i32, Comparator, bool),
}

impl Statement {
    pub fn evaluate(&self) -> EvaluatedValue {
        match self {
            Statement::Expression(expr) => EvaluatedValue::Number(expr.evaluate()),
            Statement::Comparison {
                left,
                comparator,
                right,
            } => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                match comparator {
                    Comparator::Equal => EvaluatedValue::Comparison(
                        left_val,
                        right_val,
                        comparator.clone(),
                        left_val == right_val,
                    ),
                    Comparator::NotEqual => EvaluatedValue::Comparison(
                        left_val,
                        right_val,
                        comparator.clone(),
                        left_val != right_val,
                    ),
                    Comparator::Less => EvaluatedValue::Comparison(
                        left_val,
                        right_val,
                        comparator.clone(),
                        left_val < right_val,
                    ),
                    Comparator::LessEqual => EvaluatedValue::Comparison(
                        left_val,
                        right_val,
                        comparator.clone(),
                        left_val <= right_val,
                    ),
                    Comparator::Greater => EvaluatedValue::Comparison(
                        left_val,
                        right_val,
                        comparator.clone(),
                        left_val > right_val,
                    ),
                    Comparator::GreaterEqual => EvaluatedValue::Comparison(
                        left_val,
                        right_val,
                        comparator.clone(),
                        left_val >= right_val,
                    ),
                }
            }
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expression::Number(n) => *n,
            Expression::Dice { count, sides } => {
                let mut total = 0;
                for _ in 0..*count {
                    total += rand::random_range(1i32..=*sides);
                }
                total
            }
            Expression::BinaryOp { left, op, right } => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                match op {
                    BinaryOperator::Add => left_val + right_val,
                    BinaryOperator::Subtract => left_val - right_val,
                    BinaryOperator::Multiply => left_val * right_val,
                    BinaryOperator::Divide => left_val / right_val,
                }
            }
            Expression::Parens(expr) => expr.evaluate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(input: &str) -> EvaluatedValue {
        let tokens = crate::tokenize::Tokenizer::new(input.to_string()).tokenize();
        crate::parser::Parser::new(tokens)
            .parse()
            .expect("parse should succeed")
            .evaluate()
    }

    fn evaluate_number(input: &str) -> i32 {
        match evaluate(input) {
            EvaluatedValue::Number(n) => n,
            other => panic!("expected a number, got {:?}", other),
        }
    }

    #[test]
    fn evaluates_arithmetic() {
        assert_eq!(evaluate_number("3"), 3);
        assert_eq!(evaluate_number("1+2*3"), 7);
        assert_eq!(evaluate_number("(1+2)*3"), 9);
        assert_eq!(evaluate_number("10-2-3"), 5);
    }

    #[test]
    fn division_truncates_toward_zero() {
        assert_eq!(evaluate_number("7/2"), 3);
    }

    #[test]
    fn evaluates_negative_results() {
        assert_eq!(evaluate_number("2-5"), -3);
    }

    #[test]
    fn dice_roll_stays_within_bounds() {
        for _ in 0..200 {
            let value = evaluate_number("3d6");
            assert!((3..=18).contains(&value), "3d6 rolled {}", value);
        }
    }

    #[test]
    fn single_die_covers_full_range() {
        let mut seen = std::collections::HashSet::new();
        for _ in 0..1000 {
            seen.insert(evaluate_number("1d4"));
        }
        assert_eq!(seen, (1..=4).collect());
    }

    #[test]
    fn zero_dice_evaluate_to_zero() {
        assert_eq!(evaluate_number("0d6"), 0);
    }

    #[test]
    fn dice_combine_with_arithmetic() {
        for _ in 0..100 {
            let value = evaluate_number("2d6+3");
            assert!((5..=15).contains(&value), "2d6+3 rolled {}", value);
        }
    }

    #[test]
    fn evaluates_comparison_outcomes() {
        assert_eq!(
            evaluate("3<5"),
            EvaluatedValue::Comparison(3, 5, Comparator::Less, true)
        );
        assert_eq!(
            evaluate("5<3"),
            EvaluatedValue::Comparison(5, 3, Comparator::Less, false)
        );
        assert_eq!(
            evaluate("4==4"),
            EvaluatedValue::Comparison(4, 4, Comparator::Equal, true)
        );
        assert_eq!(
            evaluate("4!=4"),
            EvaluatedValue::Comparison(4, 4, Comparator::NotEqual, false)
        );
        assert_eq!(
            evaluate("4<=4"),
            EvaluatedValue::Comparison(4, 4, Comparator::LessEqual, true)
        );
        assert_eq!(
            evaluate("4>=5"),
            EvaluatedValue::Comparison(4, 5, Comparator::GreaterEqual, false)
        );
        assert_eq!(
            evaluate("5>4"),
            EvaluatedValue::Comparison(5, 4, Comparator::Greater, true)
        );
    }

    #[test]
    fn comparison_evaluates_both_sides() {
        assert_eq!(
            evaluate("1+2==4-1"),
            EvaluatedValue::Comparison(3, 3, Comparator::Equal, true)
        );
    }

    #[test]
    fn comparator_format_round_trips() {
        assert_eq!(Comparator::Equal.format(), "==");
        assert_eq!(Comparator::NotEqual.format(), "!=");
        assert_eq!(Comparator::Less.format(), "<");
        assert_eq!(Comparator::LessEqual.format(), "<=");
        assert_eq!(Comparator::Greater.format(), ">");
        assert_eq!(Comparator::GreaterEqual.format(), ">=");
    }
}
