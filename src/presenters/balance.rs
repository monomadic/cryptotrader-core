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

    pub fn total_value_in_btc(&self) -> Option<f64> {
        Some(
            self.assets
                .iter()
                .map({
                    |asset| match asset.asset_type() {
                        AssetType::Bitcoin => asset.amount,
                        AssetType::Fiat => {
                            0.0 // implement
                        }
                        AssetType::Altcoin => {
                            find_first_btc_pair_for_symbol(&asset.symbol, self.pairs.clone())
                                .map(|p| p.price)
                                .unwrap_or(0.0)
                                * asset.amount
                        }
                    }
                })
                .sum(),
        )
    }

    pub fn total_value_in_usd(&self) -> Option<f64> {
        self.total_value_in_btc().map(|btc_price| {
            find_first_btc_usd_pair(self.pairs.clone())
                .map(|p| p.price)
                .unwrap_or(0.0)
                * btc_price
        })
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
