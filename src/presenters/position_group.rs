use models::*;
// use presenters::*;

#[derive(Debug, Clone)]
pub struct PositionGroupPresenter {
    total_btc_staked: f64,
    total_usd_staked: f64,
}

impl PositionGroupPresenter {
    pub fn new(presenters: Vec<PositionPresenter>) -> Self {
        Self {
            total_btc_staked: presenters.into_iter().map(|a| a.size_in_btc()).sum(),
            total_usd_staked: presenters.into_iter().map(|a| a.size_in_btc()).sum(),
        }
    }
}
