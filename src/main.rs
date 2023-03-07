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

fn main() {
    let text = "Text is text".to_owned();
    let pattern = "ext".to_owned();

    // brute force comparison
    let pat_len = pattern.len();
    for i in 0..(text.len() - pattern.len() + 1) {
        if text[i..(i + pat_len)].eq(&pattern) {
            println!("{pattern} found at {i}->{}", i + pat_len);
        }
    }

    // automaton comparison
    let alphabet = ['A', 'C', 'G', 'T'];
    let pattern = "GCAGC".to_owned();

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
    // searching with DFA
    let text = "GCATCGCAGCAGCTATACAGCAGAGAGGTACG";
    let mut state = 0usize;
    for (ci, c) in text.chars().enumerate() {
        let char_alphabet_idx = alphabet.iter().position(|a| *a == c).unwrap();
        state = dfa[state][char_alphabet_idx];
        if state == pattern.len() {
            println!(
                "Pattern {pattern} found at: {} {}",
                ci - pattern.len() + 1,
                &text[(ci - pattern.len() + 1)..(ci + 1)]
            );
        }
    }
}
