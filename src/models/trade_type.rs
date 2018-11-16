#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TradeType {
    pub fn is_buy(s: bool) -> TradeType {
        match s {
            true => TradeType::Buy,
            false => TradeType::Sell,
        }
    }

    pub fn buy(&self) -> bool {
        match self {
            TradeType::Buy  => true,
            TradeType::Sell => false,
        }
    }
}

impl ::std::fmt::Display for TradeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TradeType::Buy => write!(f, "BUY"),
            TradeType::Sell => write!(f, "SELL"),
        }
    }
}
