use color_eyre::eyre::Result;

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

impl TryFrom<String> for Token {
    type Error = TokenizationError;

    fn try_from(input: String) -> std::result::Result<Self, Self::Error> {
        let first = match input.chars().next() {
            Some(c) => c,
            None => {
                return Err(TokenizationError {
                    kind: TokenizationErrorKind::EmptyString,
                    token_str: Some(input),
                    message: None,
                })
            },
        };

        match first {
            '+' => Ok(Self::OpAdd),
            '-' => Ok(Self::OpSupstract),
            '*' => Ok(Self::OpMultiply),
            '/' => Ok(Self::OpDivide),

            '(' => Ok(Self::OpenParen),
            ')' => Ok(Self::CloseParen),

            '0'..='9' => match input.parse() {
                Ok(r) => Ok(Self::NumericLiteral(r)),
                Err(e) => Err(TokenizationError {
                    kind: TokenizationErrorKind::NotANumber,
                    token_str: Some(input),
                    message: Some(e.to_string()),
                }),
            },

            'A'..='Z' | 'a'..='z' | '_' => {
                for c in input.chars() {
                    if !c.is_ascii_alphanumeric() && c != '_' {
                        return Err(TokenizationError {
                            kind: TokenizationErrorKind::UnexpectedChar(c),
                            token_str: Some(input),
                            message: None,
                        });
                    }
                }
                Ok(Self::Identifier(input))
            },

            first => Err(TokenizationError {
                kind: TokenizationErrorKind::UnexpectedChar(first),
                token_str: Some(input),
                message: None,
            }),
        }
    }
}

#[allow(dead_code)]
pub fn tokenize(code: String) -> Vec<Token> {
    match try_tokenize(code) {
        Ok(tokens) => tokens,
        Err(e) => panic!("{e:?}"),
    }
}

pub fn try_tokenize(code: String) -> Result<Vec<Token>, TokenizationError> {
    let mut res = vec![];
    let mut chars = code.chars().enumerate().peekable();

    while let Some((_idx, first)) = chars.next() {
        match first {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                res.push(Token::try_from(first.to_string())?);
            },

            '0'..='9' => {
                let mut token = String::from(first);
                let mut is_frac = false;

                while let Some(&(_idx, next)) = chars.peek() {
                    match next {
                        '0'..='9' | '_' => token.push(next),

                        '.' => {
                            if is_frac {
                                token.push('.');
                                return Err(TokenizationError {
                                    kind: TokenizationErrorKind::UnexpectedChar('.'),
                                    token_str: Some(token),
                                    message: Some(String::from(
                                        "Could not parse a numeric literal with more than one dot.",
                                    )),
                                });
                            }
                            is_frac = true;
                            token.push('.');
                        },

                        'A'..='Z' | 'a'..='z' => {
                            token.push(next);
                            return Err(TokenizationError {
                                kind: TokenizationErrorKind::UnspportedSyntax(next.to_string()),
                                token_str: Some(token),
                                message: Some(String::from("Suffixes are not supported, yet!")),
                            });
                        },

                        _ => break,
                    }
                    chars.next();
                }

                let res_num = match token.parse() {
                    Ok(r) => Token::NumericLiteral(r),
                    Err(e) => {
                        return Err(TokenizationError {
                            kind: TokenizationErrorKind::NotANumber,
                            token_str: Some(token),
                            message: Some(e.to_string()),
                        });
                    },
                };

                res.push(res_num);
            },

            'A'..='Z' | 'a'..='z' | '_' => {
                let mut token = String::from(first);
                while let Some(&(_idx, next)) = chars.peek() {
                    if !next.is_ascii_alphanumeric() && next != '_' {
                        break;
                    }
                    token.push(next);
                    chars.next();
                }
                res.push(Token::try_from(token)?);
            },

            _ if first.is_whitespace() => {},
            c => {
                return Err(TokenizationError {
                    kind: TokenizationErrorKind::UnexpectedChar(c),
                    token_str: None,
                    message: None,
                })
            },
        }
    }

    res.push(Token::EndOfFile);
    Ok(res)
}

////////////////////////////////
//       Error Handling       //
////////////////////////////////

#[derive(Debug)]
pub struct TokenizationError {
    kind: TokenizationErrorKind,
    token_str: Option<String>,
    message: Option<String>,
}

#[derive(Debug)]
pub enum TokenizationErrorKind {
    EmptyString,
    NotANumber,
    UnexpectedChar(char),
    UnspportedSyntax(String),
}

impl std::error::Error for TokenizationError {}
impl std::fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenizationErrorKind::*;

        let mut err_message = match &self.kind {
            EmptyString => String::from("Unexpected empty string"),
            NotANumber => String::from("Could not parse as number"),
            UnexpectedChar(c) => format!("Unexpected character '{c}'"),
            UnspportedSyntax(s) => format!("Unsupported syntax '{s}'"),
        };

        if let Some(token) = &self.token_str {
            err_message = format!("{err_message}\nError found in token '{token}'");
        }

        if let Some(message) = &self.message {
            err_message = format!("{err_message}\n{message}");
        }

        write!(f, "{}", err_message)
    }
}
