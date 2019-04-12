#[derive(Debug, Clone)]
pub struct Pair {
    pub base: String,
    pub symbol: String,
    // pub price: f64, // todo remove
}

use crate::models::price::Price;
use std::{fmt, fmt::Display};
impl Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.symbol, self.base)
    }
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

    pub fn pairs_to_pairmap(pairs: Vec<Pair>) -> PairMap {
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

    pub fn base_pairs_for_symbol(symbol: &str, pairs: &Vec<Pair>) -> Vec<Pair> {
        pairs
            .into_iter()
            .filter({ |p| p.symbol == symbol.to_string() })
            .map(|p| p.clone())
            .collect()
    }

    pub fn find_first_btc_pair_for_symbol(symbol: &str, pairs: Vec<Pair>) -> Option<Pair> {
        pairs
            .into_iter()
            .find(|p| p.symbol == symbol && AssetType::from_symbol(&p.base) == AssetType::Bitcoin)
    }

    pub fn find_first_fiat_pair_for_symbol(symbol: &str, pairs: Vec<Pair>) -> Option<Pair> {
        pairs
            .into_iter()
            .find(|p| p.symbol == symbol && AssetType::from_symbol(&p.base) == AssetType::Fiat)
    }

    //    pub fn btc_price(prices: Vec<Price>) -> Option<f64> {
    //        Price::find_first_btc_usd_price(&prices).map(|p| p.price)
    //    }

    // pub fn price_in_fiat(symbol: &str, prices: &Vec<Price>) -> Option<f64> {
    //     Pair::find_first_fiat_pair_for_symbol(symbol, prices.clone()).map(|p| p.price)
    // }

    // pub fn price_in_btc(symbol: &str, prices: &Vec<Price>) -> Option<f64> {
    //     Pair::find_first_btc_pair_for_symbol(symbol, prices.clone()).map(|p| p.price)
    // }
}

pub fn filter_pairs_by_asset_type(asset_type: AssetType, pairs: Vec<Pair>) -> Vec<Pair> {
    pairs
        .into_iter()
        .filter({ |p| p.base_type() == asset_type })
        .collect()
}

// todo: remove
// pub fn find_all_pairs_by_symbol(symbol: &str, pairs: Vec<Pair>) -> Vec<Pair> {
//     Pair::base_pairs_for_symbol(symbol, pairs)
// }

pub fn find_pair_by_symbol_and_base(symbol: &str, base: &str, pairs: Vec<Pair>) -> Option<Pair> {
    pairs
        .into_iter()
        .find({ |p| p.symbol == symbol.to_string() && p.base == base.to_string() })
}

// // todo: remove
// pub fn find_first_btc_usd_pair(pairs: Vec<Pair>) -> Option<Pair> {
//     Pair::find_first_btc_usd_pair(&pairs)
// }

// // todo: remove
// pub fn find_first_btc_pair_for_symbol(symbol: &str, pairs: Vec<Pair>) -> Option<Pair> {
//     Pair::find_first_btc_pair_for_symbol(symbol, pairs)
// }

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn average_pair_price(prices: Vec<Price>) -> f64 {
    let total_price: f64 = prices.iter().map(|p| p.price).sum();
    total_price / prices.len() as f64
}

// todo: remove
pub fn sort_pairs(pairs: Vec<Pair>) -> PairMap {
    Pair::pairs_to_pairmap(pairs)
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

pub fn convert_currency(amount: f64, from: Price, to: Price) -> Option<f64> {
    Some((from.price / to.price) * amount)
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
