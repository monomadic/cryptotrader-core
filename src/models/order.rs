use crate::models::*;
// use crate::utils::*;
use chrono::DateTime;
use chrono::Local;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub pair: Pair,
    pub order_type: OrderType, // limit, stop loss, etc.
    pub trade_type: TradeType, // buy or sell
    pub qty: f64,
    pub executed_qty: f64,
    pub purchase_price: f64,
    // pub current_price: f64,
    pub time: DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            OrderType::Limit => write!(f, "Limit"),
            OrderType::Market => write!(f, "Market"),
            OrderType::StopLoss => write!(f, "StopLoss"),
            OrderType::StopLossLimit => write!(f, "StopLossLimit"),
            OrderType::TakeProfit => write!(f, "TakeProfit"),
            OrderType::TakeProfitLimit => write!(f, "TakeProfitLimit"),
            OrderType::LimitMaker => write!(f, "LimitMaker"),
        }
    }
}
