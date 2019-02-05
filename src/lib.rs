pub mod config;
pub mod error;
pub mod exchanges;
pub mod models;
pub mod socket;
pub mod threadpool;
pub mod presenters {
    mod asset;
    mod funds;
    mod position;
    mod order;

    pub use self::{ asset::*, funds::*, position::*, order::* };
}
pub mod indicators {
    pub mod rsi;
}
