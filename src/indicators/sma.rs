use crate::models::*;
use talib::{TA_Integer, TA_MAType, TA_RetCode, TA_MA};

pub fn sma(period: u32, candlesticks: &Vec<Candlestick>) -> Vec<f64> {
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;
    let close_prices: Vec<f64> = candlesticks.into_iter().map(|c| c.close_price).collect();
    let mut out: Vec<f64> = Vec::with_capacity(close_prices.len());

    unsafe {
        let ret_code = TA_MA(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the sma
            TA_MAType::TA_MAType_SMA,      // type of the MA, here forced to sma
            &mut out_begin,                // set to index of the first close to have an sma value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_MA call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    out
}
