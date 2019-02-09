use crate::common::*;
use cryptotrader::models::*;

mod common;

#[test]
fn it_works_with_one_entry() {
    let orders = group_by_price(vec![order_fixture(TradeType::Buy, 1.0, 100.0)]);
    assert_eq!(orders.len(), 1);
}
