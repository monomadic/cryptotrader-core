use std::collections::HashMap;

pub type Price = (String, f64);
pub type Prices = HashMap<String, f64>;


// pub fn find_price(symbol: &str, prices: Prices) -> Option<f64> {
//     prices.iter().get(|c|c.symbol == symbol)

//     // if let Some(price) = prices.iter().find(|c|c.symbol == symbol) {
//     //     Some(price.amount)
//     // } else {
//     //     None
//     // }
// }
