use crate::position::{Position, Span};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Dice,
    LParen,
    RParen,

    // Binary operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Comparison operators
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Clone, Debug)]
pub struct SpanedToken {
    pub token: Token,
    pub span: Span,
}

impl SpanedToken {
    pub fn new(token: Token, start: Position, end: Position) -> Self {
        Self {
            token,
            span: Span::new(start, end),
        }
    }
}

#[derive(Clone)]
pub struct Tokenizer {
    input: String,
    position: Position,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: Position::new("input".to_string(), 0),
        }
    }

    pub fn tokenize(&mut self) -> Vec<SpanedToken> {
        let mut tokens: Vec<SpanedToken> = Vec::new();

        while self.position.index() < self.input.len() {
            let current_char = self.input.chars().nth(self.position.index()).unwrap();

            if current_char.is_whitespace() {
                self.position.advance(1);
                continue;
            }

            if current_char.is_digit(10) {
                let start_pos = self.position.clone();
                while self.position.index() < self.input.len()
                    && self
                        .input
                        .chars()
                        .nth(self.position.index())
                        .unwrap()
                        .is_digit(10)
                {
                    self.position.advance(1);
                }
                let number_str = &self.input[start_pos.index()..self.position.index()];
                let number = number_str.parse::<i32>().unwrap();
                tokens.push(SpanedToken {
                    token: Token::Number(number),
                    span: Span::new(start_pos, self.position.clone()),
                });
                continue;
            }

            match current_char {
                '+' => {
                    tokens.push(SpanedToken::new(
                        Token::Plus,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                '-' => {
                    tokens.push(SpanedToken::new(
                        Token::Minus,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                '*' => {
                    tokens.push(SpanedToken::new(
                        Token::Multiply,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                '/' => {
                    tokens.push(SpanedToken::new(
                        Token::Divide,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                '(' => {
                    tokens.push(SpanedToken::new(
                        Token::LParen,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                ')' => {
                    tokens.push(SpanedToken::new(
                        Token::RParen,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                'd' | 'D' => {
                    tokens.push(SpanedToken::new(
                        Token::Dice,
                        self.position.clone(),
                        self.position.clone(),
                    ));
                    self.position.advance(1);
                }
                '=' => {
                    let next = self.input.chars().nth(self.position.index() + 1);

                    if next == Some('=') {
                        tokens.push(SpanedToken::new(
                            Token::Equal,
                            self.position.clone(),
                            self.position.clone(),
                        ));
                        self.position.advance(2);
                    } else {
                        panic!(
                            "Unexpected character '{}' at {}",
                            current_char,
                            self.position.format(&self.input)
                        );
                    }
                }
                '!' => {
                    let next = self.input.chars().nth(self.position.index() + 1);

                    if next == Some('=') {
                        tokens.push(SpanedToken::new(
                            Token::NotEqual,
                            self.position.clone(),
                            self.position.clone(),
                        ));
                        self.position.advance(2);
                    } else {
                        panic!(
                            "Unexpected character '{}' at {}",
                            current_char,
                            self.position.format(&self.input)
                        );
                    }
                }
                '<' => {
                    let next = self.input.chars().nth(self.position.index() + 1);

                    if next == Some('=') {
                        tokens.push(SpanedToken::new(
                            Token::LessEqual,
                            self.position.clone(),
                            self.position.clone(),
                        ));
                        self.position.advance(2);
                    } else {
                        tokens.push(SpanedToken::new(
                            Token::Less,
                            self.position.clone(),
                            self.position.clone(),
                        ));
                        self.position.advance(1);
                    }
                }
                '>' => {
                    let next = self.input.chars().nth(self.position.index() + 1);

                    if next == Some('=') {
                        tokens.push(SpanedToken::new(
                            Token::GreaterEqual,
                            self.position.clone(),
                            self.position.clone(),
                        ));
                        self.position.advance(2);
                    } else {
                        tokens.push(SpanedToken::new(
                            Token::Greater,
                            self.position.clone(),
                            self.position.clone(),
                        ));
                        self.position.advance(1);
                    }
                }
                _ => panic!(
                    "Unexpected character '{}' at {}",
                    current_char,
                    self.position.format(&self.input)
                ),
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(input: &str) -> Vec<Token> {
        Tokenizer::new(input.to_string())
            .tokenize()
            .into_iter()
            .map(|t| t.token)
            .collect()
    }

    #[test]
    fn tokenizes_dice_expression() {
        assert_eq!(
            tokens("2d6+3"),
            vec![
                Token::Number(2),
                Token::Dice,
                Token::Number(6),
                Token::Plus,
                Token::Number(3),
            ]
        );
    }

    #[test]
    fn tokenizes_uppercase_dice() {
        assert_eq!(
            tokens("1D20"),
            vec![Token::Number(1), Token::Dice, Token::Number(20)]
        );
    }

    #[test]
    fn tokenizes_multi_digit_numbers() {
        assert_eq!(tokens("123"), vec![Token::Number(123)]);
    }

    #[test]
    fn skips_whitespace() {
        assert_eq!(
            tokens("  1 +\t2 "),
            vec![Token::Number(1), Token::Plus, Token::Number(2)]
        );
    }

    #[test]
    fn tokenizes_arithmetic_operators_and_parens() {
        assert_eq!(
            tokens("(1-2)*3/4"),
            vec![
                Token::LParen,
                Token::Number(1),
                Token::Minus,
                Token::Number(2),
                Token::RParen,
                Token::Multiply,
                Token::Number(3),
                Token::Divide,
                Token::Number(4),
            ]
        );
    }

    #[test]
    fn tokenizes_comparators() {
        assert_eq!(
            tokens("1==2"),
            vec![Token::Number(1), Token::Equal, Token::Number(2)]
        );
        assert_eq!(
            tokens("1!=2"),
            vec![Token::Number(1), Token::NotEqual, Token::Number(2)]
        );
        assert_eq!(
            tokens("1<2"),
            vec![Token::Number(1), Token::Less, Token::Number(2)]
        );
        assert_eq!(
            tokens("1<=2"),
            vec![Token::Number(1), Token::LessEqual, Token::Number(2)]
        );
        assert_eq!(
            tokens("1>2"),
            vec![Token::Number(1), Token::Greater, Token::Number(2)]
        );
        assert_eq!(
            tokens("1>=2"),
            vec![Token::Number(1), Token::GreaterEqual, Token::Number(2)]
        );
    }

    #[test]
    fn trailing_single_char_comparator_does_not_panic() {
        assert_eq!(tokens("1<"), vec![Token::Number(1), Token::Less]);
        assert_eq!(tokens("1>"), vec![Token::Number(1), Token::Greater]);
    }

    #[test]
    #[should_panic(expected = "Unexpected character")]
    fn rejects_unknown_character() {
        tokens("1@2");
    }

    #[test]
    #[should_panic]
    fn rejects_lone_equal() {
        tokens("1=2");
    }

    #[test]
    fn empty_input_yields_no_tokens() {
        assert_eq!(tokens(""), vec![]);
    }
}
