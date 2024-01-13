// pub mod api {
//     include!("gen/_.rs");
// }
// use std::pin::Pin;

// use api::{
//     greet_server::{Greet, GreetServer},
//     GreetDayRequest, GreetDayResponse,
// };
// use tokio::sync::mpsc;
// use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
// use tonic::{Request, Response, Status, Streaming};

// #[derive(Default)]
// pub struct Greetings {}
// type ResponseStream = Pin<Box<dyn Stream<Item = Result<GreetDayResponse, Status>> + Send>>;
// #[tonic::async_trait]
// impl Greet for Greetings {
//     type GreetMeStream = ResponseStream;
//     async fn greet_me(
//         &self,
//         request: tonic::Request<tonic::Streaming<GreetDayRequest>>,
//     ) -> std::result::Result<tonic::Response<Self::GreetMeStream>, tonic::Status> {
//         println!("EchoServer::bidirectional_streaming_echo");

//         let mut in_stream = request.into_inner();
//         let (tx, rx) = mpsc::channel(128);

//         // this spawn here is required if you want to handle connection error.
//         // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
//         // will be drooped when connection error occurs and error will never be propagated
//         // to mapped version of `in_stream`.
//         tokio::spawn(async move {
//             while let Some(result) = in_stream.next().await {
//                 match result {
//                     Ok(v) => {
//                         tx.send(Ok(GreetDayResponse { msg: v.name }))
//                             .await
//                             .expect("working rx");
//                     }
//                     Err(err) => {
//                         // if let Some(io_err) = match_for_io_error(&err) {
//                         //     if io_err.kind() == ErrorKind::BrokenPipe {
//                         //         // here you can handle special case when client
//                         //         // disconnected in unexpected way
//                         //         eprintln!("\tclient disconnected: broken pipe");
//                         //         break;
//                         // }
//                         println!("err= {}", err);
//                         // }

//                         match tx.send(Err(err)).await {
//                             Ok(_) => (),
//                             Err(_err) => break, // response was droped
//                         }
//                     }
//                 }
//             }
//             println!("\tstream ended");
//         });

//         // echo just write the same data that was received
//         let out_stream = ReceiverStream::new(rx);

//         Ok(Response::new(Box::pin(out_stream) as Self::GreetMeStream))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // tracing_subscriber::fmt::init();

//     let addr = "127.0.0.1:3000".parse().unwrap();

//     let greeter = Greetings::default();
//     let greet_server = GreetServer::new(greeter);

//     println!("GreeterServer listening on {}", addr);

//     tonic::transport::Server::builder()
//         .add_service(greet_server)
//         .serve(addr)
//         .await?;

//     Ok(())
// }
