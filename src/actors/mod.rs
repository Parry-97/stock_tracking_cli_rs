use actix::{Actor, Context, Handler, Message, ResponseFuture};
use chrono::{DateTime, Utc};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::utils::fetch_stock_data;

#[derive(Debug)]
pub struct MonoActor {}

impl Actor for MonoActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct MonoMessage {
    pub content: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl Handler<MonoMessage> for MonoActor {
    type Result = ResponseFuture<Result<String, std::io::Error>>;
    fn handle(&mut self, msg: MonoMessage, _: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let content = msg.content;
            let from = msg.from;
            let to = msg.to;
            let csv_content = fetch_stock_data(&content, &from, &to).await?;
            Ok(csv_content)
        })
    }
}

pub struct DataLoadActor {}

impl Actor for DataLoadActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct DataLoadMessage {
    pub path: String,
}

impl Handler<DataLoadMessage> for DataLoadActor {
    type Result = ResponseFuture<Result<String, std::io::Error>>;
    fn handle(&mut self, msg: DataLoadMessage, _: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let path = msg.path;
            let content = tokio::fs::read_to_string(path).await?;
            Ok(content)
        })
    }
}

pub struct DataProcessActor {}

impl Actor for DataProcessActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<String, std::io::Error>")]
pub struct DataProcessMessage {
    pub content: String,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

impl Handler<DataProcessMessage> for DataProcessActor {
    type Result = ResponseFuture<Result<String, std::io::Error>>;
    fn handle(&mut self, msg: DataProcessMessage, _: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let content = msg.content;
            let from = msg.from;
            let to = msg.to;
            let csv_content = fetch_stock_data(&content, &from, &to).await?;
            Ok(csv_content)
        })
    }
}

pub struct DataSaveActor {}
impl Actor for DataSaveActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub struct DataSaveMessage {
    pub path: String,
    pub content: String,
}

impl Handler<DataSaveMessage> for DataSaveActor {
    type Result = ResponseFuture<Result<(), std::io::Error>>;
    fn handle(&mut self, msg: DataSaveMessage, _: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let path = msg.path;
            let content = msg.content;
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .await?;
            file.write_all(content.as_bytes()).await?;
            Ok(())
        })
    }
}
