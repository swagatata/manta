#![feature(test)]
extern crate test;

// #[path = "../src/compression.rs"] // Here
// #[path = "../src/compression/matchers.rs"] // Here
// mod compression;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use manta::compression;

    fn encode_decode_helper(
        s: &String,
        search_buffer_size: usize,
        lookahead_size: usize,
    ) -> String {
        let naive_matcher = compression::matchers::NaiveMatcher {
            search_buffer_size: search_buffer_size,
            lookahead_buffer_size: lookahead_size,
        };
        let compressor = compression::Compressor {
            matcher: Box::new(naive_matcher),
        };
        let encoded_text = compressor.compress(s);
        // println!("printing encoded text");
        // for entry in &encoded_text {
        //     println!("({}, {}, {})", entry.0, entry.1, entry.2);
        // }
        compressor.decompress(encoded_text)
    }

    #[bench]
    fn bench_hundred_thousand(b: &mut Bencher) {
        b.iter(|| {
            let plain_text = "s".repeat(100000);
            let decoded_text = encode_decode_helper(&plain_text, 0, 0);
            assert_eq!(plain_text, decoded_text);
        });
    }

    #[bench]
    fn bench_million(b: &mut Bencher) {
        b.iter(|| {
            let plain_text = "s".repeat(1000000);
            let decoded_text = encode_decode_helper(&plain_text, 0, 0);
            assert_eq!(plain_text, decoded_text);
        });
    }

    #[bench]
    fn bench_million_multi_char(b: &mut Bencher) {
        b.iter(|| {
            let plain_text = "swag".repeat(1000000);
            let decoded_text = encode_decode_helper(&plain_text, 0, 0);
            assert_eq!(plain_text, decoded_text);
        });
    }

    #[bench]
    fn bench_million_multi_char_with_buffer_limit(b: &mut Bencher) {
        b.iter(|| {
            let plain_text = "swag".repeat(1000000);
            let decoded_text = encode_decode_helper(&plain_text, 1000, 1000);
            assert_eq!(plain_text, decoded_text);
        });
    }
}
