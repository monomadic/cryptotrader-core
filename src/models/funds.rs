use models::asset::Asset;

#[derive(Debug, Clone)]
pub struct Funds {
    pub btc: Option<Asset>,
    pub fiat: Vec<Asset>,
    pub alts: Vec<Asset>,
}

// impl Funds {
//     pub fn calculate_totals(&mut self) -> Self {
//         let btc_value = if let Some(ref mut b) = self.btc {
//             b.amount
//         } else {
//             0.0
//         };

//         // let total_usd_price:f64 = self.alts.iter().map(|a| a.value_in_usd.unwrap_or(0.0) * a.amount).sum();
//         // let total_alt_price_in_btc:f64 = self.alts.iter().map(|a| a.value_in_btc.unwrap_or(0.0) * a.amount).sum();

//         // self.total_value_in_usd = total_usd_price;
//         // self.total_value_in_btc = total_alt_price_in_btc + btc_value;

//         self.clone()
//     }
// }

// impl Funds {
//     // pub fn total_btc(&self, btc_price: f64, prices: ::models::Prices) -> f64 {
//     //     let alts:f64 = self.alts.iter().map(|a| { a.amount * price_in_btc(a.clone().symbol, prices.clone()).unwrap_or(0.0) }).sum();
//     //     let fiat:f64 = self.fiat.iter().map(|a| a.amount / btc_price).sum();
//     //     self.btc.clone().unwrap_or_default().amount + alts + fiat
//     // }
// }

pub fn sort_funds(funds: Vec<Asset>) -> Funds {
    let filter:Vec<Asset> = funds.clone().into_iter().filter(|c| c.amount > 0.9).collect();

    Funds {
        btc:    funds.clone().into_iter().find(|c| c.symbol == "BTC"),
        fiat:   filter.clone().into_iter().filter(|c| c.symbol == "USDT" || c.symbol == "TUSD" || c.symbol == "USD").collect(),
        alts:   filter.into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "USD" && c.symbol != "BTC").collect(),
    }
}