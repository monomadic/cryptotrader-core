use crate::error::*;
use crate::models::*;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;
use ta::Next;

#[derive(Debug, Clone)]
pub struct MACDResult {
    pub fast: f64,
    pub slow: f64,
    pub signal: f64,
}

pub fn macd(
    fast_length: u32,
    slow_length: u32,
    signal_length: u32,
    candlesticks: &Vec<Candlestick>,
) -> CoreResult<Vec<MACDResult>> {
    let mut macd = Macd::new(fast_length, slow_length, signal_length)?;

    Ok(candlesticks
        .iter()
        .map(|candlestick| {
            let (fast, slow, signal) = macd.next(candlestick.close_price);
            MACDResult { fast, slow, signal }
        })
        .collect())
}

pub fn find_macd_crosses(results: Vec<MACDResult>) -> Vec<Crossing> {
    let signal = results
        .into_iter()
        .map(|result| (result.fast, result.slow))
        .collect();
    find_crosses(signal)
}

#[derive(Debug, Clone)]
pub enum Crossing {
    FastZeroUp(usize),
    FastZeroDown(usize),
    SlowZeroUp(usize),
    SlowZeroDown(usize),
    Death(usize),
    Golden(usize),
}

// score 0.0 to 10.0
pub fn score(crossings: Vec<Crossing>) -> f64 {
    for crossing in crossings {
        println!("{:?}", crossing);
        match crossing {
            Crossing::Death(_) => return 0.0, // is this right? should it not be distance toward a golden at this point?
            Crossing::Golden(_period) => return 10.0,
            _ => (),
        }
    }
    0.0
}

pub fn find_crosses(signal: Vec<(f64, f64)>) -> Vec<Crossing> {
    let mut crosses: Vec<Crossing> = Vec::new();
    let mut previous_slow: f64 = 0.0;
    let mut previous_fast: f64 = 0.0;
    // let signal: Vec<(f64, f64)> = signal.into_iter().rev().collect();
    let num_signals = signal.len();

    // todo: rewrite with peek
    for (index, (fast, slow)) in signal.into_iter().enumerate() {
        if index == 0 || index == num_signals {
            previous_slow = slow;
            previous_fast = fast;
            continue;
        }
        // check for zero crossings

        if previous_slow < 0.0 && slow >= 0.0 {
            crosses.push(Crossing::SlowZeroUp(num_signals - index))
        }

        if previous_slow > 0.0 && slow <= 0.0 {
            crosses.push(Crossing::SlowZeroDown(num_signals - index))
        }

        if previous_fast > 0.0 && fast <= 0.0 {
            crosses.push(Crossing::FastZeroDown(num_signals - index))
        }

        if previous_fast < 0.0 && fast >= 0.0 {
            crosses.push(Crossing::FastZeroUp(num_signals - index))
        }

        // check for golden and death crosses

        let divergence = fast - slow;
        let previous_divergence = previous_fast - previous_slow;

        // println!(
        //     "{:?}",
        //     (divergence, previous_divergence, num_signals - index)
        // );
        if divergence > 0.0 && previous_divergence <= 0.0 {
            crosses.push(Crossing::Golden(num_signals - index))
        } else if divergence < 0.0 && previous_divergence >= 0.0 {
            crosses.push(Crossing::Death(num_signals - index))
        }

        previous_slow = slow;
        previous_fast = fast;
    }

    // crosses
    crosses.into_iter().rev().collect()
}

// // fn find_last_macd_crossup(results: Vec<(f64, f64)>) -> f64 {

// fn find_last_cross(results: Vec<(f64, f64)>) -> f64 {
//     // let mut results = results.clone();
//     // let mut convergence = convergence(results.pop());

//     // while results.pop()

//     let mut signal;
//     for (signal1, signal2) in results {
//         signal = signal1;
//         if signal < 0.0 {

//         } else {

//         }
//     }

// }

// fn convergence(signal: (f64, f64)) -> f64 {
//     signal.1 - signal.0
// }

// // fn find_last_crossup(results: Vec<(f64, f64)>) -> f64 {

// // }
