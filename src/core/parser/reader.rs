use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct FileStreamReader {
    reader: BufReader<File>,
    current_line: String,
    current_words: VecDeque<String>,
    pub stack: Vec<char>,
}

impl FileStreamReader {
    pub fn new(filename: &str) -> io::Result<Self> {
        let path = Path::new(filename);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        Ok(FileStreamReader {
            reader,
            current_line: String::new(),
            current_words: VecDeque::new(),
            stack: Vec::new(),
        })
    }

    pub fn next_word(&mut self) -> io::Result<Option<String>> {
        while self.current_words.is_empty() {
            self.current_line.clear();
            let bytes_read = self.reader.read_line(&mut self.current_line)?;
            if bytes_read == 0 {
                return Ok(None);
            }
            if let Some(comment_start) = self.current_line.find(';') {
                self.current_line.truncate(comment_start);
            }
            self.current_line = self.current_line.trim_end().to_string();
            let mut words = VecDeque::new();
            let mut current_word = String::new();
            for ch in self.current_line.chars() {
                if ch.is_whitespace() {
                    if !current_word.is_empty() {
                        words.push_back(current_word.clone());
                        current_word.clear();
                    }
                } else if ch == '(' || ch == ')' {
                    if !current_word.is_empty() {
                        words.push_back(current_word.clone());
                        current_word.clear();
                    }
                    words.push_back(ch.to_string());
                } else {
                    current_word.push(ch);
                }
            }
            if !current_word.is_empty() {
                words.push_back(current_word);
            }
            self.current_words = words;
        }
        Ok(self.current_words.pop_front().map(|word| {
            self.handle_stack(&word);
            word
        }))
    }

    pub fn peek_word(&mut self) -> io::Result<Option<&String>> {
        while self.current_words.is_empty() {
            self.current_line.clear();
            let bytes_read = self.reader.read_line(&mut self.current_line)?;
            if bytes_read == 0 {
                return Ok(None);
            }
            if let Some(comment_start) = self.current_line.find(';') {
                self.current_line.truncate(comment_start);
            }
            self.current_line = self.current_line.trim_end().to_string();
            let mut words = VecDeque::new();
            let mut current_word = String::new();
            for ch in self.current_line.chars() {
                if ch.is_whitespace() {
                    if !current_word.is_empty() {
                        words.push_back(current_word.clone());
                        current_word.clear();
                    }
                } else if ch == '(' || ch == ')' {
                    if !current_word.is_empty() {
                        words.push_back(current_word.clone());
                        current_word.clear();
                    }
                    words.push_back(ch.to_string());
                } else {
                    current_word.push(ch);
                }
            }
            if !current_word.is_empty() {
                words.push_back(current_word);
            }
            self.current_words = words;
        }
        Ok(self.current_words.front())
    }

    fn handle_stack(&mut self, word: &str) {
        for ch in word.chars() {
            match ch {
                '(' | '{' | '[' => self.stack.push(ch),
                ')' => {
                    if self.stack.last() == Some(&'(') {
                        self.stack.pop();
                    }
                }
                '}' => {
                    if self.stack.last() == Some(&'{') {
                        self.stack.pop();
                    }
                }
                ']' => {
                    if self.stack.last() == Some(&'[') {
                        self.stack.pop();
                    }
                }
                _ => {}
            }
        }
    }

    pub fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test01() {
        let path = "/Users/class-undefined/code/rust/drf-parser/src/tests/pdks/s180bcd.drf";
        let mut reader = super::FileStreamReader::new(path).unwrap();
        let word = reader.next_word().unwrap().unwrap();
        println!("{:?}", word);
    }
}
