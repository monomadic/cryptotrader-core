use crate::models::*;
use crate::utils::average;
// use crate::utils::*;
use chrono::{offset::Local, prelude::DateTime};

#[derive(Debug, Clone)]
pub struct Trade {
    pub fee: f64,
    pub fee_symbol: Option<String>,
    pub id: String,
    pub pair: Pair,
    pub sale_price: f64,
    pub qty: f64,
    pub time: DateTime<Local>,
    pub trade_type: TradeType,
}

#[derive(Debug, Clone)]
pub struct TradePair {
    buy: Trade,
    sell: Option<Trade>,
}

impl TradePair {
    pub fn to_vec(&self) -> Vec<Trade> {
        if let Some(sell) = self.sell.clone() {
            vec![self.buy.clone(), sell]
        } else {
            vec![self.buy.clone()]
        }
    }
}

impl Trade {
    // pub fn entry_price(&self) -> f64 {
    //     self.price
    // }

    /// what the trade cost when bought/sold
    pub fn sale_cost(&self) -> f64 {
        self.qty * self.sale_price
    }

    /// the current value of the trade
    // pub fn current_cost(&self) -> f64 {
    //     // self.qty * self.current_price
    // }

    // pub fn current_profit(&self) -> f64 {
    //     match self.trade_type {
    //         TradeType::Buy => self.current_cost() - self.sale_cost(),
    //         TradeType::Sell => self.sale_cost() - self.current_cost(),
    //     }
    // }

    // pub fn current_profit_as_percent(&self) -> f64 {
    //     // log::info!("{} {}, {}", self.trade_type, self.entry_price(), self.current_price());

    //     match self.trade_type {
    //         TradeType::Buy => price_percent(self.sale_price, self.current_price),
    //         TradeType::Sell => price_percent(self.current_price, self.sale_price),
    //     }
    // }

    // ------------------------------------------------------------------------------------------

    // // TODO ALIAS - delete
    // pub fn profit_as_percent(&self) -> f64 {
    //     self.current_profit_as_percent()
    // }

    // // // TODO ALIAS - delete
    // // pub fn cost(&self) -> f64 {
    // //     self.entry_cost()
    // // }

    // // TODO ALIAS - delete
    // pub fn value(&self) -> f64 {
    //     self.current_cost()
    // }

    // // TODO ALIAS - delete
    // pub fn profit(&self) -> f64 {
    //     self.current_profit()
    // }

    // grouping strategy that attempts to group until an asset reaches zero. buggy at the moment.
    pub fn group_by_qty(trades: &Vec<Trade>, qty: f64) -> Vec<Trade> {
        let trades: Vec<Trade> = trades.iter().cloned().rev().collect();
        let mut position_trades: Vec<Trade> = Vec::new();
        let mut remaining_qty = qty;
        // let trade_type = trades.first().expect("need a trade type").

        for trade in trades.clone() {
            if remaining_qty.round() <= 0.0 {
                break;
            }

            match trade.trade_type {
                TradeType::Buy => {
                    remaining_qty = remaining_qty - trade.qty;
                }
                TradeType::Sell => {
                    remaining_qty = remaining_qty + trade.qty;
                }
            };

            position_trades.push(trade.clone());
        }

        position_trades
    }

    // grouping strategy that collapses trades together until the last buy/sell
    pub fn group_by_price(trades: Vec<Trade>) -> Vec<Trade> {
        group_trades_by_price(trades)
    }

    /// group trades into buy-sell buy-sell buy-sell
    pub fn group_by_trade_pair(trades: Vec<Trade>) -> Vec<TradePair> {
        let mut trade_pairs: Vec<TradePair> = Vec::new();

        // organise trades by time
        let mut trades = trades.clone();
        trades.sort_by(|a, b| a.time.cmp(&b.time));

        // group trades into vec<buy, sell>
        trades = group_and_average_trades_by_trade_type(trades);

        // reverse the list
        let mut trades: Vec<Trade> = trades.into_iter().rev().collect();

        while let Some(trade) = trades.pop() {
            if trade.trade_type == TradeType::Buy {
                trade_pairs.push(TradePair {
                    buy: trade,
                    sell: trades.pop(),
                })
            }
        }

        // let mut current_trade_pair: TradePair = TradePair {
        //     buy: trades.first().unwrap().clone(),
        //     sell: None,
        // }; // fix this

        // while let Some(last_trade) = trades.pop() {
        //     match last_trade.trade_type {
        //         TradeType::Buy => {
        //             current_trade_pair = TradePair {
        //                 buy: last_trade,
        //                 sell: None,
        //             }
        //         }
        //         TradeType::Sell => {
        //             current_trade_pair.sell = Some(last_trade);
        //             trade_pairs.push(current_trade_pair.clone());
        //         }
        //     }
        // }

        trade_pairs
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
            TradeType::Buy => trade.sale_price * trade.qty,
            TradeType::Sell => -(trade.sale_price * trade.qty),
        })
        .sum()
}

pub fn average_cost(trades: Vec<Trade>) -> f64 {
    let average: f64 = trades
        .clone()
        .into_iter()
        .map(|trade| trade.qty * trade.sale_price)
        .sum();
    average / sum_qty(trades)
}

/// average together buys and sells
pub fn group_and_average_trades_by_trade_type(trades: Vec<Trade>) -> Vec<Trade> {
    group_trades_by_trade_type(trades)
        .into_iter()
        .map(|trade_group| average_trades(trade_group))
        .collect()
}

pub fn average_trades(trades: Vec<Trade>) -> Trade {
    let first_trade: Trade = trades.first().cloned().unwrap();
    let trades_iter = trades.into_iter();
    let qty = trades_iter.clone().map(|t| t.qty).sum();

    //    let average_price: f64 = trades_iter
    //        .clone()
    //        .map(|t| t.sale_price * t.qty)
    //        .sum::<f64>()
    //        / qty;

    let average_price = average(&trades_iter.clone().map(|t| t.sale_price).collect());
    // let pairs: Vec<Pair> = trades_iter.clone().map(|t| t.pair).collect();
    let id = trades_iter
        .clone()
        .map(|t| t.id)
        .collect::<Vec<String>>()
        .join(", ");

    Trade {
        id,
        pair: first_trade.pair,
        trade_type: first_trade.trade_type,
        sale_price: average_price,
        // current_price: average(trades_iter.clone().map(|t| t.current_price).collect()),
        qty,
        time: first_trade.time,
        fee: trades_iter.map(|t| t.fee).sum(),
        fee_symbol: first_trade.fee_symbol.clone(),
    }
}

pub fn group_trades_by_trade_type(trades: Vec<Trade>) -> Vec<Vec<Trade>> {
    // TODO: check that the symbols are the same when grouping
    let mut trade_group: Vec<Trade> = Vec::new();
    let mut grouped_trades: Vec<Vec<Trade>> = Vec::new();

    if trades.is_empty() {
        return grouped_trades;
    }

    let mut current_trade_type: TradeType = trades.first().unwrap().trade_type;
    let mut ungrouped_trades: Vec<Trade> = trades.into_iter().rev().collect();

    while let Some(trade) = ungrouped_trades.pop() {
        if trade.trade_type == current_trade_type {
            trade_group.push(trade);
        } else {
            current_trade_type = trade.trade_type.clone();
            grouped_trades.push(trade_group.clone());
            trade_group = Vec::new();
            trade_group.push(trade);
        }
    }

    grouped_trades.push(trade_group.clone());
    grouped_trades
}

/// group trades into buy-sell buy-sell buy-sell
pub fn _group_trades_by_trade_type_pair(trades: Vec<Trade>) -> Vec<Vec<Trade>> {
    let mut positions = Vec::new();
    let mut current_trades: Vec<Trade> = Vec::new();
    let mut trades: Vec<Trade> = trades.into_iter().rev().collect();

    while let Some(last_trade) = trades.pop() {
        match last_trade.trade_type {
            TradeType::Buy => {
                // if the list contains sells, and we've encountered a buy, lets toss it
                if current_trades
                    .clone()
                    .into_iter()
                    .filter(|o| o.trade_type == TradeType::Sell)
                    .collect::<Vec<Trade>>()
                    .len()
                    > 0
                {
                    positions.push(current_trades.clone());
                    current_trades = Vec::new();
                }
            }
            TradeType::Sell => {}
        }
        current_trades.push(last_trade.clone());
    }

    positions.push(current_trades.clone());
    positions
}

// TODO: move into model
/// group trades with the same price together
pub fn group_trades_by_price(trades: Vec<Trade>) -> Vec<Trade> {
    let mut grouped_trades = Vec::new();
    let mut current_trade: Trade = trades.first().cloned().unwrap();
    current_trade.qty = 0.0;

    for trade in trades.clone() {
        if trade.sale_price == current_trade.sale_price
            && trade.trade_type == current_trade.trade_type
        {
            current_trade.qty += trade.qty;
        } else {
            grouped_trades.push(current_trade.clone());
            current_trade = trade.clone();
        }
    }

    grouped_trades.push(current_trade.clone());
    grouped_trades
}

pub trait TradeUtils {
    fn buys_only(&self) -> Vec<Trade>;
    fn sells_only(&self) -> Vec<Trade>;
}

impl TradeUtils for Vec<Trade> {
    fn buys_only(&self) -> Vec<Trade> {
        self.into_iter()
            .filter(|t| t.trade_type == TradeType::Buy)
            .cloned()
            .collect()
    }

    fn sells_only(&self) -> Vec<Trade> {
        self.into_iter()
            .filter(|t| t.trade_type == TradeType::Sell)
            .cloned()
            .collect()
    }
}
