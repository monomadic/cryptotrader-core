use models::*;

#[derive(Debug, Clone)]
pub struct AssetPresenter {
    pub asset: Asset,
    pub value_in_btc: f64,
    pub value_in_usd: f64,
}

// impl AssetPresenter {
//     pub fn new(asset: Asset, asset_price_in_btc: f64, btc_price_in_usd: f64) -> Self {
//         Self {
//             asset: asset.clone(),
//             value_in_btc:   asset.amount * asset_price_in_btc,
//             value_in_usd:   asset.amount * asset_price_in_btc * btc_price_in_usd,
//         }
//     }
// }

// #[test]
// fn test_asset_presenter_new() {
//     use exchanges::*;

//     let asset = Asset{ symbol: "BLAH".into(), amount: 5.0, locked: 0.0, exchange: Exchange::Binance };
//     let presenter = AssetPresenter::new(asset, 0.001, 6500.0);
//     assert_eq!(presenter.value_in_btc, 0.005);
//     assert_eq!(presenter.value_in_usd, 32.5);
// }
