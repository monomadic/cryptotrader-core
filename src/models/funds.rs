use crate::models::asset::Asset;

// deprecate
#[derive(Debug, Clone)]
pub struct Funds {
    pub btc: Option<Asset>,
    pub fiat: Vec<Asset>,
    pub alts: Vec<Asset>,
}

// to be used
#[derive(Debug, Clone)]
pub struct Pair {
    pub asset: Asset,
    pub base: String,
}

pub fn sort_funds(funds: Vec<Asset>) -> Funds {
    let filter:Vec<Asset> = funds.clone().into_iter().filter(|c| c.amount > 0.9).collect();

    Funds {
        btc:    funds.clone().into_iter().find(|c| c.symbol == "BTC"),
        fiat:   filter.clone().into_iter().filter(|c| c.symbol == "USDT" || c.symbol == "TUSD" || c.symbol == "USD").collect(),
        alts:   filter.into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "USD" && c.symbol != "BTC").collect(),
    }
}