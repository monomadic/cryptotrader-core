// use log::info;

#[derive(Debug, Clone)]
pub struct Pair {
    pub base: String,
    pub symbol: String,
    pub price: f64,
}

pub fn find_all_pairs_by_symbol(symbol: &str, pairs: Vec<Pair>) -> Vec<Pair> {
    pairs
        .into_iter()
        .filter({ |p| p.symbol == symbol.to_string() })
        .collect()
}

pub fn find_pair_by_symbol_and_base(symbol: &str, base: &str, pairs: Vec<Pair>) -> Option<Pair> {
    pairs
        .into_iter()
        .find({ |p| p.symbol == symbol.to_string() && p.base == base.to_string() })
}

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn sort_pairs(pairs: Vec<Pair>) -> PairMap {
    let mut pairs_by_symbol: PairMap = HashMap::new();

    for pair in pairs {
        if pairs_by_symbol.contains_key(&pair.symbol) {
            if let Some(mutable_pair) = pairs_by_symbol.get_mut(&pair.symbol) {
                mutable_pair.push(pair);
            } else {
            }
        } else {
            pairs_by_symbol.insert(pair.symbol.clone(), vec![pair.clone()]);
        }
    }

    pairs_by_symbol
}

pub fn filter_pairmap_by_symbols(pairs: PairMap, symbols: Vec<&str>) -> PairMap {
    pairs
        .into_iter()
        .filter(|(k, _v)| {
            let key: &str = &k.clone();
            symbols.contains(&key)
        })
        .collect()
}
