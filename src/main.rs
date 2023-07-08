use ::time::OffsetDateTime;
use async_trait::async_trait;
use chrono::prelude::*;
use clap::Parser;
use std::io::{Error, ErrorKind};
use tokio::{fs, join, time};
use yahoo_finance_api as yahoo;

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Claus Matzinger",
    about = "A Manning LiveProject: async Rust"
)]
struct Opts {
    #[clap(short, long, default_value = "AAPL,MSFT,UBER,GOOG")]
    symbols: String,
    #[clap(short, long)]
    from: String,
}

#[derive(Debug)]
struct WindowedSMA {
    window_size: usize,
}

#[async_trait]
impl AsyncStockSignal for WindowedSMA {
    type SignalType = Vec<f64>;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        n_window_sma(self.window_size, series)
    }
}

#[derive(Debug)]
struct MaxPrice {}

#[async_trait]
impl AsyncStockSignal for MaxPrice {
    type SignalType = f64;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        max(series)
    }
}
#[derive(Debug)]
struct MinPrice {}

#[async_trait]
impl AsyncStockSignal for MinPrice {
    type SignalType = f64;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        min(series)
    }
}
#[derive(Debug)]
struct PriceDifference {}

#[async_trait]
impl AsyncStockSignal for PriceDifference {
    type SignalType = (f64, f64);

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        price_diff(series)
    }
}

///
/// A trait to provide a common interface for all signal calculations.
///
#[async_trait]
trait AsyncStockSignal {
    ///
    /// The signal's data type.
    ///
    type SignalType;

    ///
    /// Calculate the signal on the provided series.
    ///
    /// # Returns
    ///
    /// The signal (using the provided type) or `None` on error/invalid data.
    ///
    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType>;
}

///
/// Calculates the absolute and relative difference between the beginning and ending of an f64 series. The relative difference is relative to the beginning.
///
/// # Returns
///
/// A tuple `(absolute, relative)` difference.
///
fn price_diff(a: &[f64]) -> Option<(f64, f64)> {
    if !a.is_empty() {
        // unwrap is safe here even if first == last
        let (first, last) = (a.first().unwrap(), a.last().unwrap());
        let abs_diff = last - first;
        let first = if *first == 0.0 { 1.0 } else { *first };
        let rel_diff = abs_diff / first;
        Some((abs_diff, rel_diff))
    } else {
        None
    }
}

///
/// Window function to create a simple moving average
///
fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    if !series.is_empty() && n > 1 {
        Some(
            series
                .windows(n)
                .map(|w| w.iter().sum::<f64>() / w.len() as f64)
                .collect(),
        )
    } else {
        None
    }
}

///
/// Find the maximum in a series of f64
///
fn max(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MIN, |acc, q| acc.max(*q)))
    }
}

///
/// Find the minimum in a series of f64
///
fn min(series: &[f64]) -> Option<f64> {
    if series.is_empty() {
        None
    } else {
        Some(series.iter().fold(f64::MAX, |acc, q| acc.min(*q)))
    }
}

///
/// Retrieve data from a data source and extract the closing prices. Errors during download are mapped onto io::Errors as InvalidData.
///
async fn fetch_closing_data(
    symbol: &str,
    beginning: &DateTime<Utc>,
    end: &DateTime<Utc>,
) -> std::io::Result<Vec<f64>> {
    let provider = yahoo::YahooConnector::new();

    let response = provider
        .get_quote_history(
            symbol,
            OffsetDateTime::from_unix_timestamp(beginning.timestamp()).unwrap(),
            OffsetDateTime::from_unix_timestamp(end.timestamp()).unwrap(),
        )
        .await
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    let mut quotes = response
        .quotes()
        .map_err(|_| Error::from(ErrorKind::InvalidData))?;
    if !quotes.is_empty() {
        quotes.sort_by_cached_key(|k| k.timestamp);
        Ok(quotes.iter().map(|q| q.adjclose as f64).collect())
    } else {
        Ok(vec![])
    }
}
#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let to = Utc::now();

    let csv_content = fs::read_to_string("sp500.may.2020.txt")
        .await
        .expect("Couldn't read symbols file");
    // a simple way to output a CSV header
    //
    let mut interval = time::interval(time::Duration::from_secs(30));

    println!("period start,symbol,price,change %,min,max,30d avg");
    loop {
        fetch_stock_data(csv_content.as_str(), &from, &to).await;
        interval.tick().await;
    }
}

async fn fetch_stock_data(content: &str, from: &DateTime<Utc>, to: &DateTime<Utc>) {
    let csv_symbols = content.split(',');
    let mut handles = vec![];

    for symbol in csv_symbols {
        handles.push(tokio::spawn(fetch_symbol(
            symbol.to_owned(),
            from.clone(),
            to.clone(),
        )));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn fetch_symbol(symbol: String, from: DateTime<Utc>, to: DateTime<Utc>) {
    let closes_fetched = fetch_closing_data(&symbol, &from, &to).await;
    if closes_fetched.is_err() {
        eprintln!("Error fetch closing data for {}", symbol);
        return;
    }
    let closes = closes_fetched.unwrap();

    if !closes.is_empty() {
        // min/max of the period. unwrap() because those are Option types
        let max_signal = MaxPrice {};

        let min_signal = MinPrice {};

        let last_price = *closes.last().unwrap_or(&0.0);

        let price_diff_signal = PriceDifference {};

        // .await
        // .unwrap_or((0.0, 0.0));
        let sma_signal = WindowedSMA { window_size: 30 };

        let (period_max, period_min, price_diff, sma) = join!(
            max_signal.calculate(&closes),
            min_signal.calculate(&closes),
            price_diff_signal.calculate(&closes),
            sma_signal.calculate(&closes)
        );

        let (_, pct_change) = price_diff.unwrap_or((0.0, 0.0));

        // a simple way to output CSV data
        println!(
            "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
            from.to_rfc3339(),
            symbol,
            last_price,
            pct_change * 100.0,
            period_min.unwrap(),
            period_max.unwrap(),
            sma.unwrap().last().unwrap_or(&0.0)
        );
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use tokio::join;

    use super::*;

    #[tokio::test]
    async fn test_PriceDifference_calculate() {
        let signal = PriceDifference {};
        assert_eq!(signal.calculate(&[]).await, None);
        assert_eq!(signal.calculate(&[1.0]).await, Some((0.0, 0.0)));
        assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some((-1.0, -1.0)));
        assert_eq!(
            signal
                .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
                .await,
            Some((8.0, 4.0))
        );
        assert_eq!(
            signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some((1.0, 1.0))
        );
    }

    #[tokio::test]
    async fn test_parallel_PriceDifference_calculate() {
        let signal = PriceDifference {};
        let (empty_series, single_value_series, multiple_vals) = join!(
            signal.calculate(&[]),
            signal.calculate(&[1.0]),
            signal.calculate(&[1.0, 0.0])
        );

        assert_eq!(empty_series, None);
        assert_eq!(single_value_series, Some((0.0, 0.0)));
        assert_eq!(multiple_vals, Some((-1.0, -1.0)));
        // assert_eq!(
        //     signal
        //         .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
        //         .await,
        //     Some((8.0, 4.0))
        // );
        // assert_eq!(
        //     signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
        //     Some((1.0, 1.0))
        // );
    }

    #[tokio::test]
    async fn test_parallel_MinPrice_calculate() {
        let signal = MinPrice {};
        let (empty_series, single_value_series, multiple_vals) = join!(
            signal.calculate(&[]),
            signal.calculate(&[1.0]),
            signal.calculate(&[1.0, 0.0])
        );

        assert_eq!(empty_series, None);
        assert_eq!(single_value_series, Some(1.0));
        assert_eq!(multiple_vals, Some(0.0));
        // assert_eq!(signal.calculate(&[]).await, None);
        // assert_eq!(signal.calculate(&[1.0]).await, Some(1.0));
        // assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some(0.0));
        // assert_eq!(
        //     signal
        //         .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
        //         .await,
        //     Some(1.0)
        // );
        // assert_eq!(
        //     signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
        //     Some(0.0)
        // );
    }
    #[tokio::test]
    async fn test_MinPrice_calculate() {
        let signal = MinPrice {};
        assert_eq!(signal.calculate(&[]).await, None);
        assert_eq!(signal.calculate(&[1.0]).await, Some(1.0));
        assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some(0.0));
        assert_eq!(
            signal
                .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
                .await,
            Some(1.0)
        );
        assert_eq!(
            signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some(0.0)
        );
    }

    #[tokio::test]
    async fn test_MaxPrice_calculate() {
        let signal = MaxPrice {};
        assert_eq!(signal.calculate(&[]).await, None);
        assert_eq!(signal.calculate(&[1.0]).await, Some(1.0));
        assert_eq!(signal.calculate(&[1.0, 0.0]).await, Some(1.0));
        assert_eq!(
            signal
                .calculate(&[2.0, 3.0, 5.0, 6.0, 1.0, 2.0, 10.0])
                .await,
            Some(10.0)
        );
        assert_eq!(
            signal.calculate(&[0.0, 3.0, 5.0, 6.0, 1.0, 2.0, 1.0]).await,
            Some(6.0)
        );
    }

    #[tokio::test]
    async fn test_WindowedSMA_calculate() {
        let series = vec![2.0, 4.5, 5.3, 6.5, 4.7];

        let signal = WindowedSMA { window_size: 3 };
        assert_eq!(
            signal.calculate(&series).await,
            Some(vec![3.9333333333333336, 5.433333333333334, 5.5])
        );

        let signal = WindowedSMA { window_size: 5 };
        assert_eq!(signal.calculate(&series).await, Some(vec![4.6]));

        let signal = WindowedSMA { window_size: 10 };
        assert_eq!(signal.calculate(&series).await, Some(vec![]));
    }
}
