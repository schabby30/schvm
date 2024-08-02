use std::{fs::read_to_string, iter::{self, from_fn}, path::PathBuf};

use crate::instructions::Instruction;

enum Token {
    Number(u16),
    String(String),
    Comma,
}

fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            ',' => tokens.push(Token::Comma),
            '0'..='9' => {
                let token: u16 = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::Number(token));
            },
            'a'..='z' => {
                let token: String = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_alphabetic())))
                    .collect::<String>();

                tokens.push(Token::String(token));
            }
            _ => return Err(format!("unrecognized character {}", ch)),
        }
    }

    Ok(tokens)
}

pub fn compile(file_path: PathBuf) -> Result<Vec<u8>, String> {
    let mut tokens : Vec<Token> = Vec::new();
    let string_content = read_to_string(file_path).unwrap();

    for line in string_content.lines() {
        tokens.append(&mut tokenizer(line)?);
    }

    let mut bytes: Vec<u8> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => {
                if n <= u16::MAX {
                    bytes.append(&mut Vec::from(n.to_be_bytes()));
                } else {
                    return Err("Too big number...".to_string());
                }
            },
            Token::String(s) => {
                let str = s.as_str();

                match str {
                    "lda" => bytes.push(Instruction::LoadA as u8),
                    "ldb" => bytes.push(Instruction::LoadB as u8),
                    "ldc" => bytes.push(Instruction::LoadC as u8),
                    "add" => bytes.push(Instruction::Add as u8),
                    "mov" => bytes.push(Instruction::Move as u8),
                    "jmp" => bytes.push(Instruction::Jump as u8),
                    "hlt" => bytes.push(Instruction::Halt as u8),
                    _ => return Err("Unknown instruction...".to_string()),
                }
            },
            Token::Comma => {},
        }
    }

    Ok(bytes)
}