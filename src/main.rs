use std::{error::Error, time::Duration};

use actix::Actor;
use chrono::{DateTime, Utc};
use clap::Parser;
use manning_lp_async_rust_project::{
    actors::{
        DataLoadActor, DataLoadMessage, DataProcessActor, DataProcessMessage, DataSaveActor,
        DataSaveMessage,
    },
    types::Opts,
};

use tokio::time;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let to = Utc::now();
    let mut interval = time::interval(Duration::from_secs(30));
    let path: String = opts.source.parse().expect("Couldn't parse 'path'");
    let save_path = path.replace(".txt", ".csv");
    let max_iters = opts.max_iterations;
    // let content = fs::read_to_string(path).await?;

    // let mono_addr = MonoActor {}.start();
    let data_load_addr = DataLoadActor {}.start();
    let data_process_addr = DataProcessActor {}.start();
    let data_save_addr = DataSaveActor {}.start();
    // a simple way to output a CSV header
    let mut iterations = 1;
    println!("period start,symbol,price,change %,min,max,30d avg");
    loop {
        // let mono_process = mono_addr
        //     .send(MonoMessage {
        //         content: content.to_string(),
        //         from: from.clone(),
        //         to: to.clone(),
        //     })
        //     .await??;
        // println!("{}\niteration #{}", mono_process, iterations);
        if iterations > max_iters {
            break;
        }
        let data_load_process = data_load_addr
            .send(DataLoadMessage {
                path: path.to_string(),
            })
            .await??;

        let data_process_process = data_process_addr
            .send(DataProcessMessage {
                content: data_load_process.to_string(),
                from: from.clone(),
                to: to.clone(),
            })
            .await??;

        data_save_addr
            .send(DataSaveMessage {
                content: data_process_process.to_string(),
                path: save_path.to_string(),
            })
            .await??;
        iterations += 1;

        interval.tick().await;
    }
    Ok(())
}
