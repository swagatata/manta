pub struct Compressor {}

impl Compressor {
    pub fn compress(&self, s: &String) -> Vec<(u32, u32, u8)> {
        if s.is_empty() {
            return vec![];
        }
        let mut cur_i = 0;
        let b = s.as_bytes();
        let mut buffer: Vec<(u32, u32, u8)> = vec![(0, 0, b[0])];
        loop {
            cur_i += (buffer.last().unwrap().1 + 1);
            if cur_i >= b.len() as u32 {
                return buffer;
            }
        }
        return buffer;
    }

    pub fn decompress(&self, encoded_text: Vec<(u32, u32, u8)>) -> String {
        if encoded_text.is_empty() {
            return String::from("");
        }
        let mut s = String::new();
        for (i, l, c) in encoded_text {
            s.push(c as char);
        }
        return s;
    }
}
