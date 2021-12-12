+ Matcher improvements
++ Stop searching if the max length match is found - Done!! Time reduced from 35secs to 0.3secs
++ Optimise search for a match using Knuth-Morris-Pratt method

+ Compressor improvements
++ Traverse the input string with iterators intead of indexes.
++ Avoid extracting vec<chars> from String, is there a more optimal way?

