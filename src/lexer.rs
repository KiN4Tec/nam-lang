use color_eyre::{eyre::Result, Report};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpAdd,
    OpSupstract,
    OpMultiply,
    OpDivide,

    OpenParen,
    CloseParen,

    Number(u32),
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

            '0'..='9' => {
                let mut res = 0u32;
                for c in input.chars() {
                    if c.is_ascii_digit() {
                        let max = u32::MAX / 10;

                        match res.cmp(&max) {
                            // Normal flow
                            std::cmp::Ordering::Less => {
                                res =
                                    res * 10 + c.to_digit(10).unwrap_or_else(|| unreachable!())
                            },

                            // On the edge of overflow
                            std::cmp::Ordering::Equal => {
                                let d = c.to_digit(10).unwrap_or_else(|| unreachable!());
                                if d > u32::MAX % 10 {
                                    return Err(Report::msg("Integer overflow"));
                                }
                                res = res * 10 + d;
                            },

                            // Overflow
                            std::cmp::Ordering::Greater => {
                                return Err(Report::msg("Integer overflow"))
                            },
                        }
                    } else {
                        return Err(Report::msg(format!(
                            "Could not parse character '{c}' \n{}",
                            "If you wanted to use suffixes, they are not supported yet"
                        )));
                    }
                }
                Ok(Self::Number(res))
            },

            'A'..='Z' | 'a'..='z' | '_' => {
                let mut res = String::new();
                for c in input.chars() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        res.push(c);
                    } else {
                        return Err(Report::msg(format!("Could not parse charater '{c}'")));
                    }
                }
                Ok(Self::Identifier(res))
            },

            first => Err(Report::msg(format!("Unexpected character {first}"))),
        }
    }
}

pub fn try_parse_from(code: String) -> Result<Vec<Token>> {
    let mut res = vec![];
    let mut iter = code.chars().peekable();

    while let Some(first) = iter.next() {
        match first {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                res.push(Token::try_from_str(first.to_string().as_str())?)
            },

            _ if first.is_ascii_alphanumeric() || first == '_' => {
                let mut token = first.to_string();
                while let Some(c) = iter.peek() {
                    if c.is_ascii_alphanumeric() || *c == '_' {
                        token.push(iter.next().unwrap_or_else(|| unreachable!()));
                    } else {
                        break;
                    }
                }
                res.push(Token::try_from_str(token.as_str())?);
            },

            _ if first.is_whitespace() => {},
            _ => return Err(Report::msg("Could not parse charater '{first}'")),
        }
    }

    res.push(Token::EndOfFile);
    Ok(res)
}
