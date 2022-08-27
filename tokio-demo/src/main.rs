use std::borrow::Borrow;
use std::time::Duration;

use log::{debug, error, info, LevelFilter, max_level, trace};
use tokio::{join, select, spawn};
use tokio::time::timeout;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{self, time};

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("./logs/", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .with_level(true)
        .with_line_number(false)
        .with_thread_names(false)
        .with_max_level(Level::TRACE)
        .init();
    let num = 999;
    let name = "test";
    // info!("hello world! num:{},name:{}", num, name);
    info!("{}","this is a test");
    trace!("hello world! num:{},name:{}", num, name);
    debug!("hello world! num:{},name:{}", num, name);
    info!("hello world! num:{},name:{}", num, name);
    error!("hello world! num:{},name:{}", num, name);
    hello().await;
    tokio::spawn(async move {
        tick2_thread(5).await;
    });
    spawn_thread().await;
    timeout_demo().await;
    waitgroup_demo().await;
    let one = tokio::spawn(async move {
        tick_thread(1).await;
    });
    let two = tokio::spawn(async move {
        tick_thread(2).await;
    });
    join!(one, two);
    tick_thread(3).await;

    info!("after join");
}

async fn spawn_thread() {
    let h = tokio::spawn(async move {
        info!("spawn thread {:?}","world!");
    });
    h.await.expect("TODO: panic message");
}

async fn tick_thread(i: i32) {
    let mut tick = tokio::time::interval(tokio::time::Duration::from_secs(1));
    loop {
        let t = tick.tick().await;
        info!("hello tick! {} {:?}",i,t.elapsed());
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn tick2_thread(i: i32) {
    let mut tick = tokio::time::interval(tokio::time::Duration::from_secs(1));
    loop {
        select! {
            _ = tick.tick() => {
                info!("hello tick2 demo {}",i);
            }
        }
    }
}

async fn timeout_demo() {
    let result = timeout(Duration::from_secs(1), hello()).await;
    info!("hello timeout! {:?}", result);
}

async fn waitgroup_demo() {
    let mut list = vec![];
    for i in 0..5 {
        let h = tokio::spawn(async move {
            info!("waiting on {}", i);
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
        list.push(h);
    }
    futures::future::join_all(list).await;
    info!("wait group done");
}

async fn hello() {
    info!("async hello!");
    debug!("async hello!");
}