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
    let pattern = "GCAGAGAG".to_owned();

    let mut dfa: Vec<Vec<usize>> = vec![vec![0usize; alphabet.len()]; pattern.len()];
    let mut q = "".to_owned();

    for p in pattern.chars() {
        let state = q.len();
        let mut cq = q.clone();
        for (alphabet_idx, a) in alphabet.iter().enumerate() {
            let mut qa = q.clone();
            qa.push(*a);
            if qa.chars().nth(qa.len() - 1).unwrap() == p {
                dfa[state][alphabet_idx] = state + 1;
                cq = qa.clone();
            } else {
                for k in (0..=state).rev() {
                    if k == 0 || pattern.chars().nth(k - 1).unwrap() == *a {
                        let mut i = 0;
                        if k > 0 {
                            let pat_chars: Vec<char> = pattern.chars().collect();
                            while i < k - 1 {
                                if pat_chars[i] != pat_chars[state - k + i + 1] {
                                    break;
                                }
                                i += 1;
                            }
                            if i == k - 1 {
                                dfa[state][alphabet_idx] = k;
                                break;
                            }
                        } else if k == 0 {
                            dfa[state][alphabet_idx] = k;
                            break;
                        }
                    }
                }
            }
        }
        q = cq;
    }
    dbg!(alphabet);
    dbg!(dfa);
}
