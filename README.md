# Stock Tracking CLI App

This is a CLI app for tracking the price of stocks. It uses the [Yahoo Finance API crate](https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/index.html) to get the stock prices. The app is written entirely in _Rust_ and uses the following tools:

- [Clap](https://docs.rs/clap/2.33.3/clap/) for parsing command line arguments
- [Yahoo Finance API](https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/index.html) for getting the stock prices
- [Rustls](https://docs.rs/rustls/0.19.1/rustls/) for HTTPS requests
- [Tokio](https://docs.rs/tokio/1.9.0/tokio/) for async runtime
- [Actix]("https://actix.rs/") for actor system

The app is compiled using the 2018 edition of Rust.

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
