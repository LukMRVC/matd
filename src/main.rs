mod string_matching;
use std::fs::File;
use std::io::{BufReader, Read};

use rand::Rng;

fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let text = "GCATCGCAGCAGCTATACAGCAGAGAGGTACG".to_owned();
    let pattern = "GCAGC".to_owned();

    let stupid_matches = string_matching::match_stupid(&text, &pattern);
    dbg!(stupid_matches);

    // automaton comparison
    let pattern = "GCAGC".to_owned();

    let dfa_matches = string_matching::match_dfa(&text, &pattern);
    dbg!(dfa_matches);

    // KMP pattern finding
    let text = "AABAACAADAABAABA".to_owned();
    let pattern = "AABA".to_owned();
    let kmp_matches = string_matching::match_kmp(&text, &pattern);
    dbg!(kmp_matches);

    let dnadatafile = File::open("./data/dna.50MB")?;
    let mut dnareader = BufReader::new(dnadatafile);
    let mut dnadata: Vec<u8> = vec![];
    if let Err(_) = dnareader.read_to_end(&mut dnadata) {
        panic!("Unable to read DNA data");
    }

    let mut rng = rand::thread_rng();
    let mut pattern_words: Vec<String> = vec![];

    for _ in 0..1000 {
        let s = rng.gen_range(0..dnadata.len());
        let l = rng.gen_range(0..50);
        pattern_words.push(String::from_utf8(Vec::from(&dnadata[s..(s + l)])).unwrap());
    }

    {
        let Ok(text) = String::from_utf8(dnadata.clone()) else {
            panic!("Could not convert DNA data to String");
        };
        benchmark_pattern_search(
            &text,
            &pattern_words,
            &string_matching::match_stupid,
            "Stupid",
        );
        benchmark_pattern_search(&text, &pattern_words, &string_matching::match_dfa, "DFA");
        benchmark_pattern_search(&text, &pattern_words, &string_matching::match_kmp, "KMP");
        benchmark_pattern_search(&text, &pattern_words, &string_matching::match_bmh, "BMH");
    }

    let engdatafile = File::open("./data/english.50MB")?;
    let mut engreader = BufReader::new(engdatafile);
    let mut engdata: Vec<u8> = vec![];
    if let Err(_) = engreader.read_to_end(&mut engdata) {
        panic!("Unable to read ENG data");
    }

    let mut pattern_words: Vec<String> = vec![];

    for _ in 0..1000 {
        let s = rng.gen_range(0..engdata.len());
        let l = rng.gen_range(0..10);
        pattern_words.push(String::from_utf8(Vec::from(&engdata[s..(s + l)])).unwrap());
    }

    {
        let text = String::from_utf8(engdata.clone()).unwrap();

        benchmark_pattern_search(
            &text,
            &pattern_words,
            &string_matching::match_stupid,
            "Stupid",
        );
        benchmark_pattern_search(&text, &pattern_words, &string_matching::match_dfa, "DFA");
        benchmark_pattern_search(&text, &pattern_words, &string_matching::match_kmp, "KMP");
        benchmark_pattern_search(&text, &pattern_words, &string_matching::match_bmh, "BMH");
    }

    Ok(())
}

fn benchmark_pattern_search(
    text: &str,
    patterns: &[String],
    func: &dyn Fn(&str, &str) -> Vec<usize>,
    method: &str,
) {
    let start = std::time::Instant::now();
    for pat in patterns.iter() {
        func(&text, pat);
    }
    println!(
        "{method} matching done in {}ms",
        start.elapsed().as_millis()
    );
    let avg = start.elapsed().as_millis() as f32 / patterns.len() as f32;
    println!("AVG {avg}ms per query");
}
