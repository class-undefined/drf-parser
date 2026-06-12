use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct LayerMapParser<R: BufRead> {
    pub layermap: HashMap<(String, String), (usize, usize)>,
    reader: crate::core::parser::reader::FileStreamReader<R>,
}

impl LayerMapParser<BufReader<File>> {
    pub fn from_path(filename: &str) -> io::Result<Self> {
        let reader = crate::core::parser::reader::FileStreamReader::from_path_with_comments(
            filename,
            vec!["#", ";"],
        )?;
        Ok(LayerMapParser {
            layermap: HashMap::new(),
            reader,
        })
    }
}

impl<'a> LayerMapParser<BufReader<&'a [u8]>> {
    pub fn from_string(content: &'a str) -> Self {
        let reader = crate::core::parser::reader::FileStreamReader::from_string_with_comments(
            content,
            vec!["#", ";"],
        );
        LayerMapParser {
            layermap: HashMap::new(),
            reader,
        }
    }
}

impl<R: BufRead> LayerMapParser<R> {
    pub fn parse(&mut self) {
        while self.reader.peek_word().is_ok() && self.reader.peek_word().as_ref().unwrap().is_some()
        {
            let layer_name = self.reader.next_word().unwrap().unwrap();
            let purpose = self.reader.next_word().unwrap().unwrap();
            let layer_number = self
                .reader
                .next_word()
                .unwrap()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let purpose_number = self
                .reader
                .next_word()
                .unwrap()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            self.layermap.insert(
                (layer_name.to_string(), purpose.to_string()),
                (layer_number, purpose_number),
            );
        }
    }

    pub fn to_json(&self) -> String {
        let mut layermap = HashMap::with_capacity(self.layermap.len());
        for (k, v) in &self.layermap {
            let key = format!("{}#{}", k.0, k.1);
            layermap.insert(key, v);
        }
        serde_json::to_string(&layermap).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::LayerMapParser;

    #[test]
    fn parses_semicolon_and_hash_comments() {
        let content = r#"
; header comment
0M drawing 1 0
#CM drawing 51 0
TE drawing 51 0
BE drawing 58 0 ; inline comment
"#;
        let mut parser = LayerMapParser::from_string(content);
        parser.parse();

        assert_eq!(parser.layermap.len(), 3);
        assert_eq!(
            parser
                .layermap
                .get(&(String::from("0M"), String::from("drawing"))),
            Some(&(1, 0))
        );
        assert_eq!(
            parser
                .layermap
                .get(&(String::from("TE"), String::from("drawing"))),
            Some(&(51, 0))
        );
        assert_eq!(
            parser
                .layermap
                .get(&(String::from("BE"), String::from("drawing"))),
            Some(&(58, 0))
        );
    }

    #[test]
    fn test01() {
        let mut parser = LayerMapParser::from_path(
            "/Users/class-undefined/code/rust/drf-parser/src/tests/pdks/tsmcN65.layermap",
        );
        parser.as_mut().unwrap().parse();
        println!("{:?}", parser.as_ref().unwrap().to_json())
    }
}
