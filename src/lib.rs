use unicode_normalization::UnicodeNormalization;

// Constants
const SCORE_MATCH: i32 = 16;
const SCORE_GAP_START: i32 = -3;
const SCORE_GAP_EXTENSION: i32 = -1;
const BONUS_BOUNDARY: i32 = SCORE_MATCH / 2;
const BONUS_FIRST_CHAR_MULTIPLIER: i32 = 2;

fn normalize_rune(r: char) -> char {
    r.to_lowercase().nfd().next().unwrap_or(r)
}

enum CharClass {
    White,
    Alnum,
    Punct,
}

fn char_class(c: char) -> CharClass {
    if c.is_whitespace() {
        CharClass::White
    } else if c.is_alphanumeric() {
        CharClass::Alnum
    } else {
        CharClass::Punct
    }
}

fn bonus_for(prev_class: &CharClass, curr_class: &CharClass) -> i32 {
    match curr_class {
        CharClass::Alnum => match prev_class {
            CharClass::White => BONUS_BOUNDARY + 2,
            CharClass::Punct => BONUS_BOUNDARY + 1,
            CharClass::Alnum => 0,
        },
        _ => 0,
    }
}

/// Performs a fuzzy match between `text` and `pattern`.
///
/// Returns a tuple containing:
/// - start index of the match in `text`
/// - end index of the match in `text`
/// - score of the match
/// - vector of matched positions in `text`
///
/// If no match is found, returns (-1, -1, 0, vec![]).
pub fn fuzzy_match_v2(
    text: &str,
    pattern: &str,
    case_sensitive: bool,
    normalize: bool,
) -> (isize, isize, i32, Vec<usize>) {
    if pattern.is_empty() {
        return (0, 0, 0, vec![]);
    }

    let text = if !case_sensitive {
        text.to_lowercase()
    } else {
        text.to_string()
    };
    let pattern = if !case_sensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let text: String = if normalize {
        text.chars().map(normalize_rune).collect()
    } else {
        text
    };
    let pattern: String = if normalize {
        pattern.chars().map(normalize_rune).collect()
    } else {
        pattern
    };

    let (m, n) = (pattern.len(), text.len());

    if m > n {
        return (-1, -1, 0, vec![]);
    }

    // Phase 1 & 2: Bonus calculation
    let mut bonus = vec![0; n];
    let mut prev_class = CharClass::White;
    for (i, c) in text.chars().enumerate() {
        let curr_class = char_class(c);
        bonus[i] = bonus_for(&prev_class, &curr_class);
        prev_class = curr_class;
    }

    // Phase 3: Score matrix calculation
    let mut h = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        h[i][0] = SCORE_GAP_START + (i as i32 - 1) * SCORE_GAP_EXTENSION;
    }

    let (mut max_score, mut max_i, mut max_j) = (0, 0, 0);

    let pattern: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = text.chars().collect();

    for i in 1..=m {
        for j in 1..=n {
            let score = if pattern[i - 1] == text[j - 1] {
                let mut score = h[i - 1][j - 1] + SCORE_MATCH;
                if i == 1 {
                    score += bonus[j - 1] * BONUS_FIRST_CHAR_MULTIPLIER;
                } else {
                    score += bonus[j - 1];
                }
                score
            } else {
                std::cmp::max(
                    h[i][j - 1] + SCORE_GAP_EXTENSION,
                    h[i - 1][j] + SCORE_GAP_START,
                )
            };

            h[i][j] = std::cmp::max(0, score);

            if h[i][j] > max_score {
                max_score = h[i][j];
                max_i = i;
                max_j = j;
            }
        }
    }

    if max_score == 0 {
        return (-1, -1, 0, vec![]);
    }

    // Phase 4: Backtracing
    let mut positions = Vec::new();
    let (mut i, mut j) = (max_i, max_j);
    while i > 0 && j > 0 {
        if pattern[i - 1] == text[j - 1] {
            positions.push(j - 1);
            i -= 1;
            j -= 1;
        } else if h[i][j - 1] + SCORE_GAP_EXTENSION == h[i][j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    positions.reverse();

    (
        positions[0] as isize,
        (positions[positions.len() - 1] + 1) as isize,
        max_score,
        positions,
    )
}

/// Performs a fuzzy match between `text` and `pattern` and returns only the score.
///
/// This is a simplified version of `fuzzy_match_v2` that only returns the match score.
pub fn fuzzy_match_score(text: &str, pattern: &str, case_sensitive: bool, normalize: bool) -> i32 {
    let (_, _, score, _) = fuzzy_match_v2(text, pattern, case_sensitive, normalize);
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_v2() {
        let text = "abcdefghijklmnopqrstuvwxyz";
        let pattern = "ace";
        let (start, end, score, positions) = fuzzy_match_v2(text, pattern, false, true);
        assert_eq!(start, 0);
        assert_eq!(end, 5);
        assert!(score > 0);
        assert_eq!(positions, vec![0, 2, 4]);
    }

    #[test]
    fn test_fuzzy_match_score() {
        let text = "algorithm";
        let pattern = "alm";
        let score = fuzzy_match_score(text, pattern, false, true);
        assert!(score > 0);
    }
}
