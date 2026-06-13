pub mod expression;
pub mod parser;
pub mod position;
pub mod tokenize;

use clap::Parser;

use crate::tokenize::Tokenizer;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = r"A simple command-line dice roller and expression evaluator for tabletop RPGs.

Supported features:
- Basic arithmetic: +, -, *, /
- Dice notation: NdM (e.g., 3d6 for rolling three six-sided dice)
- Parentheses for grouping
- Comparison operators: ==, !=, <, >, <=, >=
- Critical and fumble detection based on configurable thresholds
- Optional suppression of critical/fumble output for clean results"
)]
struct Args {
    #[arg(short, long)]
    expression: String,

    #[arg(short, long, default_value_t = 5)]
    crit_fumble_thres: i32,

    #[arg(short = 'G', long, default_value_t = false)]
    crit_fumble_greater_is_better: bool,

    #[arg(short, long, default_value_t = false)]
    without_judge: bool,
}

fn crit_fumble_result(
    value: i32,
    threshold: i32,
    greater_is_better: bool,
    without_judge: bool,
) -> &'static str {
    if without_judge {
        return "";
    }
    if value <= threshold {
        if greater_is_better {
            ": Fumble"
        } else {
            ": Critical"
        }
    } else if value >= 100 - threshold {
        if greater_is_better {
            ": Critical"
        } else {
            ": Fumble"
        }
    } else {
        ""
    }
}

fn main() {
    let args = Args::parse();
    let tokens = Tokenizer::new(args.expression.clone()).tokenize();
    match parser::Parser::new(tokens).parse() {
        Ok(stmt) => {
            let result = stmt.evaluate();

            match result {
                expression::EvaluatedValue::Number(n) => {
                    println!(
                        "{} = {}{}",
                        args.expression,
                        n,
                        crit_fumble_result(
                            n,
                            args.crit_fumble_thres,
                            args.crit_fumble_greater_is_better,
                            args.without_judge
                        )
                    );
                }
                expression::EvaluatedValue::Comparison(left, right, comparator, outcome) => {
                    println!(
                        "Comparison: {} {} {} => {}{}",
                        left,
                        comparator.format(),
                        right,
                        if outcome { "success" } else { "failure" },
                        crit_fumble_result(
                            left,
                            args.crit_fumble_thres,
                            args.crit_fumble_greater_is_better,
                            args.without_judge
                        )
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::crit_fumble_result;

    #[test]
    fn without_judge_suppresses_output() {
        assert_eq!(crit_fumble_result(1, 5, false, true), "");
        assert_eq!(crit_fumble_result(100, 5, true, true), "");
    }

    #[test]
    fn low_roll_is_critical_when_lower_is_better() {
        assert_eq!(crit_fumble_result(5, 5, false, false), ": Critical");
        assert_eq!(crit_fumble_result(1, 5, false, false), ": Critical");
    }

    #[test]
    fn low_roll_is_fumble_when_greater_is_better() {
        assert_eq!(crit_fumble_result(5, 5, true, false), ": Fumble");
    }

    #[test]
    fn high_roll_is_fumble_when_lower_is_better() {
        assert_eq!(crit_fumble_result(95, 5, false, false), ": Fumble");
        assert_eq!(crit_fumble_result(100, 5, false, false), ": Fumble");
    }

    #[test]
    fn high_roll_is_critical_when_greater_is_better() {
        assert_eq!(crit_fumble_result(95, 5, true, false), ": Critical");
    }

    #[test]
    fn middle_roll_is_neither() {
        assert_eq!(crit_fumble_result(6, 5, false, false), "");
        assert_eq!(crit_fumble_result(50, 5, false, false), "");
        assert_eq!(crit_fumble_result(94, 5, false, false), "");
    }
}
