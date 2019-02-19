use crate::models::*;
use crate::utils::*;
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
    pub fn entry_price(&self) -> f64 {
        self.price
    }

    /// what the trade cost when bought/sold
    pub fn entry_cost(&self) -> f64 {
        self.qty * self.price
    }

    /// the current value of the trade
    pub fn current_cost(&self) -> f64 {
        self.qty * self.pair.price
    }

    pub fn current_profit(&self) -> f64 {
        self.current_cost() - self.entry_cost()
        // match self.trade_type {
        //     TradeType::Buy => self.current_cost() - self.entry_cost(),
        //     TradeType::Sell => 0.0 - self.current_cost() - self.entry_cost(),
        // }
    }

    pub fn current_profit_as_percent(&self) -> f64 {
        // log::info!("{} {}, {}", self.trade_type, self.entry_price(), self.current_price());
        price_percent(self.entry_price(), self.current_price())
    }

    pub fn current_price(&self) -> f64 {
        self.pair.price
    }

    pub fn purchase_price(&self) -> f64 {
        self.price
    }

    // ------------------------------------------------------------------------------------------

    // TODO ALIAS - delete
    pub fn profit_as_percent(&self) -> f64 {
        self.current_profit_as_percent()
    }

    // TODO ALIAS - delete
    pub fn cost(&self) -> f64 {
        self.entry_cost()
    }

    // TODO ALIAS - delete
    pub fn value(&self) -> f64 {
        self.current_cost()
    }

    // TODO ALIAS - delete
    pub fn profit(&self) -> f64 {
        self.current_profit()
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

    let average_price = average(&trades_iter.clone().map(|t| t.price).collect());

    let pairs = trades_iter.clone().map(|t| t.pair).collect();
    let id = trades_iter
        .clone()
        .map(|t| t.id)
        .collect::<Vec<String>>()
        .join(", ");

    Trade {
        id,
        pair: average_pairs(pairs),
        trade_type: first_trade.trade_type,
        price: average_price,
        qty,
        time: first_trade.time,
        fee: trades_iter.map(|t| t.fee).sum(),
        fee_symbol: first_trade.fee_symbol.clone(),
    }
}

pub fn group_trades_by_trade_type(trades: Vec<Trade>) -> Vec<Vec<Trade>> {
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
pub fn group_trades_by_trade_type_pair(trades: Vec<Trade>) -> Vec<Vec<Trade>> {
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

/// group trades with the same price together
pub fn group_trades_by_price(trades: Vec<Trade>) -> Vec<Trade> {
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
