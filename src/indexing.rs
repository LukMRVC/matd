use std::collections::BTreeSet;

use fxhash::FxHashMap;

pub type IndexListing = BTreeSet<usize>;
type InvertedIndex = FxHashMap<String, IndexListing>;

pub fn create_index() -> InvertedIndex {
    FxHashMap::<String, IndexListing>::default()
}

#[inline(always)]
pub fn append_to_index(mut ivx: InvertedIndex, terms: &[String], doc_id: usize) -> InvertedIndex {
    for term in terms.iter() {
        ivx.entry(term.clone())
            .and_modify(|set| {
                set.insert(doc_id);
            })
            .or_insert_with(|| {
                let mut s = BTreeSet::default();
                s.insert(doc_id);
                s
            });
    }

    ivx
}
