# rizzer

Rust Fuzzy Matching Library

## Algorithm Description
This library implements a fuzzy string matching algorithm
based on the algorithm used by [fzf](https://github.com/junegunn/fzf).
A very poor port for now, but I'll try to improve it over time.

The algorithm works as follows:

1. It calculates bonus scores for character positions based on their context (e.g., after whitespace or punctuation).
2. It builds a score matrix using dynamic programming, considering matches, gaps, and bonuses.
3. It performs backtracing to find the best matching subsequence.
4. The algorithm supports case-insensitive matching and Unicode normalization.

The matching process assigns higher scores to continuous matches
and matches at word boundaries, making it particularly effective
for searching within longer texts or lists of items.

## API Description

The library exposes two main functions:

1. `fuzzy_match(text: &str, pattern: &str, case_sensitive: bool, normalize: bool) -> (isize, isize, i32, Vec<usize>)`
    - Performs a full fuzzy match between `text` and `pattern`.
    - Returns a tuple containing:
        - Start index of the match
        - End index of the match
        - Match score
        - Vector of matched positions

2. `fuzzy_match_score(text: &str, pattern: &str, case_sensitive: bool, normalize: bool) -> i32`
    - A simplified version that only returns the match score.

Both functions accept the following parameters:
- `text`: The text to search in
- `pattern`: The pattern to search for
- `case_sensitive`: Whether the match should be case-sensitive
- `normalize`: Whether to apply Unicode normalization

Use these functions to implement fuzzy searching in your Rust
applications. The best use I've found is for matching on lists of strings
for autocomplete, result filtering etc.
