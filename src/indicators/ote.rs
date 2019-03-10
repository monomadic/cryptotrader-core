use crate::models::*;

/// Optimal Trading Environment Index
/// - gives a risk-reward approximation.
pub fn ote(period: u32, candlesticks: &Vec<Candlestick>) -> f64 {
    let candlesticks: Vec<Candlestick> = candlesticks
        .into_iter()
        .cloned()
        .take(period as usize)
        .collect();

    total_range(&candlesticks) / average_bar_range(&candlesticks)
}

fn average_bar_range(candlesticks: &Vec<Candlestick>) -> f64 {
    candlesticks
        .iter()
        .map(|candle| (candle.open_price - candle.close_price).abs())
        .sum::<f64>()
        / candlesticks.len() as f64
}

fn total_range(candlesticks: &Vec<Candlestick>) -> f64 {
    let high_price: f64 = candlesticks
        .iter()
        .max_by(|a, b| cmp_f64(&a.high_price, &b.high_price))
        .map(|candle| candle.high_price)
        .unwrap_or(0.0);

    let low_price: f64 = candlesticks
        .iter()
        .min_by(|a, b| cmp_f64(&a.low_price, &b.low_price))
        .map(|candle| candle.low_price)
        .unwrap_or(0.0);

    high_price - low_price
}

use std::cmp::Ordering;
fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}
