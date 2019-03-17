#[derive(Debug, Clone)]
pub struct Depth {
    pub bids: Vec<Offer>,
    pub asks: Vec<Offer>,
}

#[derive(Debug, Clone)]
pub struct Offer {
    pub price: f64,
    pub qty: f64,
}
