use color_eyre::eyre::Result;

use std::io::Write;

mod lexer {
    use color_eyre::{eyre::Result, Report};

    #[derive(Debug)]
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
                            res = res * 10 + c.to_digit(10).unwrap_or_else(|| unreachable!());
                        } else {
                            return Err(Report::msg(format!(
                                "Could not parse character '{c}'\n{}",
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

                first => return Err(Report::msg(format!("Unexpected character {first}"))),
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
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // init()
    {
        println!("\nConVector v0.0");
    }

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        let tokens = match lexer::try_parse_from(input) {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("{e:?}");
                continue;
            }
        };
        dbg!(tokens);
    }

    Ok(())
}
