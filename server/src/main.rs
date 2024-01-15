pub mod message_queue;
pub mod models;
pub mod send_service_impl;
pub mod api {
    include!("gen/_.rs");
}
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
};

use api::{
    send_service_server::{SendService, SendServiceServer},
    SendAckResponse, SendMessageRequest,
};
use message_queue::Queue;
use send_service_impl::Service;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

// #[derive()]
pub struct SendHandler {
    svc: send_service_impl::Service,
}
impl SendHandler {
    fn new() -> SendHandler {
        let queue = Arc::new(Mutex::new(Queue::new()));
        let svc = Service::new(queue.clone());
        // SendHandler { svc: Arc::new(svc) }
        SendHandler { svc }
    }
}
// impl SendHandler {
//     fn new() -> SendHandler {
//         let queue = Rc::new(RefCell::new(Queue::new()));
//         let svc = Service::new(queue.clone());
//         SendHandler { svc }
//     }
// }

type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendAckResponse, Status>> + Send>>;
#[tonic::async_trait]
impl SendService for SendHandler {
    type SendMsgStream = ResponseStream;
    async fn send_msg(
        &self,
        request: tonic::Request<tonic::Streaming<SendMessageRequest>>,
    ) -> std::result::Result<tonic::Response<Self::SendMsgStream>, tonic::Status> {
        println!("EchoServer::bidirectional_streaming_echo");

        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be drooped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        while let Some(result) = in_stream.next().await {
            match result {
                Ok(v) => {
                    // let x = self.borrow_mut();
                    self.svc.send(v).expect("msg");
                    // .svc
                    // .send(v)
                    // .expect("failed to send msg");
                    tx.send(Ok(SendAckResponse::default()))
                        .await
                        .expect("working rx");
                }
                Err(err) => {
                    println!("err= {}", err);

                    match tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break, // response was droped
                    }
                }
            }
        }
        println!("\tstream ended");

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as Self::SendMsgStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let greeter = SendHandler::new();
    let greet_server = SendServiceServer::new(greeter);

    println!("GreeterServer listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(greet_server)
        .serve(addr)
        .await?;

    Ok(())
}
