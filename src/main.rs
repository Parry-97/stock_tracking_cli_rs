use clap::Parser;
use manning_lp_async_rust_project::{types::Opts, utils::fetch_stock_data};

use chrono::prelude::*;
use tokio::{fs, time};

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let to = Utc::now();

    let csv_content = fs::read_to_string("sp500.may.2020.txt")
        .await
        .expect("Couldn't read symbols file");

    // a simple way to output a CSV header
    println!("period start,symbol,price,change %,min,max,30d avg");
    let mut interval = time::interval(time::Duration::from_secs(30));
    loop {
        fetch_stock_data(csv_content.as_str(), &from, &to).await;
        interval.tick().await;
    }
}
