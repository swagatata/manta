pub mod matchers;

pub struct Compressor {
    pub matcher: Box<dyn matchers::Matcher>,
}

impl Compressor {
    pub fn compress(&self, s: &String) -> Vec<(usize, usize, char)> {
        if s.is_empty() {
            return vec![];
        }
        let chars: Vec<char> = s.chars().collect();
        let mut buffer: Vec<(usize, usize, char)> = vec![(0, 0, chars[0])];
        let mut cur_i: usize = 1;
        println!("cur_i & length: {} {}", 0, 0);

        let mut character: char;
        let mut offset: usize;
        while cur_i < chars.len() {
            // A round of matching between
            // s[0..cur_i) and
            // s[cur_i..s.len())
            let (start_i, length) = self.matcher.find_max_match(&chars[0..cur_i], &chars[cur_i..s.len()]);
            offset = match length {
                0 => 0,
                _ => cur_i - start_i, 
            };
            character = match chars.len() > cur_i + length {
                true => chars[cur_i + length],
                false => 0 as char,
            };
            buffer.push((offset, length, character));
            println!("cur_i, length, character: {} {} {}", cur_i, length, character);
            cur_i += length + 1;
        }
        buffer
    }

    pub fn decompress(&self, encoded_text: Vec<(usize, usize, char)>) -> String {
        if encoded_text.is_empty() {
            return String::from("");
        }
        let mut string = String::new();
        let mut cur_i = 0;
        for (index, length, character) in encoded_text {
            for i in 0..length {                                
                string.push(string.chars().nth(cur_i + i - index ).unwrap());
            } 
            cur_i += length;
            if character != 0 as char {
                string.push(character);
            }
            cur_i += 1;
        }
        string
    }
}
