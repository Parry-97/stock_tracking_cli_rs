use async_trait::async_trait;
use clap::Parser;

use crate::calculations::{max, min, n_window_sma, price_diff};

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Param Singh, Claus Matzinger",
    about = "A Manning LiveProject: Async Streams in Rust"
)]
pub struct Opts {
    /// Optional .txt source file to read symbols from
    #[clap(short, long, default_value = "sp500.may.2020.txt")]
    pub source: String,

    /// Required start date for the period to fetch
    #[clap(short, long)]
    pub from: String,
    // Optional number of max iterations to run
    // #[clap(short, long, default_value = "1")]
    // pub max_iterations: usize,
}

#[derive(Debug)]
pub struct WindowedSMA {
    pub window_size: usize,
}

#[async_trait]
impl AsyncStockSignal for WindowedSMA {
    type SignalType = Vec<f64>;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        n_window_sma(self.window_size, series)
    }
}

#[derive(Debug)]
pub struct MaxPrice {}

#[async_trait]
impl AsyncStockSignal for MaxPrice {
    type SignalType = f64;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        max(series)
    }
}
#[derive(Debug)]
pub struct MinPrice {}

#[async_trait]
impl AsyncStockSignal for MinPrice {
    type SignalType = f64;

    async fn calculate(&self, series: &[f64]) -> Option<Self::SignalType> {
        min(series)
    }
}
#[derive(Debug)]
pub struct PriceDifference {}

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
pub trait AsyncStockSignal {
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
