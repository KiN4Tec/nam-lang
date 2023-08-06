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
    fn try_from_str(input: &str) -> Result<Self> {
        match input
            .chars()
            .next()
            .unwrap_or_else(|| unreachable!("Unexpected empty token"))
        {
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
                        return Err(Report::msg(format!("Could not parse charater '{c}'")));
                    }
                }
                Ok(Self::Identifier(input.to_string()))
            },

            first => Err(Report::msg(format!("Unexpected character {first}"))),
        }
    }
}

pub fn try_tokenize(code: String) -> Result<Vec<Token>> {
    let mut res = vec![];
    let mut idx = 0;

    while let Some(first) = code.chars().nth(idx) {
        match first {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                res.push(Token::try_from_str(first.to_string().as_str())?);
                idx += 1;
            },

            _ if first.is_ascii_digit() => {
                let mut token = String::new();
                let mut is_frac = false;
                while let Some(next) = code.chars().nth(idx) {
                    if next.is_ascii_digit() {
                        token.push(next);
                        idx += 1;
                    } else if next == '.' {
                        if is_frac {
                            return Err(Report::msg(format!(
                                "at {}: Could not parse a numeric literal with more than one dot.",
                                idx + 1
                            )));
                        }
                        is_frac = true;
                        token.push('.');
                        idx += 1;
                    } else if next.is_ascii_alphabetic() {
                        return Err(Report::msg(format!(
                            "at {}: Suffixes are not supported yet",
                            idx + 1
                        )));
                    } else {
                        break;
                    }
                }
                res.push(Token::NumericLiteral(token.parse()?));
            },

            _ if first.is_ascii_alphabetic() || first == '_' => {
                let mut token = String::new();
                while let Some(next) = code.chars().nth(idx) {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        token.push(next);
                    } else {
                        break;
                    }
                    idx += 1;
                }
                res.push(Token::try_from_str(token.as_str())?);
            },

            _ if first.is_whitespace() => {
                idx += 1;
            },

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
