# Stock Tracking CLI App

This is a CLI app for tracking the price of stocks. It uses the [Yahoo Finance API crate](https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/index.html) to get the stock prices. The app is written entirely in _Rust_ and uses the following tools:

- [Clap](https://docs.rs/clap/2.33.3/clap/) for parsing command line arguments
- [Yahoo Finance API](https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/index.html) for getting the stock prices
- [Rustls](https://docs.rs/rustls/0.19.1/rustls/) for HTTPS requests
- [Tokio](https://docs.rs/tokio/1.9.0/tokio/) for async runtime
- [Actix](https://actix.rs/) for actor system
- [Warp](https://docs.rs/warp/0.3.1/warp/) for web server and buffer REST service

The app is compiled using the 2018 edition of Rust.

The app architecture can be summarized in the following diagram:
![Architecture diagram](https://d16rtcb5cr0vb4.cloudfront.net/C0683+Processing+Data+in+Async+Actors+%2FResources%2FImages%2F2021-07-14_Data-Streaming-with-Async-Rust-2+%281%29_V1.png?Expires=1689211669&Signature=aoXT7N2c-b00zlj~Ugooe0TsByrrC6aVblbAeFLDE69oMew3EXJfgbYjaN0FD-kvnt0HCl6JShTOc2MqJFNhNPOLkoCB8yhID~3iaE7w5CsC15lcVVztVZsjDQil8ZzfXKWoqeGIW1IxERZ492lEUoVZwc5BvhjJAqWasevfXs~NPeoFe6wi4WVvnP9MIo-FTIiRcVYTvupgih52RCUoddXhbWVAte3CkB93hzsiWk2UlGF9Gtn6JNtkLcUJR4kzVtuU1TbBhbGOGJJdDMpAIkWfidlikFyvZjUNVnNmI5ansrJeDi-BZ53L5ebcgg7T3u7Ddky8Ea-iiXLQUKAgjA__&Key-Pair-Id=APKAIHLKH2FX732Z3HGA)

## Example Usage

```bash

$ ./stock_cli --help

stock_cli 1.0
Param Singh, Claus Matzinger
A Manning LiveProject: Async Streams in Rust

USAGE:
    stock_cli [OPTIONS] --from <FROM>

OPTIONS:
    -f, --from <FROM>
            Required start date for the period to fetch

    -h, --help
            Print help information

    -m, --max-iterations <MAX_ITERATIONS>
            Optional number of max iterations to run [default: 1]

    -s, --source <SOURCE>
            Optional .txt source file to read symbols from [default: sp500.may.2020.txt]

    -V, --version
            Print version information

```
