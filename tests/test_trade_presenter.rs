//use crate::common::*;
//use cryptotrader::models::*;
//use cryptotrader::presenters::TradePresenter;
//
//mod common;
//
//// trade_fixture(trade_type: TradeType, price: f64, current_price: f64, qty: f64);
//// pair_fixture(base: String, symbol: String, price: f64);
//
//fn trade_presenter_1() -> TradePresenter {
//    // buy 1 SMOSH @ 1 btc, now at 2 btc
//    let trade = trade_fixture(TradeType::Buy, 1.0, 2.0, 1.0);
//    // 1 btc = 10 dollars
//    let fiat_pair = Some(pair_fixture(
//        DEFAULT_FIAT.to_string(),
//        DEFAULT_SYMBOL.to_string(),
//    ));
//    TradePresenter { trade, fiat_pair }
//}
//
//fn trade_presenter_2() -> TradePresenter {
//    let trade = trade_fixture(TradeType::Buy, 1.0, 2.0, 2.0);
//    let fiat_pair = Some(pair_fixture(
//        DEFAULT_FIAT.to_string(),
//        DEFAULT_SYMBOL.to_string(),
//    ));
//    TradePresenter { trade, fiat_pair }
//}
//
////#[test]
////fn trade_presenter_value_in_fiat() {
////    assert_eq!(trade_presenter_1().value_in_fiat(), Some(20.0));
////    // assert_eq!(presenter.trade.current_profit_as_percent(), 100.0);
////}
////
////#[test]
////fn trade_presenter_profit_in_fiat() {
////    assert_eq!(trade_presenter_1().profit_in_fiat(), Some(100.0));
////}
