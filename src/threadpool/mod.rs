use std::sync::Arc;
// use rayon;
use rayon::prelude::*;

use exchanges::*;
use models::*;
use error::*;

use std::collections::HashMap;
// static MAX_THREADS:usize = 8;

pub fn chart_data(client: Arc<ExchangeAPI+Send+Sync>, pairs: Vec<String>, interval: &str) -> Vec<(String, Result<Vec<Candlestick>, TrailerError>)> {
    // let pool = rayon::ThreadPoolBuilder::new().num_threads(MAX_THREADS).build().unwrap();

    pairs.into_par_iter().map(|pair| {
        (pair.clone(), client.chart_data(&pair, interval))
    }).collect()
}

pub fn chart_data_2(client: Arc<ExchangeAPI+Send+Sync>, pairs: Vec<String>, interval: &str) -> HashMap<String, Result<Vec<Candlestick>, TrailerError>> {
    // let pool = rayon::ThreadPoolBuilder::new().num_threads(MAX_THREADS).build().unwrap();

    let mut chart_data = HashMap::new();

    for pair in pairs {
        chart_data.insert(pair.clone(), client.chart_data(&pair, interval));
    };

    chart_data
}
