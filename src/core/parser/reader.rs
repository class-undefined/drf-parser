use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct FileStreamReader<R: BufRead> {
    reader: R,
    current_line: String,
    current_words: VecDeque<String>,
    pub stack: Vec<char>,
    comment_signs: Vec<&'static str>,
}
impl FileStreamReader<BufReader<File>> {
    pub fn from_path(filename: &str, comment_sign: Option<&'static str>) -> io::Result<Self> {
        let file = File::open(Path::new(filename))?;
        Ok(FileStreamReader::new(BufReader::new(file), comment_sign))
    }

    pub fn from_path_with_comments(
        filename: &str,
        comment_signs: Vec<&'static str>,
    ) -> io::Result<Self> {
        let file = File::open(Path::new(filename))?;
        Ok(FileStreamReader::new_with_comments(
            BufReader::new(file),
            comment_signs,
        ))
    }
}

impl<'a> FileStreamReader<BufReader<&'a [u8]>> {
    pub fn from_string(content: &'a str, comment_sign: Option<&'static str>) -> Self {
        FileStreamReader::new(BufReader::new(content.as_bytes()), comment_sign)
    }

    pub fn from_string_with_comments(content: &'a str, comment_signs: Vec<&'static str>) -> Self {
        FileStreamReader::new_with_comments(BufReader::new(content.as_bytes()), comment_signs)
    }
}

impl<R> FileStreamReader<R>
where
    R: BufRead,
{
    pub fn new(reader: R, comment_sign: Option<&'static str>) -> Self {
        let comment_signs = comment_sign.into_iter().collect();
        FileStreamReader::new_with_comments(reader, comment_signs)
    }

    pub fn new_with_comments(reader: R, comment_signs: Vec<&'static str>) -> Self {
        FileStreamReader {
            reader,
            current_line: String::new(),
            current_words: VecDeque::new(),
            stack: Vec::new(),
            comment_signs,
        }
    }

    pub fn next_word(&mut self) -> io::Result<Option<String>> {
        while self.current_words.is_empty() {
            self.current_line.clear();
            let bytes_read = self.reader.read_line(&mut self.current_line)?;
            if bytes_read == 0 {
                return Ok(None);
            }
            self.truncate_comment();

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
            self.truncate_comment();
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

    fn truncate_comment(&mut self) {
        if let Some(comment_start) = self
            .comment_signs
            .iter()
            .filter_map(|sign| self.current_line.find(sign))
            .min()
        {
            self.current_line.truncate(comment_start);
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
        let mut reader = super::FileStreamReader::from_path(path, Some(";")).unwrap();
        let word = reader.next_word().unwrap().unwrap();
        println!("{:?}", word);
    }
}
