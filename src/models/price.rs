use crate::models::*;

#[derive(Debug, Clone)]
pub struct Price {
    pub pair: Pair,
    pub price: f64,
}

impl Price {
    pub fn find_first_btc_usd_price(pairs: &Vec<Price>) -> Option<Price> {
        pairs
            .into_iter()
            .find(|p| {
                AssetType::from_symbol(&p.pair.symbol) == AssetType::Bitcoin
                    && AssetType::from_symbol(&p.pair.base) == AssetType::Stablecoin
            })
            .map(|p| p.clone())
    }

    pub fn find_first_btc_price_for_symbol(symbol: &str, prices: Vec<Price>) -> Option<Price> {
        prices.into_iter().find(|p| {
            p.pair.symbol == symbol && AssetType::from_symbol(&p.pair.base) == AssetType::Bitcoin
        })
    }
}
