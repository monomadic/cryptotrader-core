use crate::models::*;

#[derive(Debug, Clone)]
pub struct Price {
    pub pair: Pair,
    pub price: f64,
}

impl Price {
    pub fn find_first_btc_usd_price(prices: &Vec<Price>) -> Option<Price> {
        prices
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

//pub struct Prices(Vec<Price>);

pub trait PriceUtils {
    /// strips price information from Vec returning simple Pairs.
    fn to_pairs(&self) -> Vec<Pair>;
    fn price_for(&self, pair: Pair) -> Option<f64>;
    fn filter_by(&self, symbol: &str) -> Vec<Price>;
    fn first_btc_price_for(&self, symbol: &str) -> Option<Price>;
    fn price_of(&self, symbol: &str, base: &str) -> Option<f64>;
}

impl PriceUtils for Vec<Price> {
    fn to_pairs(&self) -> Vec<Pair> {
        self.iter().map(|price| price.pair.clone()).collect()
    }

    fn price_for(&self, pair: Pair) -> Option<f64> {
        self.into_iter().find(|p| p.pair == pair).map(|p| p.price)
    }

    fn filter_by(&self, symbol: &str) -> Vec<Price> {
        self.into_iter()
            .filter(|p| p.pair.symbol.to_uppercase() == symbol.to_uppercase())
            .map(|p| p.clone())
            .collect()
    }

    fn first_btc_price_for(&self, symbol: &str) -> Option<Price> {
        self.into_iter()
            .find(|p| {
                p.pair.symbol == symbol
                    && AssetType::from_symbol(&p.pair.base) == AssetType::Bitcoin
            })
            .map(|p| p.clone())
    }

    fn price_of(&self, symbol: &str, base: &str) -> Option<f64> {
        if symbol == base {
            return Some(1.0);
        };

        self.price_for(Pair::new(symbol, base)).or(self
            .price_for(Pair::new(base, symbol))
            .map(|price| 1.0 / price))
    }
}
