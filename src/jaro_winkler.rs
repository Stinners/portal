
use std::cmp::{max, min};

pub fn jaro(s1: &str, s2: &str) -> f64 {

    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let search_range = max(len1, len2) / 2 - 1;

    let mut match_index = 0;
    let mut common_chars: f64 = 0.0;
    let mut transpositions: f64 = 0.0;
    for (i, s1_char) in s1.chars().enumerate() {
        let min_char = max(0, i.saturating_sub(search_range));
        let max_char = min(i + search_range, len2);

        // Iterate over all chars in s2 between min_char and max_char
        let s2_window = s2.chars()
                          .take(max_char)
                          .skip(min_char);
        for (j, s2_char) in (min_char..).zip(s2_window) {
            if s1_char == s2_char {
                common_chars += 1.0;
                if j < match_index {
                    transpositions += 1.0;
                }
                match_index = j;
                break;
            }
        }
    }

    if common_chars == 0.0 {
        return 0.0;
    }

    ( 1. / 3. ) * (
     common_chars / (len1 as f64) +  
     common_chars / (len2 as f64) + 
     (common_chars - transpositions) / common_chars)
}

/* Calculates the common prefix between two strings where 
 * max_chars is the max number of characters to check 
 */
fn common_prefix(s1: &str, s2: &str, max_chars: usize) -> usize {
    s1.chars()
      .zip(s2.chars())
      .take(max_chars)
      .take_while(|(c1, c2)| c1 == c2)
      .count()
}

pub fn jaro_winkler(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }

    let jaro_sim = jaro(s1, s2);
    let prefix_len = common_prefix(s1, s2, 4) as f64;

    jaro_sim + (prefix_len * 0.1 * (1.0 - jaro_sim))
}

#[cfg(test)] 
mod tests {
    use jaro_winkler::jaro;
    #[test]
    fn jaro_distance() {
        let example1 = jaro("MARTHA", "MARHTA");
        assert!(0.944 < example1 && 0.945 > example1);

        let example2 = jaro("DIXON", "DICKSONX");
        assert!(0.766 < example2 && 0.767 > example2);

        let example3 = jaro("JELLYFISH", "SMELLYFISH");
        assert!(0.895 < example3 && 0.897 > example3);
    }
}

