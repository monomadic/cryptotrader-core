#[derive(Debug, Clone)]
pub struct Pair {
    pub base: String,
    pub symbol: String,
    pub price: f64,
}

// pub trait FindPair {
//     pub fn find_pair(&self, pair: &str, base: &str) -> Option<Pair>;
// }

// impl FindPair for Iterator<Pair> {

// }

// pub fn find_pair(pair: &str, base: &str, pairs: Vec<Pair>) -> Option<Pair> {
//     let result = pairs.into_iter().find_map(|p| p.symbol == pair && p.base == base)
// }
