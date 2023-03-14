use std::collections::HashSet;

fn get_next_state(
    current_state: usize,
    qa: &mut String,
    pat_chars: &[char],
    p: char,
    a: &char,
) -> usize {
    // if prefix is same with next alphabet char, simply go to next state
    if qa.chars().nth(qa.len() - 1).unwrap() == p {
        return current_state + 1;
    } else {
        for k in (0..=current_state).rev() {
            // if the biggest possible suffix matches the pattern
            if k == 0 || pat_chars[k - 1] == *a {
                let mut i = 0;
                if k > 0 {
                    // find the biggest suffix, that is also a prefix
                    while i < k - 1 {
                        // when chars are not same, break from loop
                        if pat_chars[i] != pat_chars[current_state - k + i + 1] {
                            break;
                        }
                        i += 1;
                    }
                    // i is equal to next biggest state, assign to DFA
                    if i == k - 1 {
                        return k;
                    }
                } else if k == 0 {
                    // is k is zero, simply go to zero
                    return k;
                }
            }
        }
        0
    }
}

fn build_dfa(alphabet: &[char], pattern: &str) -> Vec<Vec<usize>> {
    let mut dfa: Vec<Vec<usize>> = vec![vec![0usize; alphabet.len()]; pattern.len() + 1];
    let mut q = "".to_owned();
    let pat_chars: Vec<char> = pattern.chars().collect();

    // build the DFA
    for p in pattern.chars() {
        let state = q.len();
        for (alphabet_idx, a) in alphabet.iter().enumerate() {
            let mut qa = q.clone();
            qa.push(*a);
            dfa[state][alphabet_idx] = get_next_state(state, &mut qa, &pat_chars, p, a);
        }
        q.push(p);
    }

    let state = q.len();
    for (alphabet_idx, a) in alphabet.iter().enumerate() {
        let mut qa = q.clone();
        qa.push(*a);
        dfa[state][alphabet_idx] = get_next_state(state, &mut qa, &pat_chars, '$', a);
    }
    dfa
}

pub fn match_dfa(text: &str, pattern: &str) -> Vec<usize> {
    let alphabet: Vec<char> = Vec::from_iter(HashSet::<char>::from_iter(text.chars().into_iter()));
    let mut indices: Vec<usize> = vec![];
    let dfa = build_dfa(&alphabet, pattern);
    let mut char_alphabet_indexes = [0; 255];
    for (ai, a) in alphabet.iter().enumerate() {
        char_alphabet_indexes[*a as usize] = ai;
    }
    let mut state = 0usize;
    for (ci, c) in text.chars().enumerate() {
        let char_alphabet_idx = char_alphabet_indexes[c as usize];
        state = dfa[state][char_alphabet_idx];
        if state == pattern.len() {
            indices.push(ci - pattern.len() + 1);
            // println!(
            //     "Pattern {pattern} found at: {} {}",
            //     ci - pattern.len() + 1,
            //     &text[(ci - pattern.len() + 1)..(ci + 1)]
            // );
        }
    }
    indices
}

pub fn match_stupid(text: &str, pattern: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    let pat_len = pattern.len();
    for i in 0..(text.len() - pattern.len() + 1) {
        if text[i..(i + pat_len)].eq(pattern) {
            // println!("{pattern} found at {i}->{}", i + pat_len);
            indices.push(i);
        }
    }
    indices
}

pub fn match_kmp(text: &str, pattern: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    let mut lsp = vec![0usize; pattern.len()];

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut len = 0usize;
    let mut i = 1;
    while i < pattern.len() {
        if pattern_chars[i] == pattern_chars[len] {
            len += 1;
            lsp[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lsp[len - 1];
            } else {
                lsp[i] = 0;
                i += 1;
            }
        }
    }

    let mut i = 0;
    let mut j = 0;
    while i < text.len() {
        if text_chars[i] == pattern_chars[j] {
            i += 1;
            j += 1;
            if j == pattern.len() {
                indices.push(i - j);
                j = lsp[j - 1];
            }
        } else if i < text.len() {
            if j != 0 {
                j = lsp[j - 1];
            } else {
                i += 1;
            }
        }
    }

    indices
}

pub fn match_bmh(text: &str, pattern: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = vec![];
    // bad character match bcm
    let mut bcm = [pattern.len(); 256];
    // bmh is the index of last character occurence in pattern
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    for (ci, c) in pattern_chars.iter().take(pattern.len() - 1).enumerate() {
        bcm[*c as usize] = pattern.len() - ci - 1;
    }

    // match pattern from behind. On first inequality, jump according to bmh table
    // if character is not found in pattern jump of length of pattern |pattern|
    let mut i = 0;
    while i <= text.len() - pattern.len() {
        let c = text_chars[i + pattern.len() - 1];
        if pattern_chars[pattern.len() - 1] == c
            && pattern_chars.eq(&text_chars[i..(i + pattern.len())])
        {
            indices.push(i);
        }
        i += bcm[c as usize]
    }

    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horspool_is_fine() {
        let text = "GCATCGCAGAGAGTATACAGTACG".to_owned();
        let pattern = "GCAGAGAG".to_owned();
        let res = match_bmh(&text, &pattern);
        assert_eq!(res, vec![5]);

        let text = "Hawaii has some nice beaches and pineapples".to_owned();
        let pattern = "pine".to_owned();
        let res = match_bmh(&text, &pattern);
        assert_eq!(res, vec![33]);
    }

    #[test]
    fn kmp_is_fine() {
        let text = "GCATCGCAGAGAGTATACAGTACG".to_owned();
        let pattern = "GCAGAGAG".to_owned();
        let res = match_kmp(&text, &pattern);
        assert_eq!(res, vec![5]);

        let text = "Hawaii has some nice beaches and pineapples".to_owned();
        let pattern = "pine".to_owned();
        let res = match_kmp(&text, &pattern);
        assert_eq!(res, vec![33]);
    }

    #[test]
    fn dfa_is_fine() {
        let text = "GCATCGCAGAGAGTATACAGTACG".to_owned();
        let pattern = "GCAGAGAG".to_owned();
        let res = match_dfa(&text, &pattern);
        assert_eq!(res, vec![5]);

        let text = "Hawaii has some nice beaches and pineapples".to_owned();
        let pattern = "pine".to_owned();
        let res = match_dfa(&text, &pattern);
        assert_eq!(res, vec![33]);
    }
}
