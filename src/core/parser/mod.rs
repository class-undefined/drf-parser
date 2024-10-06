use std::collections::HashMap;

use super::ast;

pub mod reader;

pub struct Parser {
    pub drf: HashMap<String, ast::drf::Drf>,
    pub reader: reader::FileStreamReader,
}

impl Parser {
    pub fn new(path: &str) -> std::io::Result<Self> {
        Ok(Parser {
            drf: HashMap::new(),
            reader: reader::FileStreamReader::new(path)?,
        })
    }

    fn parse_display(&mut self) {
        let header = self.reader.next_word().unwrap().unwrap();
        assert!(
            header == "drDefineDisplay(",
            "Expected drDefineDisplay, got {}",
            header
        );
        while self.reader.is_stack_empty() == false {
            let word = self.reader.next_word().unwrap().unwrap();
            if word == "(" || word == ")" {
                continue;
            }
            let drf = ast::drf::Drf::new(word.clone());
            self.drf.insert(word, drf);
        }
    }

    fn parse_color(&mut self) {
        let header = self.reader.next_word().unwrap().unwrap();
        assert!(
            header == "drDefineColor(",
            "Expected drDefineColor, got {}",
            header
        );
        let mut params: Vec<String> = Vec::with_capacity(6);
        while self.reader.stack.len() >= 1 {
            let word = self.reader.next_word().unwrap().unwrap();
            if word == ")" && self.reader.stack.len() == 0 {
                break;
            }
            if word == "(" {
                continue;
            }
            if word == ")" {
                if self.reader.stack.len() == 0 {
                    break;
                }
                let display = params[0].clone();
                let color = ast::color::Color::from_vec(&params);
                self.drf
                    .get_mut(&display)
                    .unwrap()
                    .colors
                    .insert(color.name.clone(), color);
                params.clear();

                continue;
            }
            params.push(word);
        }
    }

    fn parse_stipple(&mut self) {
        let header = self.reader.next_word().unwrap().unwrap();
        assert!(
            header == "drDefineStipple(",
            "Expected drDefineStipple, got {}",
            header
        );
        todo!("parse_stipple");
    }

    pub fn parse(&mut self) {
        let mut token = self.reader.peek_word();
        while token.is_ok() {
            match token.unwrap() {
                Some(word) => {
                    if word == "drDefineDisplay(" {
                        self.parse_display();
                    } else if word == "drDefineColor(" {
                        self.parse_color();
                    }
                }
                None => (),
            }
            token = self.reader.peek_word();
        }
        if token.is_err() {
            panic!("{:?}", token.err().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::parser::{reader, Parser};

    #[test]
    fn test01() {
        let path = "/Users/class-undefined/code/rust/drf-parser/src/tests/pdks/hh180.drf";
        let mut parser = Parser::new(path).unwrap();
        parser.parse();
    }
}
