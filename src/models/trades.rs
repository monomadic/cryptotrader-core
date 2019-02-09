use crate::models::*;

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub pair: Pair,
    pub trade_type: TradeType,
    pub price: f64,
    pub qty: f64,
    pub time: u64,
}

impl Trade {
    pub fn cost(&self) -> f64 {
        self.price * self.qty
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::models::Trade;

//     #[test]
//     fn test_sum_qty() {
//         assert_eq!(
//             20.0,
//             sum_qty(vec![
//                 Trade {
//                     cost: 10.,
//                     qty: 10.0,
//                     buy: true
//                 },
//                 Trade {
//                     cost: 20.,
//                     qty: 10.0,
//                     buy: true
//                 },
//             ])
//         );
//         assert_eq!(
//             10.0,
//             sum_qty(vec![
//                 Trade {
//                     cost: 10.,
//                     qty: 10.0,
//                     buy: true
//                 },
//                 Trade {
//                     cost: 20.,
//                     qty: 10.0,
//                     buy: true
//                 },
//                 Trade {
//                     cost: 10.,
//                     qty: 10.0,
//                     buy: false
//                 },
//             ])
//         );
//     }

//     #[test]
//     fn test_average_cost() {
//         assert_eq!(
//             15.0,
//             average_cost(vec![
//                 Trade {
//                     cost: 10.,
//                     qty: 1.0,
//                     buy: true
//                 },
//                 Trade {
//                     cost: 20.,
//                     qty: 1.0,
//                     buy: true
//                 },
//             ])
//         );
//         assert_eq!(
//             17.5,
//             average_cost(vec![
//                 Trade {
//                     cost: 10.,
//                     qty: 10.0,
//                     buy: true
//                 },
//                 Trade {
//                     cost: 20.,
//                     qty: 30.0,
//                     buy: true
//                 },
//             ])
//         );
//     }

//     #[test]
//     fn test_average_trades() {
//         let result = average_trades(vec![
//             Trade {
//                 cost: 10.,
//                 qty: 10.0,
//                 buy: true,
//             },
//             Trade {
//                 cost: 20.,
//                 qty: 10.0,
//                 buy: true,
//             },
//         ]);
//         assert_eq!(1, result.len());

//         let test_value = result.first().unwrap();
//         assert_eq!(15.0, test_value.cost);
//         assert_eq!(20.0, test_value.qty);
//         assert_eq!(true, test_value.buy);
//     }
// }

pub fn sum_qty(trades: Vec<Trade>) -> f64 {
    trades
        .into_iter()
        .map(|trade| match trade.trade_type {
            TradeType::Buy => trade.qty,
            TradeType::Sell => -trade.qty,
        })
        .sum()
}

pub fn sum_cost(trades: Vec<Trade>) -> f64 {
    trades
        .into_iter()
        .map(|trade| match trade.trade_type {
            TradeType::Buy => trade.price * trade.qty,
            TradeType::Sell => -(trade.price * trade.qty),
        })
        .sum()
}

pub fn average_cost(trades: Vec<Trade>) -> f64 {
    let average: f64 = trades
        .clone()
        .into_iter()
        .map(|trade| trade.qty * trade.price)
        .sum();
    average / sum_qty(trades)
}

// /// average together buys and sells into 1 reduction each
// pub fn average_trades(trades: Vec<Trade>) -> Vec<Trade> {
//     let mut grouped_trades = Vec::new();
//     let mut current_trade = Trade {
//         cost: trades.first().unwrap().price,
//         qty: 0.0,
//         buy: trades.first().unwrap().buy,
//     };

//     for trade in trades.clone() {
//         if trade.buy == current_trade.buy {
//             current_trade.cost = ((trade.cost * trade.qty)
//                 + (current_trade.cost * current_trade.qty))
//                 / (trade.qty + current_trade.qty);
//             current_trade.qty += trade.qty;
//         } else {
//             grouped_trades.push(current_trade.clone());
//             current_trade = trade.clone();
//         }
//     }
//     grouped_trades.push(current_trade.clone());
//     grouped_trades
// }

// pub fn averaged_trade(trades: Vec<Trade>) -> Trade {
//     Trade {
//         cost: 0.1,
//         qty: 0.1,
//         buy: trades.first().unwrap().buy,
//     }
// }

// // group trades into buys and sells
// pub fn group_trades(trades: Vec<Trade>) -> Vec<Trade> {
//     let mut grouped_trades = Vec::new();
//     let mut current_trade = Trade {
//         cost: trades.first().unwrap().cost,
//         qty: 0.0,
//         buy: trades.first().unwrap().buy,
//     };

//     for trade in trades.clone() {
//         if trade.cost == current_trade.cost && trade.buy == current_trade.buy {
//             current_trade.qty += trade.qty;
//         } else {
//             grouped_trades.push(current_trade.clone());
//             current_trade = trade.clone();
//         }
//     }
//     grouped_trades.push(current_trade.clone());
//     grouped_trades
// }
