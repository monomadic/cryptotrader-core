pub mod binance;
pub use self::binance::BinanceWS;

// pub trait SocketConnection {
//     fn new();
//     fn connect();
//     fn PriceChange();
// }

#[derive(Debug)]
pub enum Event {
    PriceChange(String, f64, f64),
}
