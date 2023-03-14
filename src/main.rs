mod string_matching;

fn main() {
    let text = "GCATCGCAGCAGCTATACAGCAGAGAGGTACG".to_owned();
    let pattern = "GCAGC".to_owned();

    let stupid_matches = string_matching::match_stupid(&text, &pattern);
    dbg!(stupid_matches);

    // automaton comparison
    let alphabet = ['A', 'C', 'G', 'T'];
    let pattern = "GCAGC".to_owned();

    let dfa_matches = string_matching::match_dfa(&text, &pattern, &alphabet);
    dbg!(dfa_matches);

    // KMP pattern finding
    let text = "AABAACAADAABAABA".to_owned();
    let pattern = "AABA".to_owned();
    let kmp_matches = string_matching::match_kmp(&text, &pattern);
    dbg!(kmp_matches);
}
