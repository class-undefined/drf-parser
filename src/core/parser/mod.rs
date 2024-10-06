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
    }

    pub fn parse(&mut self) {
        let token = self.reader.peek_word();
        if token.is_err() {
            panic!("{:?}", token.err().unwrap());
        }
        match token.unwrap() {
            Some(word) => {
                if word == "drDefineDisplay(" {
                    self.parse_display();
                }
            }
            None => (),
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
