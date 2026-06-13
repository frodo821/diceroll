use crate::{
    expression::{BinaryOperator, Comparator, Expression, Statement},
    tokenize::{SpanedToken, Token},
};

pub struct Parser {
    tokens: Vec<SpanedToken>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpanedToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<SpanedToken> {
        self.tokens.get(self.position).cloned()
    }

    fn eos(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    pub fn parse(&mut self) -> Result<Statement, String> {
        let left = self.expect_expression()?;
        if self.eos() {
            return Ok(Statement::Expression(left));
        }

        let comparator_token = self.peek().unwrap();
        let comparator = match &comparator_token.token {
            Token::Equal => Comparator::Equal,
            Token::NotEqual => Comparator::NotEqual,
            Token::Less => Comparator::Less,
            Token::LessEqual => Comparator::LessEqual,
            Token::Greater => Comparator::Greater,
            Token::GreaterEqual => Comparator::GreaterEqual,
            _ => return Err("Expected a comparator".to_string()),
        };
        self.advance(1);

        let right = self.expect_expression()?;

        if !self.eos() {
            return Err("Unexpected tokens after expression".to_string());
        }

        Ok(Statement::Comparison {
            left,
            comparator,
            right,
        })
    }

    // expression := term (("+" | "-") term)*
    fn expect_expression(&mut self) -> Result<Expression, String> {
        let mut left = self.expect_term()?;

        while let Some(token) = self.peek() {
            let op = match &token.token {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };
            self.advance(1);
            let right = self.expect_term()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // term := factor (("*" | "/") factor)*
    fn expect_term(&mut self) -> Result<Expression, String> {
        let mut left = self.expect_factor()?;

        while let Some(token) = self.peek() {
            let op = match &token.token {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                _ => break,
            };
            self.advance(1);
            let right = self.expect_factor()?;
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // factor := number "d" number | number | "(" expression ")"
    fn expect_factor(&mut self) -> Result<Expression, String> {
        let token = self
            .peek()
            .ok_or_else(|| "Unexpected end of input".to_string())?;

        match token.token {
            Token::Number(count) => {
                self.advance(1);

                if let Some(next) = self.peek() {
                    if let Token::Dice = next.token {
                        self.advance(1);
                        if let Some(sides_token) = self.peek() {
                            if let Token::Number(sides) = sides_token.token {
                                self.advance(1);
                                return Ok(Expression::Dice { count, sides });
                            }
                        }
                        return Err("Expected number of sides after 'd'".to_string());
                    }
                }

                Ok(Expression::Number(count))
            }
            Token::LParen => {
                self.advance(1);
                let expr = self.expect_expression()?;

                match self.peek() {
                    Some(SpanedToken {
                        token: Token::RParen,
                        ..
                    }) => {
                        self.advance(1);
                        Ok(Expression::Parens(Box::new(expr)))
                    }
                    _ => Err("Expected closing parenthesis".to_string()),
                }
            }
            _ => Err("Expected a number, dice notation, or parenthesized expression".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::Tokenizer;

    fn parse(input: &str) -> Result<Statement, String> {
        let tokens = Tokenizer::new(input.to_string()).tokenize();
        Parser::new(tokens).parse()
    }

    fn num(n: i32) -> Expression {
        Expression::Number(n)
    }

    fn binop(left: Expression, op: BinaryOperator, right: Expression) -> Expression {
        Expression::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    #[test]
    fn parses_single_number() {
        assert_eq!(parse("3"), Ok(Statement::Expression(num(3))));
    }

    #[test]
    fn parses_dice_notation() {
        assert_eq!(
            parse("2d6"),
            Ok(Statement::Expression(Expression::Dice {
                count: 2,
                sides: 6
            }))
        );
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        assert_eq!(
            parse("1+2*3"),
            Ok(Statement::Expression(binop(
                num(1),
                BinaryOperator::Add,
                binop(num(2), BinaryOperator::Multiply, num(3)),
            )))
        );
        assert_eq!(
            parse("2*3+1"),
            Ok(Statement::Expression(binop(
                binop(num(2), BinaryOperator::Multiply, num(3)),
                BinaryOperator::Add,
                num(1),
            )))
        );
    }

    #[test]
    fn same_precedence_is_left_associative() {
        assert_eq!(
            parse("1-2-3"),
            Ok(Statement::Expression(binop(
                binop(num(1), BinaryOperator::Subtract, num(2)),
                BinaryOperator::Subtract,
                num(3),
            )))
        );
    }

    #[test]
    fn parens_override_precedence() {
        assert_eq!(
            parse("(1+2)*3"),
            Ok(Statement::Expression(binop(
                Expression::Parens(Box::new(binop(num(1), BinaryOperator::Add, num(2)))),
                BinaryOperator::Multiply,
                num(3),
            )))
        );
    }

    #[test]
    fn parses_dice_in_arithmetic() {
        assert_eq!(
            parse("2d6+3"),
            Ok(Statement::Expression(binop(
                Expression::Dice { count: 2, sides: 6 },
                BinaryOperator::Add,
                num(3),
            )))
        );
    }

    #[test]
    fn parses_comparison_statement() {
        assert_eq!(
            parse("1d20>=15"),
            Ok(Statement::Comparison {
                left: Expression::Dice {
                    count: 1,
                    sides: 20
                },
                comparator: Comparator::GreaterEqual,
                right: num(15),
            })
        );
    }

    #[test]
    fn parses_nested_parens() {
        assert_eq!(
            parse("((5))"),
            Ok(Statement::Expression(Expression::Parens(Box::new(
                Expression::Parens(Box::new(num(5)))
            ))))
        );
    }

    #[test]
    fn rejects_empty_input() {
        assert!(parse("").is_err());
    }

    #[test]
    fn rejects_dice_without_sides() {
        assert!(parse("2d").is_err());
        assert!(parse("2d+1").is_err());
    }

    #[test]
    fn rejects_dangling_operator() {
        assert!(parse("1+").is_err());
        assert!(parse("+1").is_err());
    }

    #[test]
    fn rejects_unclosed_paren() {
        assert!(parse("(1+2").is_err());
    }

    #[test]
    fn rejects_comparator_without_right_side() {
        assert!(parse("1d20>=").is_err());
    }

    #[test]
    fn rejects_trailing_tokens() {
        assert!(parse("1 2").is_err());
        assert!(parse("1>2>3").is_err());
    }
}
