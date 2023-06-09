use simdutf8::basic::from_utf8;
use std::fs::read_dir;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::{fs::File, fs::OpenOptions, path::Path};

const BIG_ALPHA_RANGE: std::ops::RangeInclusive<u8> = 65..=90;
const SMALL_ALPHA_RANGE: std::ops::RangeInclusive<u8> = 97..=122;
const NUM_RANGE: std::ops::RangeInclusive<u8> = 48..=57;

pub fn get_stop_words() -> Vec<String> {
    // Get the stop words
    return stop_words::get(stop_words::LANGUAGE::English);
}

pub fn process(filepath: &Path) -> std::io::Result<()> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut bytes = Vec::<u8>::new();
    reader.read_to_end(&mut bytes)?;

    bytes.retain(|byte| {
        BIG_ALPHA_RANGE.contains(byte)
            || SMALL_ALPHA_RANGE.contains(byte)
            || NUM_RANGE.contains(byte)
            || *byte == 32u8
            || *byte == 10u8
    });

    let Ok(str_data) = from_utf8(&bytes) else {
        panic!("Unable to convert bytes to string");
    };

    let words = str_data.to_lowercase();
    let words = words.split_ascii_whitespace();

    // Get the stop words
    let eng_stop_words = get_stop_words();
    let eng_stop_words: Vec<&str> = eng_stop_words.iter().map(|sw| sw.as_str()).collect();

    let ofpn = format!(
        "data/processed/{}",
        filepath.file_name().unwrap().to_str().unwrap()
    );
    let ofp = Path::new(&ofpn);
    let of = OpenOptions::new().write(true).create(true).open(ofp)?;
    let mut writer = BufWriter::new(of);
    for word in words {
        let word = word.trim();
        if !eng_stop_words.contains(&word) {
            if let Ok(s) = stem::get(word) {
                writer.write_fmt(format_args!("{}\n", s))?;
            } else {
                panic!("Unable to stem word");
            }
        }
    }
    Ok(())
}

pub fn read_dir_files(
    dirpath: &str,
) -> Result<(Vec<PathBuf>), std::boxed::Box<dyn std::error::Error>> {
    let paths = read_dir(dirpath)?;
    let mut files = vec![];
    for path in paths {
        let path = path?;
        if path.file_type()?.is_file() {
            files.push(path.path());
        }
    }

    Ok(files)
}
