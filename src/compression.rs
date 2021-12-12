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
        println!("cur_i, offset, length, character: {} {} {} {}", 0, 0, 0, chars[0]);
        let mut cur_i: usize = 1;
        let search_buffer_size = self.matcher.get_search_buffer_size(); 
        let lookahead_size = self.matcher.get_lookahead_buffer_size();

        let mut character: char;
        let mut offset: usize;
        while cur_i < chars.len() {
            // A round of matching between
            // s[0..cur_i) and
            // s[cur_i..s.len())
            let search_buffer_start_i = match search_buffer_size {
                0 => 0,
                search_buffer_size => match cur_i > search_buffer_size {
                    true => cur_i - search_buffer_size,
                    false => 0, 
                },
            };
            let lookahead_end_i = match lookahead_size {
                0 => chars.len(),
                _ => std::cmp::min(cur_i + lookahead_size, chars.len()),
            }; 
            println!("search_buffer_start_i is {}", search_buffer_start_i);
            let (start_i, length) = self.matcher.find_max_match(
                &chars[search_buffer_start_i..cur_i], 
                &chars[cur_i..lookahead_end_i]
            );
            offset = match length {
                0 => 0,
                _ => cur_i - (search_buffer_start_i + start_i), 
            };
            character = match chars.len() > cur_i + length {
                true => chars[cur_i + length],
                false => 0 as char,
            };
            buffer.push((offset, length, character));
            println!("cur_i, offset, length, character: {} {} {} {}", cur_i, offset, length, character);
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
