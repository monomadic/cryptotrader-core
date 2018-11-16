use models::*;
use presenters::*;

#[derive(Debug, Clone)]
pub struct FundsPresenter {
    pub btc:                    Option<AssetPresenter>,
    pub fiat:                   Vec<AssetPresenter>,
    pub alts:                   Vec<AssetPresenter>,
    pub total_value_in_usd:     f64,
    pub total_value_in_btc:     f64,
}

impl FundsPresenter {
    pub fn new(funds: Funds, prices: Prices, btc_price_in_usd: f64) -> FundsPresenter {
        // let btc_price = if let Some(btc.clone()) = funds.btc { btc.amount } else { 0.0 };
        // let btc_price = *prices.get("BTCUSDT").unwrap_or(&0.0);

        // let fiat = funds.fiat.into_iter().map(|asset| AssetPresenter::new(asset.clone(), *prices.get(&asset.symbol).unwrap_or(&0.0), btc_price_in_usd));
        let fiat = funds.clone().fiat.into_iter().map(|asset| AssetPresenter{
            asset: asset.clone(),
            value_in_btc: asset.amount / btc_price_in_usd,
            value_in_usd: asset.amount,
        });

        let alts = funds.clone().alts.into_iter().map(|asset| AssetPresenter{
            asset: asset.clone(),
            value_in_btc: asset.amount * *prices.get(&format!("{}BTC", &asset.symbol)).unwrap_or(&0.0),
            value_in_usd: asset.amount * *prices.get(&format!("{}BTC", &asset.symbol)).unwrap_or(&0.0) * btc_price_in_usd,
        });

        // let alts = funds.alts.into_iter().map(|asset| AssetPresenter::new(asset.clone(), *prices.get(&asset.symbol).unwrap_or(&0.0), btc_price_in_usd));

        let alts_btc_value:f64  = alts.clone().map(|a| a.value_in_btc).sum();
        let fiat_btc_value:f64  = fiat.clone().map(|a| a.value_in_btc).sum();
        let btc_value:f64       = funds.clone().btc.and_then(|b| Some(b.amount)).unwrap_or(0.0);
        let total_price_in_btc  = alts_btc_value + fiat_btc_value + btc_value;

        let alts_usd_value:f64  = alts.clone().map(|a| a.value_in_usd).sum();
        let fiat_usd_value:f64  = fiat.clone().map(|a| a.value_in_usd).sum();
        let btc_usd_value:f64   = funds.clone().btc.and_then(|b| Some(b.amount * btc_price_in_usd)).unwrap_or(0.0);
        let total_price_in_usd  = alts_usd_value + fiat_usd_value + btc_usd_value;

        // code smell, fix this.
        // let mut total_price_in_btc:f64 = alts.clone().map(|a| a.value_in_btc).sum();
        // let fiat_price_btc:f64 = fiat.clone().map(|a| a.value_in_btc).sum();
        // total_price_in_btc = total_price_in_btc + fiat_price_btc;

        // let mut total_price_in_usd:f64 = alts.clone().map(|a| a.value_in_usd).sum();

        let btc = if let Some(btc) = funds.btc {
            Some(AssetPresenter{ asset: btc.clone(), value_in_btc: btc.amount, value_in_usd: btc.amount * btc_price_in_usd })
        } else { None };

        Self {
            btc:                    btc,
            fiat:                   fiat.collect(),
            alts:                   alts.collect(),
            total_value_in_usd:     total_price_in_usd,
            total_value_in_btc:     total_price_in_btc,
        }
    }
}

// #[test]
// fn test_funds_presenter_new() {
//     let funds = Funds { btc: Vec::new(),  };

//     let fp = FundsPresenter::new(funds: , prices: Prices)
//     assert_eq!(bot.stop_distance, 200.0);
// }