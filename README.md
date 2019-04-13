# cryptotrader-core
Simple to use Crypto Exchange REST API client in rust.

This library has been in a semi-working, messy alpha state for over a year now, I figured it was time to clean it up and maybe people will actually find it useful. There's not much in the way of decent exchange support for rust, so hopefully this helps to change that.

The goal is not to create another simple wrapper around an API, but an easy to use high level abstraction across all apis.

Currently supports
- [binance](/robsaunders/binance-rs)

Coming soon:
- [huobi](/robsaunders/huobi-rs)
- kucoin
- ?

Note that this is a core project with many components using this as a dependency. Some of the other projects in the cryptotrader toolchain include:
- [cryptotrader-cli](/robsaunders/cryptotrader-cli) - command line interface to cryptotrader.
- [cryptotrader-ticker](/robsaunders/cryptotrader-ticker) - websocket based multi threaded realtime crypto ticker.

There are other projects as well (web server front end in elm and back end as a rust service layer) but I will release those as they become a bit more mature.

## Example use

```rust
use cryptotrader::exchanges::*;

fn main() {
    let client = BinanceAPI::connect("key", "secret");
    let assets = client.balances().unwrap();

    for asset in assets {
        println!("{} - {}", asset.symbol, asset.amount);
    }
}
```
