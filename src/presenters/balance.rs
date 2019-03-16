use crate::models::*;

#[derive(Debug, Clone)]
pub struct BalancePresenter {
    pub assets: Vec<Asset>,
    pub pairs: Vec<Pair>,
}

impl BalancePresenter {
    // pub fn total_value_in_btc(&self) -> Option<f64> {
    //     log::info!(
    //         "total_value_in_btc {:#?}",
    //         find_first_btc_usd_pair(self.pairs.clone())
    //     );
    //     find_first_btc_usd_pair(self.pairs.clone()).map(|p| p.price)
    // }

    // todo: this doesn't need to be an option does it?
    pub fn total_value_in_btc(&self) -> f64 {
        self.assets
            .iter()
            .map({
                |asset| match asset.asset_type() {
                    AssetType::Bitcoin => asset.amount,
                    AssetType::Fiat => {
                        asset.amount
                            / Pair::find_first_btc_usd_pair(&self.pairs)
                                .map(|p| p.price)
                                .unwrap_or(0.0)
                    }
                    AssetType::Altcoin => {
                        Pair::find_first_btc_pair_for_symbol(&asset.symbol, self.pairs.clone())
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
                        Pair::find_first_btc_pair_for_symbol(&asset.symbol, self.pairs.clone())
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
        self.total_value_in_btc()
            * Pair::find_first_btc_usd_pair(&self.pairs)
                .map(|p| p.price)
                .unwrap_or(0.0)
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
