use std::{collections::VecDeque, fs};

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

            let final_content = if fs::metadata(&path).is_ok() {
                format!("{}\n", content)
            } else {
                format!(
                    "period start,symbol,price,change %,min,max,30d avg\n{}\n",
                    content
                )
            };
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .await?;
            file.write_all(final_content.as_bytes()).await?;
            Ok(())
        })
    }
}

pub struct BufferActor {
    buffer: VecDeque<String>,
    max_buffer_size: usize,
}

impl BufferActor {
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(max_buffer_size),
            max_buffer_size,
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), std::io::Error>")]
pub struct BufferMessage {
    pub content: String,
}

impl Actor for BufferActor {
    type Context = Context<Self>;
}

impl Handler<BufferMessage> for BufferActor {
    type Result = Result<(), std::io::Error>;
    fn handle(&mut self, msg: BufferMessage, _: &mut Self::Context) -> Self::Result {
        if self.buffer.len() == self.max_buffer_size {
            self.buffer.pop_back();
        }
        self.buffer.push_front(msg.content);
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec< String >, std::io::Error>")]
pub struct BufferReadMessage {
    pub n: usize,
}

impl Handler<BufferReadMessage> for BufferActor {
    type Result = Result<Vec<String>, std::io::Error>;
    fn handle(&mut self, msg: BufferReadMessage, _: &mut Self::Context) -> Self::Result {
        let mut content = vec![];
        for _ in 0..msg.n {
            if let Some(line) = self.buffer.pop_front() {
                content.push(line);
            } else {
                break;
            }
        }
        Ok(content)
    }
}
