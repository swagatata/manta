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
        // println!("cur_i, offset, length, character: {} {} {} {}", 0, 0, 0, chars[0]);
        let mut cur_i: usize = 1;

        let mut character: char;
        let mut offset: usize;
        while cur_i < chars.len() {
            // A round of matching between
            // s[0..cur_i) and
            // s[cur_i..s.len())
            let search_buffer_start_i = match self.matcher.get_search_buffer_size() {
                0 => 0,
                search_buffer_size => match cur_i > search_buffer_size {
                    true => cur_i - search_buffer_size,
                    false => 0, 
                },
            };
            let lookahead_end_i = match self.matcher.get_lookahead_buffer_size() {
                0 => chars.len(),
                lookahead_size => std::cmp::min(cur_i + lookahead_size, chars.len()),
            }; 
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
            // println!("cur_i, offset, length, character: {} {} {} {}", cur_i, offset, length, character);
            cur_i += length + 1;
        }
        buffer
    }

    pub fn decompress(&self, encoded_text: Vec<(usize, usize, char)>) -> String {
        if encoded_text.is_empty() {
            return String::from("");
        }
        let mut total_string_size = 0;
        for (_, length, _) in &encoded_text {
            total_string_size += length + 1;
        }
        let mut decompressed: Vec<char> = Vec::with_capacity(total_string_size);
        let mut cur_i = 0;
        for (index, length, character) in encoded_text {
            // println!("offset, length, character: {} {} {}", index, length, character);
            for i in 0..length {                                
                decompressed.push(decompressed[cur_i + i - index]);
            } 
            cur_i += length;
            if character != 0 as char {
                decompressed.push(character);
            }
            cur_i += 1;
        }
        decompressed.into_iter().collect()
    }
}
