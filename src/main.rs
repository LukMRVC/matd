mod approx_matching;
mod indexing;
mod preprocessing;
mod string_matching;
use std::collections::btree_set::Intersection;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Read};

use indexing::{append_to_index, create_index, IndexListing};
use rand::Rng;

fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    // run_string_matching()
    // run_preprocessing()
    let mut ivx = create_index();
    let filepaths = preprocessing::read_dir_files("data/processed")?;
    let mut doc_mapping: Vec<String> = vec![];
    let start = std::time::Instant::now();
    for (doc_id, file) in filepaths.iter().enumerate() {
        doc_mapping.push(file.file_name().unwrap().to_str().unwrap().to_owned());
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .map(|l| l.expect("Could not parse line!"))
            .collect();
        ivx = append_to_index(ivx, &lines, doc_id);
    }
    println!("Index built in {}ms", start.elapsed().as_millis());

    let words = vec![
        "johnston".to_owned(),
        "historian".to_owned(),
        "ebook".to_owned(),
    ];

    let listings: Vec<Option<&IndexListing>> = words
        .iter()
        .map(|w| ivx.get(w))
        .filter(|opt| opt.is_some())
        .collect();
    // dbg!(&listings);
    let start = std::time::Instant::now();
    let mut result: Option<IndexListing> = None;
    for listing in listings.iter() {
        let set = listing.unwrap();
        if let Some(intersection) = result {
            result = Some(intersection.intersection(set).cloned().collect());
        } else {
            result = Some(set.clone());
        }
    }

    dbg!(result.unwrap());
    Ok(())
}

fn benchmark_pattern_search(
    text: &str,
    patterns: &[String],
    func: &impl Fn(&str, &str) -> Vec<usize>,
    method: &str,
) {
    let start = std::time::Instant::now();
    for (i, pat) in patterns.iter().enumerate() {
        func(&text, pat);
        println!("{i}")
    }
    println!(
        "{method} matching done in {}ms",
        start.elapsed().as_millis()
    );
    let avg = start.elapsed().as_millis() as f32 / patterns.len() as f32;
    println!("AVG {avg}ms per query");
}

fn run_string_matching() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let text = "GCATCGCAGCAGCTATACAGCAGAGAGGTACG".to_owned();
    let pattern = "GCAGC".to_owned();

    let stupid_matches = string_matching::match_stupid(&text, &pattern);

    // automaton comparison
    let pattern = "GCAGC".to_owned();

    let dfa_matches = string_matching::match_dfa(&text, &pattern);

    // KMP pattern finding
    let text = "AABAACAADAABAABA".to_owned();
    let pattern = "AABA".to_owned();
    let kmp_matches = string_matching::match_kmp(&text, &pattern);

    let dnadatafile = File::open("./data/dna.50MB")?;
    let mut dnareader = BufReader::new(dnadatafile);
    let mut dnadata: Vec<u8> = vec![];
    if let Err(_) = dnareader.read_to_end(&mut dnadata) {
        panic!("Unable to read DNA data");
    }

    let mut rng = rand::thread_rng();
    let mut pattern_words: Vec<String> = vec![];

    for _ in 0..100 {
        let s = rng.gen_range(0..dnadata.len());
        let l = rng.gen_range(5..50);
        pattern_words.push(String::from_utf8(Vec::from(&dnadata[s..(s + l)])).unwrap());
    }

    {
        let Ok(text) = String::from_utf8(dnadata.clone()) else {
            panic!("Could not convert DNA data to String");
        };
        // benchmark_pattern_search(
        //     &text,
        //     &pattern_words,
        //     &string_matching::match_stupid,
        //     "Stupid",
        // );
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

    for _ in 0..100 {
        let s = rng.gen_range(0..engdata.len());
        let l = rng.gen_range(2..10);
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

fn run_preprocessing() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let paths = read_dir("data/gutenberg")?;

    for path in paths {
        let path = path?;
        if path.file_type()?.is_file() {
            preprocessing::process(&path.path())?;
        }
    }

    Ok(())
}
