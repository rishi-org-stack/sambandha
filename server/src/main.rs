pub mod message_queue;
pub mod models;
pub mod send_service_impl;
pub mod api {
    include!("gen/_.rs");
}
use std::pin::Pin;

use api::{
    send_service_server::{SendService, SendServiceServer},
    SendAckResponse, SendMessageRequest,
};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

#[derive(Default)]
pub struct SendHandler {}
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
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
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
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as Self::SendMsgStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let greeter = SendHandler::default();
    let greet_server = SendServiceServer::new(greeter);

    println!("GreeterServer listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(greet_server)
        .serve(addr)
        .await?;

    Ok(())
}
