use crate::models::*;
use chrono::{offset::Local, prelude::DateTime};

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub pair: Pair,
    pub trade_type: TradeType,
    pub price: f64,
    pub qty: f64,
    pub time: DateTime<Local>,
    pub fee: f64,
    pub fee_symbol: Option<String>,
}

impl Trade {
    /// what the trade cost when bought/sold
    pub fn cost(&self) -> f64 {
        self.price * self.qty
    }

    /// the current value of the trade
    pub fn value(&self) -> f64 {
        self.pair.price * self.qty
    }
}

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

/// average together buys and sells
pub fn average_trades(trades: Vec<Trade>) -> Vec<Trade> {
    let mut grouped_trades = Vec::new();

    let mut current_trade: Trade = trades.first().cloned().unwrap();
    current_trade.qty = 0.0;

    for trade in trades.clone() {
        if trade.trade_type == current_trade.trade_type {
            current_trade.price = trade.price + current_trade.price / trade.qty + current_trade.qty;
            current_trade.qty += trade.qty;
        } else {
            grouped_trades.push(current_trade.clone());
            current_trade = trade.clone();
        }
    }
    grouped_trades.push(current_trade.clone());
    grouped_trades
}

// pub fn averaged_trade(trades: Vec<Trade>) -> Trade {
//     Trade {
//         cost: 0.1,
//         qty: 0.1,
//         buy: trades.first().unwrap().buy,
//     }
// }

/// group trades with the same price together
pub fn group_trades(trades: Vec<Trade>) -> Vec<Trade> {
    let mut grouped_trades = Vec::new();
    let mut current_trade: Trade = trades.first().cloned().unwrap();
    current_trade.qty = 0.0;

    for trade in trades.clone() {
        if trade.price == current_trade.price && trade.trade_type == current_trade.trade_type {
            current_trade.qty += trade.qty;
        } else {
            grouped_trades.push(current_trade.clone());
            current_trade = trade.clone();
        }
    }

    grouped_trades.push(current_trade.clone());
    grouped_trades
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
