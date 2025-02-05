#[derive(Debug)]
pub struct Trace(pub Vec<TraceEvent>);

#[derive(Debug)]
pub enum TraceEvent {
    TokenizerEvent {
        pos: usize,
        tokens: Vec<Token>,
        evaluation: Option<String>,
    },
    ParserEvent {
        pos: usize,
        tokens: Vec<Token>,
        state: ParserState,
        evaluation: Option<String>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Mul(i64, i64);

#[derive(Debug)]
pub struct Instructions(pub Vec<Mul>);

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Enable,
    Disable,
    MulStart,
    Number(String),
    Comma,
    MulEnd,
    Garbage,
}

#[derive(Debug)]
pub struct Parser(pub Vec<Token>);

#[derive(Debug)]
pub enum ParserState {
    Empty { enabled: bool },
    MulStart { enabled: bool },
    MulFirstArg { enabled: bool, arg1: i64 },
    MulFirstArgComma { enabled: bool, arg1: i64 },
    MulSecondArg { enabled: bool, arg1: i64, arg2: i64 },
}

#[derive(Debug, PartialEq)]
pub enum ParserConfig {
    Part1,
    Part2,
}

#[derive(Debug)]
pub struct Tokenizer(pub String);

impl ParserState {
    pub fn enabled(&self) -> bool {
        match self {
            ParserState::Empty { enabled } => *enabled,
            ParserState::MulStart { enabled } => *enabled,
            ParserState::MulFirstArg { enabled, .. } => *enabled,
            ParserState::MulFirstArgComma { enabled, .. } => *enabled,
            ParserState::MulSecondArg { enabled, .. } => *enabled,
        }
    }
}

impl Parser {
    pub fn parse(&self, config: ParserConfig) -> Instructions {
        let mut instructions: Vec<Mul> = vec![];
        let mut state = ParserState::Empty { enabled: true };

        // TODO: fold?
        // let (state, instructions): (ParserState, Vec<Mul>) = self.0.iter().fold(
        //     (ParserState::Empty { enabled: true }, vec![]),
        //     |(state, instructions): (ParserState, Vec<Mul>), token: &Token| todo!(),
        // );

        for token in &self.0 {
            state = match token {
                Token::Enable => ParserState::Empty { enabled: true },
                Token::Disable => ParserState::Empty { enabled: false },
                Token::MulStart => match state {
                    ParserState::Empty { enabled } => ParserState::MulStart { enabled },
                    _ => ParserState::Empty {
                        enabled: state.enabled(),
                    },
                },
                Token::Number(num) => match state {
                    ParserState::MulStart { enabled } => ParserState::MulFirstArg {
                        enabled,
                        arg1: num.parse::<i64>().unwrap(),
                    },
                    ParserState::MulFirstArgComma { enabled, arg1 } => ParserState::MulSecondArg {
                        enabled,
                        arg1,
                        arg2: num.parse::<i64>().unwrap(),
                    },
                    _ => ParserState::Empty {
                        enabled: state.enabled(),
                    },
                },
                Token::Comma => match state {
                    ParserState::MulFirstArg { enabled, arg1 } => {
                        ParserState::MulFirstArgComma { enabled, arg1 }
                    }
                    _ => ParserState::Empty {
                        enabled: state.enabled(),
                    },
                },
                Token::MulEnd => match state {
                    ParserState::MulSecondArg {
                        enabled,
                        arg1,
                        arg2,
                    } => {
                        if enabled || config == ParserConfig::Part1 {
                            instructions.push(Mul(arg1, arg2));
                        }
                        ParserState::Empty { enabled }
                    }
                    _ => ParserState::Empty {
                        enabled: state.enabled(),
                    },
                },
                Token::Garbage => ParserState::Empty {
                    enabled: state.enabled(),
                },
            };
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

    pub fn tokenize(&self) -> (Vec<Token>, Trace) {
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
        let mut tokens: Vec<Token> = vec![];
        let mut trace: Vec<TraceEvent> = vec![];

        while pos <= self.0.len() {
            trace.push(TraceEvent::TokenizerEvent {
                pos,
                tokens: tokens.clone(),
                evaluation: None,
            });
            if let Some((token, new_pos)) = tokenizers.iter().find_map(|t| t(self, pos)) {
                trace.push(TraceEvent::TokenizerEvent {
                    pos,
                    tokens: tokens.clone(),
                    evaluation: Some(format!("found token: {:?}", token)),
                });
                tokens.push(token);
                pos = new_pos;
            } else {
                panic!("failed to match any tokenizers")
            }
        }
        (tokens, Trace(trace))
    }
}

impl Mul {
    pub fn eval(&self) -> i64 {
        self.0 * self.1
    }
}

impl Instructions {
    pub fn eval(&self) -> i64 {
        self.0.iter().map(|mul| mul.eval()).sum()
    }

    pub fn parse(input: &str, config: ParserConfig) -> (Self, Trace) {
        let (tokens, trace) = Tokenizer(input.to_string()).tokenize();
        (Parser(tokens).parse(config), trace)
    }
}

pub fn part1(input: &str) -> i64 {
    let (instructions, _) = Instructions::parse(input, ParserConfig::Part1);
    instructions.eval()
}

pub fn part2(input: &str) -> i64 {
    let (instructions, _) = Instructions::parse(input, ParserConfig::Part2);
    instructions.eval()
}

pub const INPUT: &str = include_str!("day03.input");

pub const PART1_EXAMPLE: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
pub const PART2_EXAMPLE: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let memory = PART1_EXAMPLE.trim();
        let instructions = Instructions::parse(memory, ParserConfig::Part1);
        assert_eq!(
            instructions.0,
            vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)]
        );
        assert_eq!(instructions.eval(), 161);
    }

    #[test]
    fn example_part2() {
        let memory = PART2_EXAMPLE.trim();
        let instructions = Instructions::parse(memory, ParserConfig::Part2);
        assert_eq!(instructions.0, vec![Mul(2, 4), Mul(8, 5)]);
        assert_eq!(instructions.eval(), 48);
    }
}
