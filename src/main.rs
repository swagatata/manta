mod compression;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::compression::*;
    use rstest::*;

    fn helper(
        s: &str,
        search_buffer_size: usize,
        lookahead_size: usize,
        encoded_text_size: usize,
    ) -> String {
        let plain_text = s.to_string();
        let naive_matcher = matchers::NaiveMatcher {
            search_buffer_size: search_buffer_size,
            lookahead_buffer_size: lookahead_size,
        };
        let compressor = Compressor {
            matcher: Box::new(naive_matcher),
        };
        let encoded_text = compressor.compress(&plain_text);
        assert_eq!(encoded_text.len(), encoded_text_size);
        println!("printing encoded text");
        for entry in &encoded_text {
            println!("({}, {}, {})", entry.0, entry.1, entry.2);
        }
        compressor.decompress(encoded_text)
    }

    /**
     * No limit on search buffer or look ahead
     */
    #[rstest]
    #[case("", 0)]
    #[case("s", 1)]
    #[case("*", 1)]
    #[case("swag", 4)]
    #[case("aabaa", 3)]
    #[case("ababa", 3)]
    #[case("abaaba", 4)]
    #[case("ababcbababaa", 5)]
    #[case("ababcbababacbaaa", 7)]
    fn test_encode_decode(#[case] s: &str, #[case] encoded_text_size: usize) {
        let decoded_text = helper(s, 0, 0, encoded_text_size);
        assert_eq!(s.to_string(), decoded_text);
    }

    /**
     * Test with limited search buffer
     */
    #[rstest]
    #[case("", 1, 0)]
    #[case("s", 1, 1)]
    #[case("*", 1, 1)]
    #[case("swag", 1, 4)]
    #[case("swag", 2, 4)]
    #[case("swag", 3, 4)]
    #[case("aabaa", 4, 3)]
    #[case("aabaa", 1, 4)]
    #[case("ababcbababaa", 4, 5)]
    #[case("ababcbababacbaaa", 6, 7)]
    fn test_with_search_buffer(
        #[case] s: &str,
        #[case] search_buffer_size: usize,
        #[case] encoded_text_size: usize,
    ) {
        let decoded_text = helper(s, search_buffer_size, 0, encoded_text_size);
        assert_eq!(s.to_string(), decoded_text);
    }

    /**
     * Test with limited lookaside buffer and search buffer
     */
    #[rstest]
    #[case("", 1, 0, 0)]
    #[case("s", 1, 1, 1)]
    #[case("*", 1, 1, 1)]
    #[case("swag", 1, 4, 4)]
    #[case("swag", 2, 4, 4)]
    #[case("swag", 3, 4, 4)]
    #[case("aabaa", 4, 1, 3)]
    #[case("aabaa", 1, 4, 4)]
    #[case("ababcbababaa", 4, 5, 5)]
    #[case("ababcbababaa", 4, 2, 6)]
    #[case("ababcbababacbaaa", 6, 2, 8)]
    fn test_with_lookaside_buffer(
        #[case] s: &str,
        #[case] search_buffer_size: usize,
        #[case] lookaside_buffer_size: usize,
        #[case] encoded_text_size: usize,
    ) {
        let decoded_text = helper(
            s,
            search_buffer_size,
            lookaside_buffer_size,
            encoded_text_size,
        );
        assert_eq!(s.to_string(), decoded_text);
    }

    /**
     * Test with big data sets
     */
    #[rstest]
    #[case(10000, 100)]
    #[case(100000, 100)]
    fn test_big_strings(#[case] string_size: usize, #[case] buffer_size: usize) {
        // large string with a single character repeated n times
        let plain_text = "a".repeat(string_size);
        let naive_matcher = matchers::NaiveMatcher::default();
        let mut compressor = Compressor {
            matcher: Box::new(naive_matcher),
        };
        let encoded_text = compressor.compress(&plain_text);
        let expected_code_size = (string_size as f64).log2().ceil() as usize;
        assert!(
            encoded_text.len() <= expected_code_size,
            "left : {} right : {}",
            encoded_text.len(),
            expected_code_size,
        );
        let decoded_text = compressor.decompress(encoded_text);
        assert_eq!(plain_text, decoded_text);

        compressor = Compressor {
            matcher: Box::new(matchers::NaiveMatcher {
                search_buffer_size: buffer_size,
                lookahead_buffer_size: buffer_size,
            }),
        };
        let encoded_text = compressor.compress(&plain_text);
        assert!(
            encoded_text.len()
                <= (string_size / buffer_size) + (buffer_size as f64).log2().ceil() as usize,
            "left : {} right : {}",
            encoded_text.len(),
            10000.0_f64.log2()
        );
        let decoded_text: String = compressor.decompress(encoded_text);
        assert_eq!(plain_text, decoded_text);
    }
}
