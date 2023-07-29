use color_eyre::eyre::Result;

use std::io::Write;

mod lexer {
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
        fn from_str(input: &str) -> Self {
            match input
                .chars()
                .next()
                .unwrap_or_else(|| panic!("Unexpected empty token"))
            {
                '+' => Self::OpAdd,
                '-' => Self::OpSupstract,
                '*' => Self::OpMultiply,
                '/' => Self::OpDivide,

                '(' => Self::OpenParen,
                ')' => Self::CloseParen,

                '0'..='9' => {
                    let mut res = 0u32;
                    for c in input.chars() {
                        if c.is_ascii_digit() {
                            res = res * 10 + c.to_digit(10).unwrap();
                        } else {
                            panic!("Could not parse character {c}");
                        }
                    }
                    Self::Number(res)
                },

                'A'..='Z' | 'a'..='z' | '_' => {
                    let mut res = String::new();
                    for c in input.chars() {
                        if c.is_ascii_alphabetic() || c == '_' {
                            res.push(c);
                        } else {
                            panic!("Could not parse character {c}");
                        }
                    }
                    Self::Identifier(res)
                },

                c => panic!("Unexpected character {c}"),
            }
        }
    }

    pub fn parse_from(code: String) -> Vec<Token> {
        let mut res = vec![];
        let mut iter = code.chars().peekable();

        while let Some(first) = iter.next() {
            match first {
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    res.push(Token::from_str(first.to_string().as_str()))
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
                    res.push(Token::from_str(token.as_str()));
                },

                _ if first.is_whitespace() => {},
                _ => panic!("Could not parse charater '{first}'"),
            }
        }

        res.push(Token::EndOfFile);
        res
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

        let tokens = lexer::parse_from(input);
        dbg!(tokens);
    }

    Ok(())
}
