# cryptotrader-core
Simple to use Crypto Exchange REST API client in rust.

This library has been in a semi-working, messy alpha state for over a year now, I figured it was time to clean it up and maybe people will actually find it useful. There's not much in the way of decent exchange support for rust, so hopefully this helps to change that.

The api wrappers currently implemented in rust I found to be quite buggy and not complete for what I needed. I extended some and submit various PRs but I think in the end I will be just using the forks found here.

Currently supports
- [binance](/robsaunders/binance-rs)

Coming soon:
- [binance](/robsaunders/huobi-rs)
- kucoin
- ?

Note that this is a core project with many components using this as a dependency. Some of the other projects in the cryptotrader toolchain include:
- [cryptotrader-cli](/robsaunders/cryptotrader-cli) - command line interface to cryptotrader.
- [cryptotrader-ticker](/robsaunders/cryptotrader-ticker) - websocket based multi threaded realtime crypto ticker.

There are other projects as well (web server front end in elm and back end as a rust service layer) but I will release those as they become a bit more mature.
