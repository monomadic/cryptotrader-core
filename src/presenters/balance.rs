use crate::models::*;

#[derive(Debug, Clone)]
pub struct BalancePresenter {
    pub assets: Vec<Asset>,
    pub prices: Vec<Price>,
}

impl BalancePresenter {
    pub fn _total_value_in(&self, symbol: &str) -> f64 {
        self.assets
            .clone()
            .into_iter()
            .map(|asset| self.prices.price_of(&asset.symbol, symbol).unwrap_or(0.0))
            .sum()
    }

    // todo: this doesn't need to be an option does it?
    pub fn total_value_in_btc(&self) -> f64 {
        let btc_price = Price::find_first_btc_usd_price(&self.prices)
            .map(|p| p.price)
            .unwrap_or(0.0);

        self.assets
            .iter()
            .map({
                |asset| match asset.asset_type() {
                    AssetType::Bitcoin => asset.amount,
                    AssetType::Stablecoin => asset.amount / btc_price,
                    AssetType::Altcoin => {
                        Price::find_first_btc_price_for_symbol(&asset.symbol, self.prices.clone())
                            .map(|p| p.price)
                            .unwrap_or(0.0)
                            * asset.amount
                    }
                }
            })
            .sum()
    }

    pub fn alts_value_in_btc(&self) -> f64 {
        self.assets
            .iter()
            .map({
                |asset| match asset.asset_type() {
                    AssetType::Altcoin => {
                        Price::find_first_btc_price_for_symbol(&asset.symbol, self.prices.clone())
                            .map(|p| p.price)
                            .unwrap_or(0.0)
                            * asset.amount
                    }
                    _ => 0.0,
                }
            })
            .sum()
    }

    pub fn total_value_in_usd(&self) -> f64 {
        let btc_price = Price::find_first_btc_usd_price(&self.prices).map_or(0.0, |p| p.price);

        self.assets
            .iter()
            .map({
                |asset| match asset.asset_type() {
                    AssetType::Bitcoin => asset.amount * btc_price,
                    AssetType::Stablecoin => asset.amount,
                    AssetType::Altcoin => {
                        Price::find_first_btc_price_for_symbol(&asset.symbol, self.prices.clone())
                            .map(|p| p.price)
                            .unwrap_or(
                                Price::find_first_btc_price_for_symbol(
                                    &asset.symbol,
                                    self.prices.clone(),
                                )
                                .map_or(0.0, |p| p.price * btc_price),
                            )
                            * asset.amount
                    }
                }
            })
            .sum()

        // self.total_value_in_btc()
        //     * Pair::find_first_btc_usd_pair(&self.pairs)
        //         .map(|p| p.price)
        //         .unwrap_or(0.0)
    }

    // pub fn asset_presenters(&self) -> Vec<AsssetPresener> {

    // }
}

// #[test]
// fn test_asset_presenter_new() {
//     use exchanges::*;

//     let asset = Asset{ symbol: "BLAH".into(), amount: 5.0, locked: 0.0, exchange: Exchange::Binance };
//     let presenter = AssetPresenter::new(asset, 0.001, 6500.0);
//     assert_eq!(presenter.value_in_btc, 0.005);
//     assert_eq!(presenter.value_in_usd, 32.5);
// }
