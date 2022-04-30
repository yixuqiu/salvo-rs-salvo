// Copyright (c) 2018-2020 Sean McArthur
// Licensed under the MIT license http://opensource.org/licenses/MIT
//port from https://github.com/seanmonstar/warp/blob/master/examples/sse.rs

use std::convert::Infallible;
use std::time::Duration;

use futures_util::StreamExt;
use salvo::extra::sse::{self, SseEvent};
use salvo::prelude::*;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

// create server-sent event
fn sse_counter(counter: u64) -> Result<SseEvent, Infallible> {
    Ok(SseEvent::default().data(counter.to_string()))
}

#[fn_handler]
async fn handle_tick(_req: &mut Request, res: &mut Response) {
    let event_stream = {
        let mut counter: u64 = 0;
        let interval = interval(Duration::from_secs(1));
        let stream = IntervalStream::new(interval);
        stream.map(move |_| {
            counter += 1;
            sse_counter(counter)
        })
    };
    sse::streaming(res, event_stream).ok();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::with_path("ticks").get(handle_tick);
    tracing::info!("Listening on http://0.0.0.0:7878");
    Server::new(TcpListener::bind("0.0.0.0:7878")).serve(router).await;
}