#[derive(Debug)]
struct Tokenizer(String);

#[derive(Debug, PartialEq)]
struct Mul(i64, i64);

#[derive(Debug)]
struct Instructions(Vec<Mul>);

#[derive(Debug, PartialEq)]
enum Token {
    Enable,
    Disable,
    MulStart,
    Number(String),
    Comma,
    MulEnd,
    Garbage,
}

#[derive(Debug)]
enum ParserState {
    Empty { enabled: bool },
    MulStart { enabled: bool },
    MulFirstArg { enabled: bool, arg1: i64 },
    MulFirstArgComma { enabled: bool, arg1: i64 },
    MulSecondArg { enabled: bool, arg1: i64, arg2: i64 },
}

#[derive(Debug)]
struct Parser(Vec<Token>);

#[derive(Debug, PartialEq)]
enum ParserConfig {
    Part1,
    Part2,
}

impl Parser {
    fn parse(&self, config: ParserConfig) -> Instructions {
        let mut instructions = vec![];
        let mut state = ParserState::Empty { enabled: true };

        for token in &self.0 {
            // do() or don't()
            if let Token::Enable | Token::Disable = token {
                let enabled: bool = match token {
                    Token::Enable => true,
                    Token::Disable => false,
                    _ => panic!(),
                };
                state = ParserState::Empty { enabled };
                continue;
            }
            // mul(arg1, arg2)
            state = match state {
                ParserState::Empty { enabled } => match token {
                    Token::MulStart => ParserState::MulStart { enabled },
                    _ => ParserState::Empty { enabled },
                },
                ParserState::MulStart { enabled } => match token {
                    Token::Number(arg1) => ParserState::MulFirstArg {
                        enabled,
                        arg1: arg1.parse::<i64>().unwrap(),
                    },
                    _ => ParserState::Empty { enabled },
                },
                ParserState::MulFirstArg { enabled, arg1 } => match token {
                    Token::Comma => ParserState::MulFirstArgComma { enabled, arg1 },
                    _ => ParserState::Empty { enabled },
                },
                ParserState::MulFirstArgComma { enabled, arg1 } => match token {
                    Token::Number(arg2) => ParserState::MulSecondArg {
                        enabled,
                        arg1,
                        arg2: arg2.parse::<i64>().unwrap(),
                    },
                    _ => ParserState::Empty { enabled },
                },
                ParserState::MulSecondArg {
                    enabled,
                    arg1,
                    arg2,
                } => match token {
                    Token::MulEnd => {
                        if enabled || config == ParserConfig::Part1 {
                            instructions.push(Mul(arg1, arg2));
                        }
                        ParserState::Empty { enabled }
                    }
                    _ => ParserState::Empty { enabled },
                },
            }
        }

        Instructions(instructions)
    }
}

impl Tokenizer {
    fn enable(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        rest.starts_with("do()").then(|| (Token::Enable, pos + 4))
    }

    fn disable(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        rest.starts_with("don't()")
            .then(|| (Token::Disable, pos + 7))
    }

    fn mul_start(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        rest.starts_with("mul(").then(|| (Token::MulStart, pos + 4))
    }

    fn mul_end(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        rest.starts_with(")").then(|| (Token::MulEnd, pos + 1))
    }

    fn number(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        if let Some(number) = rest.split(|c| !char::is_numeric(c)).next() {
            if !number.is_empty() {
                return Some((Token::Number(number.to_string()), pos + number.len()));
            }
        }
        None
    }

    fn comma(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        rest.starts_with(",").then(|| (Token::Comma, pos + 1))
    }

    fn garbage(&self, pos: usize) -> Option<(Token, usize)> {
        Some((Token::Garbage, pos + 1))
    }

    fn tokenize(&self) -> Vec<Token> {
        let tokenizers = [
            Self::enable,
            Self::disable,
            Self::mul_start,
            Self::mul_end,
            Self::number,
            Self::comma,
            Self::garbage,
        ];

        let mut pos: usize = 0;
        let mut tokens = vec![];

        while pos <= self.0.len() {
            if let Some((token, new_pos)) = tokenizers.iter().find_map(|t| t(self, pos)) {
                tokens.push(token);
                pos = new_pos;
            } else {
                panic!("failed to match any tokenizers")
            }
        }
        tokens
    }
}

impl Mul {
    fn eval(&self) -> i64 {
        self.0 * self.1
    }
}

impl Instructions {
    fn eval(&self) -> i64 {
        self.0.iter().map(|mul| mul.eval()).sum()
    }

    fn parse(input: &str, config: ParserConfig) -> Self {
        let tokens = Tokenizer(input.to_string()).tokenize();
        Parser(tokens).parse(config)
    }
}

fn part1(input: &str) -> i64 {
    Instructions::parse(input, ParserConfig::Part1).eval()
}

fn part2(input: &str) -> i64 {
    Instructions::parse(input, ParserConfig::Part2).eval()
}

fn main() {
    let input = include_str!("day03.input");

    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let memory =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".trim();
        let instructions = Instructions::parse(memory, ParserConfig::Part1);
        assert_eq!(
            instructions.0,
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );
        assert_eq!(instructions.eval(), 161);
    }

    #[test]
    fn example_part2() {
        let memory =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".trim();
        let instructions = Instructions::parse(memory, ParserConfig::Part2);
        assert_eq!(instructions.0, vec![Mul(2, 4), Mul(8, 5)]);
        assert_eq!(instructions.eval(), 48);
    }
}
