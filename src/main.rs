mod compression;


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    #[case("", 0)]
    #[case("s", 1)]
    #[case("*", 1)]
    #[case("swag", 4)]
    #[case("aabaa", 3)]
    #[case("ababcbababaa", 5)]
    #[case("ababcbababacbaaa", 7)]
    fn test_encode_decode(#[case] s: &str, #[case] encoded_text_size: usize) {
        let plain_text = s.to_string();
        let naive_matcher = super::compression::matchers::NaiveMatcher {};
        let compressor = super::compression::Compressor{
            matcher: Box::new(naive_matcher),
        }; 
        let encoded_text = compressor.compress(&plain_text);
        assert_eq!(encoded_text.len(), encoded_text_size);
        println!("printing encoded text");
        for entry in &encoded_text {
            println!("({}, {}, {})", entry.0, entry.1, entry.2);
        }
        let decoded_text: String = compressor.decompress(encoded_text);
        assert_eq!(plain_text, decoded_text);
    }
}