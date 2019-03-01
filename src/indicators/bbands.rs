use crate::models::*;
use talib::{TA_Integer, TA_MAType, TA_RetCode, TA_BBANDS};

// pub fn TA_BBANDS(
//      startIdx: ::std::os::raw::c_int,
//      endIdx: ::std::os::raw::c_int,
//      inReal: *const f64,
//      optInTimePeriod: ::std::os::raw::c_int,

//      optInNbDevUp: f64,
//      optInNbDevDn: f64,
//      optInMAType: TA_MAType,

//      outBegIdx: *mut ::std::os::raw::c_int,
//      outNBElement: *mut ::std::os::raw::c_int,

//      outRealUpperBand: *mut f64,
//      outRealMiddleBand: *mut f64,
//      outRealLowerBand: *mut f64
// ) -> TA_RetCode;

#[derive(Debug, Clone)]
pub struct BBand {
    pub upper_band: f64,
    pub middle_band: f64,
    pub lower_band: f64,
}

pub fn bbands(length: u32, candlesticks: &Vec<Candlestick>) -> Vec<BBand> {
    // let mut out: Vec<f64> = Vec::with_capacity(close_prices.len());
    // let mut out_begin: TA_Integer = 0;
    // let mut out_size: TA_Integer = 0;
    let close_prices: Vec<f64> = candlesticks.into_iter().map(|c| c.close_price).collect();

    let mut out_upper_band: Vec<f64> = Vec::with_capacity(close_prices.len());
    let mut out_middle_band: Vec<f64> = Vec::with_capacity(close_prices.len());
    let mut out_lower_band: Vec<f64> = Vec::with_capacity(close_prices.len());

    let mut out_upper_band_begin: TA_Integer = 0;
    let mut out_upper_band_size: TA_Integer = 0;

    let standard_deviation_up: f64 = 2.0;
    let standard_deviation_down: f64 = 2.0;

    //    Type of Moving Average: 0=SMA, 1=EMA, 2=WMA, 3=DEMA, 4=TEMA,
    //    5=TRIMA, 6=KAMA, 7=MAMA, 8=T3   (Default=SMA)
    // let MA_Type: i32 = 0;

    unsafe {
        let ret_code = TA_BBANDS(
            0,                             // startIdx
            close_prices.len() as i32 - 1, // endIdx
            close_prices.as_ptr(),         // inReal pointer to the first element of the vector
            length as i32,                 // optInTimePeriod period of the rsi From 2 to 100000
            standard_deviation_up,         // optInNbDevUp
            standard_deviation_down,       //optInNbDevDn
            TA_MAType::TA_MAType_SMA,      //optInMAType
            &mut out_upper_band_begin,     // outBegIdx
            &mut out_upper_band_size,      //outNBElement
            out_upper_band.as_mut_ptr(),   // outRealUpperBand: *mut f64,
            out_middle_band.as_mut_ptr(),  // outRealMiddleBand: *mut f64,
            out_lower_band.as_mut_ptr(),   // outRealLowerBand: *mut f64
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_RSI call
            TA_RetCode::TA_SUCCESS => {
                out_upper_band.set_len(out_upper_band_size as usize);
                out_middle_band.set_len(out_upper_band_size as usize);
                out_lower_band.set_len(out_upper_band_size as usize);
            }
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    println!(
        "{} {} {}",
        out_upper_band[0], out_middle_band[0], out_lower_band[0]
    );

    out_upper_band
        .into_iter()
        .enumerate()
        .map(|(index, upper_band)| BBand {
            upper_band,
            middle_band: out_middle_band[index],
            lower_band: out_lower_band[index],
        })
        .collect()
}
