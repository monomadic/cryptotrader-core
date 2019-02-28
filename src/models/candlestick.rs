#[derive(Debug, Clone)]
pub struct Candlestick {
    pub open_time: i32,
    pub open_price: f64,
    pub close_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub volume: f64,
    pub number_of_trades: u64,
}
