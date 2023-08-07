use color_eyre::{eyre::Result, Report};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpAdd,
    OpSupstract,
    OpMultiply,
    OpDivide,

    OpenParen,
    CloseParen,

    NumericLiteral(f64),
    Identifier(String),

    EndOfFile,
}

impl Token {
    fn try_from_str(input: String) -> Result<Self> {
        let first = match input.chars().next() {
            Some(c) => c,
            None => return Err(Report::msg("Unexpected empty token")),
        };

        match first {
            '+' => Ok(Self::OpAdd),
            '-' => Ok(Self::OpSupstract),
            '*' => Ok(Self::OpMultiply),
            '/' => Ok(Self::OpDivide),

            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::CloseParen),

            '0'..='9' => Ok(Self::NumericLiteral(input.parse()?)),

            'A'..='Z' | 'a'..='z' | '_' => {
                for c in input.chars() {
                    if !(c.is_ascii_alphanumeric() || c == '_') {
                        return Err(Report::msg(format!(
                            "Unexpected character {c} in token {input}"
                        )));
                    }
                }
                Ok(Self::Identifier(input))
            },

            first => Err(Report::msg(format!(
                "Unexpected character {first} in token {input}"
            ))),
        }
    }
}

pub fn try_tokenize(code: String) -> Result<Vec<Token>> {
    let mut res = vec![];
    let mut chars = code.chars().enumerate().peekable();

    while let Some((_idx, first)) = chars.next() {
        match first {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                res.push(Token::try_from_str(first.to_string())?);
            },

            _ if first.is_ascii_digit() => {
                let mut token = String::from(first);
                let mut is_frac = false;

                while let Some(&(idx, next)) = chars.peek() {
                    match next {
                        '0'..='9' => {
                            token.push(next);
                        },

                        '.' => {
                            if is_frac {
                                return Err(Report::msg(format!(
                                    "at {}: Could not parse a numeric literal with more than one dot.",
                                    idx + 1
                                )));
                            }
                            is_frac = true;
                            token.push('.');
                        },

                        'A'..='Z' | 'a'..='z' => {
                            return Err(Report::msg(format!(
                                "at {}: Suffixes are not supported yet",
                                idx + 1
                            )));
                        },
                        _ => break,
                    }
                    chars.next();
                }
                res.push(Token::NumericLiteral(token.parse()?));
            },

            _ if first.is_ascii_alphabetic() || first == '_' => {
                let mut token = String::from(first);
                while let Some(&(_idx, next)) = chars.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        token.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                res.push(Token::try_from_str(token)?);
            },

            _ if first.is_whitespace() => {},
            _ => return Err(Report::msg(format!("Could not parse charater '{first}'"))),
        }
    }

    res.push(Token::EndOfFile);
    Ok(res)
}

#[allow(dead_code)]
pub fn tokenize(code: String) -> Vec<Token> {
    match try_tokenize(code) {
        Ok(tokens) => tokens,
        Err(e) => panic!("{e:?}"),
    }
}
