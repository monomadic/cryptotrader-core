use crate::models::pair::Pair;
#[derive(Debug, Clone)]
pub struct BookTicker {
    pub pair: Pair,
    pub bid_price: f64,
    pub bid_qty: f64,
    pub ask_price: f64,
    pub ask_qty: f64,
}

impl BookTicker {
    pub fn find_by_symbol_and_base(
        symbol: &str,
        base: &str,
        tickers: Vec<BookTicker>,
    ) -> Option<BookTicker> {
        tickers
            .into_iter()
            .find({ |t| t.pair.symbol == symbol.to_string() && t.pair.base == base.to_string() })
    }

    pub fn filter_by_symbol(symbol: &str, tickers: Vec<BookTicker>) -> Vec<BookTicker> {
        tickers
            .into_iter()
            .filter({ |t| t.pair.symbol == symbol.to_string() })
            .collect()
    }
}
