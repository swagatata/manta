/**
 * Type representing any implementation of the matching algorithm.
 */
pub trait Matcher {
    /**
     * Given two string slices, find the longest prefix of the pattern
     * present in the search_buffer.
     *
     * Return (start_i, length)
     *  start_i: Index in the search_buffer where the matching starts
     *  length: length of the match.
     */
    fn find_max_match(&self, search_buffer: &[char], pattern: &[char]) -> (usize, usize);

    fn get_search_buffer_size(&self) -> usize;
}

/**
 * Basic implementation of with brute force sequential matching.
 */
pub struct NaiveMatcher {
    pub search_buffer_size: usize,
}

impl Matcher for NaiveMatcher {
    fn find_max_match(&self, search_buffer: &[char], pattern: &[char]) -> (usize, usize) {
        // Length of max match
        let mut match_length_max = 0;
        let mut start_index = 0;

        // loop variables
        let mut match_length;
        for i in 0..search_buffer.len() {
            // Min of (remaining part of search buffer, pattern)
            let max_match_limit = std::cmp::min(search_buffer.len() - i, pattern.len());
            match_length = 0;
            for j in 0..max_match_limit {
                if search_buffer[i + j] != pattern[j] {
                    break;
                }
                match_length += 1;
            }
            if match_length > match_length_max {
                start_index = i;
                match_length_max = match_length;
            }
        }
        (start_index, match_length_max)
    }

    fn get_search_buffer_size(&self) -> usize {
        self.search_buffer_size
    }
}

impl Default for NaiveMatcher {
    fn default() -> NaiveMatcher {
        NaiveMatcher {
            search_buffer_size: 0,
        }
    }
}
