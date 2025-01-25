#[derive(Debug)]
struct Tokenizer(String);

#[derive(Debug, PartialEq)]
struct Mul(i64, i64);

#[derive(Debug)]
struct Instructions(Vec<Mul>);

#[derive(Debug, PartialEq)]
enum Token {
    MulStart,
    Number(String),
    Comma,
    MulEnd,
    Garbage,
}

#[derive(Debug)]
enum ParserState {
    Empty,
    MulStart,
    MulFirstArg { arg1: i64 },
    MulFirstArgComma { arg1: i64 },
    MulSecondArg { arg1: i64, arg2: i64 },
}

#[derive(Debug)]
struct Parser(Vec<Token>);

impl Parser {
    fn parse(&self) -> Instructions {
        let mut instructions = vec![];
        let mut state = ParserState::Empty;

        for token in &self.0 {
            state = match state {
                ParserState::Empty => match token {
                    Token::MulStart => ParserState::MulStart,
                    _ => ParserState::Empty,
                },
                ParserState::MulStart => match token {
                    Token::Number(arg1) => ParserState::MulFirstArg {
                        arg1: arg1.parse::<i64>().unwrap(),
                    },
                    _ => ParserState::Empty,
                },
                ParserState::MulFirstArg { arg1 } => match token {
                    Token::Comma => ParserState::MulFirstArgComma { arg1 },
                    _ => ParserState::Empty,
                },
                ParserState::MulFirstArgComma { arg1 } => match token {
                    Token::Number(arg2) => ParserState::MulSecondArg {
                        arg1,
                        arg2: arg2.parse::<i64>().unwrap(),
                    },
                    _ => ParserState::Empty,
                },
                ParserState::MulSecondArg { arg1, arg2 } => match token {
                    Token::MulEnd => {
                        instructions.push(Mul(arg1, arg2));
                        ParserState::Empty
                    }
                    _ => ParserState::Empty,
                },
            }
        }

        Instructions(instructions)
    }
}

impl Tokenizer {
    fn mul_start(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        if rest.starts_with("mul(") {
            Some((Token::MulStart, pos + 4))
        } else {
            None
        }
    }

    fn mul_end(&self, pos: usize) -> Option<(Token, usize)> {
        let rest = self.0.get(pos..)?;
        if rest.starts_with(")") {
            Some((Token::MulEnd, pos + 1))
        } else {
            None
        }
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
        if rest.starts_with(",") {
            Some((Token::Comma, pos + 1))
        } else {
            None
        }
    }

    fn garbage(&self, pos: usize) -> Option<(Token, usize)> {
        Some((Token::Garbage, pos + 1))
    }

    fn tokenize(&self) -> Vec<Token> {
        let tokenizers = vec![
            Self::mul_start,
            Self::mul_end,
            Self::number,
            Self::comma,
            Self::garbage,
        ];

        let mut pos: usize = 0;
        let mut tokens = vec![];

        while pos <= self.0.len() {
            let mut success = false;
            for tokenizer in &tokenizers {
                if let Some((token, new_pos)) = tokenizer(self, pos) {
                    tokens.push(token);
                    pos = new_pos;
                    success = true;
                    break;
                }
            }
            if !success {
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

    fn parse(input: &str) -> Self {
        let tokens = Tokenizer(input.to_string()).tokenize();
        Parser(tokens).parse()
    }
}

fn part1(input: &str) -> i64 {
    Instructions::parse(input).eval()
}

fn part2(_input: &str) -> i64 {
    todo!()
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
        let instructions = Instructions::parse(memory);
        assert_eq!(
            instructions.0,
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );
        assert_eq!(instructions.eval(), 161);
    }
}
