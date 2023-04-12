mod approx_matching;
mod indexing;
mod preprocessing;
mod string_matching;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Read};

use indexing::{append_to_index, create_index, IndexListing};
use rand::Rng;

use crate::preprocessing::get_stop_words;

fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let filepaths = preprocessing::read_dir_files("data/processed")?;

    let doc_count = filepaths.len();
    let mut doc_mapping: Vec<String> = vec![];
    let mut term_freq: Vec<Vec<u32>> = vec![];
    let mut term_idx_map: HashMap<String, usize> = HashMap::default();
    println!("Please enter your query: ");
    let mut queryinput: String = String::new();
    std::io::stdin().read_line(&mut queryinput)?;

    // let start = std::time::Instant::now();
    for (doc_id, file) in filepaths.iter().enumerate() {
        doc_mapping.push(file.file_name().unwrap().to_str().unwrap().to_owned());
        // println!("{doc_id}: {}", doc_mapping.last().unwrap());
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let terms: Vec<String> = reader
            .lines()
            .map(|l| l.expect("Could not parse line!"))
            .collect();

        // column represents a document
        // row represents a word

        for term in terms.iter() {
            if let Some(term_idx) = term_idx_map.get(term) {
                term_freq[*term_idx][doc_id] += 1;
            } else {
                term_freq.push(vec![0; doc_count]);
                let term_idx = term_freq.len() - 1;
                term_idx_map.insert(term.clone(), term_idx);
                term_freq[term_idx][doc_id] = 1;
            }
        }
    }

    // let start = std::time::Instant::now();
    let inverse_doc_freq: Vec<f32> = term_freq
        .par_iter()
        .map(|documents_freq| {
            let docs_containing_term = documents_freq
                .iter()
                .filter(|occurences| **occurences > 0)
                .count();
            (doc_count as f32 / docs_containing_term as f32).log10()
        })
        .collect();

    let tf_idf: Vec<Vec<f32>> = term_freq
        .par_iter()
        .enumerate()
        .map(|(term_id, documents_freq)| {
            documents_freq
                .iter()
                .map(|doc_freq| (*doc_freq as f32) * inverse_doc_freq[term_id])
                .collect::<Vec<f32>>()
        })
        .collect();

    fn dot_product(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
        v1.iter().zip(v2.iter()).map(|(f1, f2)| f1 * f2).sum()
    }

    fn normalize(v1: &Vec<f32>) -> Vec<f32> {
        let magnitude = v1.iter().fold(0f32, |acc, f| acc + (f * f));
        if magnitude.eq(&0f32) {
            return v1.clone();
        }
        v1.iter().map(|f| f / magnitude).collect()
    }

    fn normalize_u32(v1: &Vec<u32>) -> Vec<f32> {
        let magnitude = v1.iter().fold(0f32, |acc, f| acc + (f * f) as f32);
        if magnitude.eq(&0f32) {
            return v1.iter().map(|f| *f as f32).collect();
        }
        v1.iter().map(|f| *f as f32 / magnitude).collect()
    }

    fn vector_similiarity(
        vec: &Vec<Vec<f32>>,
        doc_mapping: &Vec<String>,
        collection_name: String,
        doc_or_word: String,
        term_idx_map: &HashMap<String, usize>,
    ) {
        for (norm_id, norm_doc) in vec.iter().enumerate().take(150) {
            let most_sim_doc = vec
                .iter()
                .enumerate()
                .filter(|(id, _)| *id != norm_id)
                .map(|(id, doc)| (id, dot_product(norm_doc, doc)))
                .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
            if let Some((doc_id, similiarity)) = most_sim_doc {
                if (doc_or_word.eq_ignore_ascii_case("document")) {
                    println!(
                        "{collection_name}: {f1} most similiar document is {f2} with score {similiarity}",
                        f1 = doc_mapping[norm_id],
                        f2 = doc_mapping[doc_id]
                    );
                } else {
                    let kv = term_idx_map
                        .iter()
                        .find(|(_, word_id)| **word_id == norm_id);
                    let w2 = term_idx_map.iter().find(|(_, word_id)| **word_id == doc_id);
                    println!(
                        "{collection_name}: {f1} most similiar word is {f2} with score {similiarity}",
                        f1 = kv.unwrap().0,
                        f2 = w2.unwrap().0
                    );
                }
            } else {
                panic!("No similiarity was found!");
            }
        }
    }

    let tf_row_norm: Vec<Vec<f32>> = term_freq.iter().map(normalize_u32).collect();
    let total_doc_count = term_freq[0].len();
    let mut tf_col_norm: Vec<Vec<f32>> = vec![];
    let mut tfidf_col_norm: Vec<Vec<f32>> = vec![];
    let tfidf_row_norm: Vec<Vec<f32>> = tf_idf.iter().map(normalize).collect();

    for col in 0..total_doc_count {
        let mut col_vec: Vec<u32> = vec![];
        let mut col_vec_idf: Vec<f32> = vec![];
        for row in 0..term_freq.len() {
            col_vec.push(term_freq[row][col].clone());
            col_vec_idf.push(tf_idf[row][col].clone());
        }
        tf_col_norm.push(normalize_u32(&col_vec));
        tfidf_col_norm.push(normalize(&col_vec_idf));
    }

    vector_similiarity(
        &tfidf_col_norm,
        &doc_mapping,
        "TFIDF".to_owned(),
        "document".to_owned(),
        &term_idx_map,
    );

    vector_similiarity(
        &tf_col_norm,
        &doc_mapping,
        "TF".to_owned(),
        "document".to_owned(),
        &term_idx_map,
    );

    vector_similiarity(
        &tfidf_row_norm,
        &doc_mapping,
        "TFIDF".to_owned(),
        "word".to_owned(),
        &term_idx_map,
    );

    vector_similiarity(
        &tf_row_norm,
        &doc_mapping,
        "TF".to_owned(),
        "word".to_owned(),
        &term_idx_map,
    );

    let stopwords = get_stop_words();
    let stopwords: Vec<&str> = stopwords.iter().map(|sw| sw.as_str()).collect();

    let query_terms: Vec<String> = queryinput
        .split_ascii_whitespace()
        .filter(|term| !stopwords.contains(term))
        .map(|term| stem::get(term).unwrap())
        .collect();
    dbg!(&query_terms);
    // scores is a vector of scores per document, tuple of (document_id, score)
    let mut scores: Vec<(usize, f32)> = vec![];
    for doc_id in 0..doc_count {
        let mut score = 0f32;
        for qt in query_terms.iter() {
            if let Some(term_id) = term_idx_map.get(qt) {
                score += tf_idf[*term_id][doc_id];
            }
        }
        scores.push((doc_id, score));
    }
    scores.sort_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap());
    println!("Highest scores docuemnts: ");
    for (doc_id, score) in scores.iter().rev().take(10) {
        println!(
            "Document {doc_id} - {filename}: {score}",
            filename = doc_mapping[*doc_id]
        );
    }

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

fn run_inverted_index_matching() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
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
