use std::io::{Error, ErrorKind};

use chrono::{DateTime, Utc};
use time::OffsetDateTime;
use tokio::join;
use yahoo_finance_api as yahoo;

use crate::types::{AsyncStockSignal, MaxPrice, MinPrice, PriceDifference, WindowedSMA};
///
/// Retrieve data from a data source and extract the closing prices. Errors during download are mapped onto io::Errors as InvalidData.
///
pub async fn fetch_closing_data(
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

pub async fn fetch_stock_data(
    content: &str,
    from: &DateTime<Utc>,
    to: &DateTime<Utc>,
) -> Result<String, std::io::Error> {
    let csv_symbols = content.split(',');
    let mut handles = vec![];
    let mut lines = vec!["period start,symbol,price,change %,min,max,30d avg".to_string()];
    for symbol in csv_symbols {
        handles.push(tokio::spawn(fetch_symbol(
            symbol.to_owned(),
            from.clone(),
            to.clone(),
        )));
    }

    for handle in handles {
        lines.push(handle.await.unwrap());
    }

    Ok(lines.join("\n"))
}

async fn fetch_symbol(symbol: String, from: DateTime<Utc>, to: DateTime<Utc>) -> String {
    let closes_fetched = fetch_closing_data(&symbol, &from, &to).await;
    if closes_fetched.is_err() {
        return "Could not fetch closing data".to_string();
    }
    let closes = closes_fetched.unwrap();

    if closes.is_empty() {
        return "Retrieved Series is is empty".to_string();
    }
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
    format!(
        "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
        from.to_rfc3339(),
        symbol,
        last_price,
        pct_change * 100.0,
        period_min.unwrap(),
        period_max.unwrap(),
        sma.unwrap().last().unwrap_or(&0.0)
    )
}
