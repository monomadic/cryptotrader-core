// use log::info;

#[derive(Debug, Clone)]
pub struct Pair {
    pub base: String,
    pub symbol: String,
    pub price: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Fiat,
    Bitcoin,
    Altcoin,
}

impl AssetType {
    pub fn from_symbol(symbol: &str) -> Self {
        if crate::KNOWN_STABLECOIN_SYMBOLS.contains(&symbol) {
            AssetType::Fiat
        } else if crate::KNOWN_BTC_SYMBOLS.contains(&symbol) {
            AssetType::Bitcoin
        } else {
            AssetType::Altcoin
        }
    }
}

impl Pair {
    /// This is only a guess, based on the stablecoins out there.
    pub fn base_type(&self) -> AssetType {
        AssetType::from_symbol(&self.base)
    }

    pub fn base_is_fiat(&self) -> bool {
        crate::KNOWN_STABLECOIN_SYMBOLS.contains(&(&*self.base))
    }

    pub fn base_is_btc(&self) -> bool {
        crate::KNOWN_BTC_SYMBOLS.contains(&(&*self.base))
    }

    pub fn base_is_alt(&self) -> bool {
        !(self.base_is_fiat() || self.base_is_btc())
    }
}

pub fn filter_pairs_by_asset_type(asset_type: AssetType, pairs: Vec<Pair>) -> Vec<Pair> {
    pairs
        .into_iter()
        .filter({ |p| p.base_type() == asset_type })
        .collect()
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

pub fn find_first_btc_usd_pair(pairs: Vec<Pair>) -> Option<Pair> {
    pairs.into_iter().find(|p| {
        AssetType::from_symbol(&p.symbol) == AssetType::Bitcoin
            && AssetType::from_symbol(&p.base) == AssetType::Fiat
    })
}

pub fn find_first_btc_pair_for_symbol(symbol: &str, pairs: Vec<Pair>) -> Option<Pair> {
    pairs
        .into_iter()
        .find(|p| p.symbol == symbol && AssetType::from_symbol(&p.base) == AssetType::Bitcoin)
}

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn average_pairs(pairs: Vec<Pair>) -> Pair {
    let mut pair = pairs.first().cloned().unwrap();
    let total_price: f64 = pairs.clone().into_iter().map(|p| p.price).sum();
    pair.price = total_price / pairs.len() as f64;
    pair
}

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

pub fn convert_currency(amount: f64, from: Pair, to: Pair) -> Option<f64> {
    Some((from.price / to.price) * amount)
    // if from.symbol == to.symbol {
    //     Some((from.price / to.price) * amount)
    // } else {
    //     None
    // }
}

// pub fn convert_currency_via(amount: f64, from: Pair, to: Pair, via: Pair) -> Option<f64> {
//     // let from_currency = from.price;
//     // let to_currency = via.price;
//     // let via_currency = via.price;

//     // if from.symbol == to.symbol {
//     //     Some((from.price / to.price) * amount)
//     // } else {
//     //     None
//     // }

//     Some(from.price / to.price / via.price)
// }

// pub fn price_in(symbol: &str, base: &str, pairs: Vec<Pair>) -> Option<f64> {

//     if let symbol = find_pair_by_symbol_and_base(symbol, base)

//     let source_pairs = find_all_pairs_by_symbol(symbol);

//     let pairmap = sort_pairs(pairs);

//     if let Some(pair) = orders.entry(symbol) {

//     } else {

//     }
// }
