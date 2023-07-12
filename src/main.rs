use std::{error::Error, time::Duration, usize};

use actix::{Actor, Addr};
use chrono::{DateTime, Utc};
use clap::Parser;
use stock_cli::{
    actors::{
        BufferActor, BufferMessage, BufferReadMessage, DataLoadActor, DataLoadMessage,
        DataProcessActor, DataProcessMessage, DataSaveActor, DataSaveMessage,
    },
    types::Opts,
};

use tokio::time;
use warp::Filter;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let from: DateTime<Utc> = opts.from.parse().expect("Couldn't parse 'from' date");
    let to = Utc::now();
    let mut interval = time::interval(Duration::from_secs(10));

    let path: String = opts.source.parse().expect("Couldn't parse 'path'");
    let save_path = path.replace(".txt", ".csv");
    // let max_iters = opts.max_iterations;
    // let content = fs::read_to_string(path).await?;

    // let mono_addr = MonoActor {}.start();
    let data_load_addr = DataLoadActor {}.start();
    let data_process_addr = DataProcessActor {}.start();
    let data_save_addr = DataSaveActor {}.start();
    let buffer_actor_addr = BufferActor::new(20).start();

    let buffer_addr = buffer_actor_addr.clone();
    let tail_handler = warp::path!("tail" / usize)
        .and(with_db(buffer_addr))
        .and(warp::get())
        .and_then(buffer_read_handler);
    // a simple way to output a CSV header
    let mut iterations = 1;
    tokio::spawn(async {
        warp::serve(tail_handler).run(([127, 0, 0, 1], 8080)).await;
    });
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
        interval.tick().await;
        // if iterations > max_iters {
        //     break;
        // }
        let data_load_process = data_load_addr
            .send(DataLoadMessage {
                path: path.to_string(),
            })
            .await??;

        let data_process_process = data_process_addr
            .send(DataProcessMessage {
                content: data_load_process,
                from: from.clone(),
                to: to.clone(),
            })
            .await??;

        println!(
            "{}\n ----------------------Iteration #{}--------------------------",
            data_process_process, iterations,
        );

        buffer_actor_addr
            .send(BufferMessage {
                content: data_process_process.clone(),
            })
            .await??;

        data_save_addr
            .send(DataSaveMessage {
                content: data_process_process,
                path: save_path.to_string(),
            })
            .await??;
        iterations += 1;
    }

    // Ok(())
}

async fn buffer_read_handler(n: usize, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    match db.send(BufferReadMessage { n }).await {
        Ok(it) => {
            return Ok(warp::reply::json::<Vec<String>>(
                &it.unwrap_or(Vec::default()),
            ))
        }
        _ => return Err(warp::reject::not_found()),
    };
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

type Db = Addr<BufferActor>;
