pub fn match_ndfa(text: &str, pattern: &str, max_errors: usize) -> bool {
    // let mut indices: Vec<usize> = vec![];
    //             Vec<(state, automaton_ix, pattern_ix, errors)>
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();

    let mut configs: Vec<(usize, usize, usize, usize)> = vec![(1, 0, 0, 0)];

    // neshoda znaku
    // - posun o pozici v automatu a posun v pozici query
    // prazdne slovo
    // - posun o pozici v automatu, v query zustavam na stejne pozici
    // symbol navic
    // - posun v pozici query
    // shoda znaku
    // - posun obou pozic
    let pattern_len = pattern.len();
    while let Some((state, aix, pix, errors)) = configs.pop() {
        if state % (pattern_len + 1) == 0 && aix == text.len() {
            // indices.push(state);
            return true;
        }

        if pix >= pattern_len || aix >= text_chars.len() {
            continue;
        }

        if text_chars[aix] == pattern_chars[pix] {
            configs.push((state + 1, aix + 1, pix + 1, errors));
        } else if errors < max_errors {
            // char replace
            configs.push((state + pattern_len + 2, aix + 1, pix + 1, errors + 1));
            // empty char - insertion
            configs.push((state + pattern_len + 2, aix, pix + 1, errors + 1));
            // char deletion
            configs.push((state + pattern_len + 1, aix + 1, pix, errors + 1));
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ndfa_survey_match() {
        let res = match_ndfa("survey", "murvey", 2);
        assert_eq!(res, true);

        let res = match_ndfa("survey", "ssurvey", 2);
        assert_eq!(res, true);

        let res = match_ndfa("survey", "murvxy", 2);
        assert_eq!(res, true);

        let res = match_ndfa("survey", "murfxy", 2);
        assert_eq!(res, false);
    }

    #[test]
    fn ndfa_bad_match() {
        let _match = match_ndfa("karel", "dale", 2);
        assert_eq!(_match, false, "False positive match");
    }

    #[test]
    fn ndfa_test() {
        let words = vec![
            "karel",
            "robot",
            "kafe",
            "otoman",
            "stul",
            "sysel",
            "hubert",
            "kanape",
            "jurta",
            "nhl",
            "ork",
            "vlak",
            "zidle",
            "afrika",
            "evropa",
            "slon",
            "zebra",
            "saty",
            "auto",
            "autobus",
            "kolo",
            "fontana",
            "opera",
            "rakousko",
            "hora",
            "beh",
            "touha",
            "kamarad",
            "pocitac",
            "procesor",
            "klavesnice",
            "mys",
            "parek",
        ];

        let pattern = "dale";
        let mut matches: Vec<&&str> = vec![];

        for w in words.iter() {
            if match_ndfa(w, pattern, 2) {
                matches.push(w);
            }
        }

        dbg!(matches);
    }
}
