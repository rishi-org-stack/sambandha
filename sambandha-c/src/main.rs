pub mod pb {
    include!("gen/_.rs");
}

use std::time::Duration;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Channel;

use pb::{send_service_client::SendServiceClient, Content, SendMessageRequest};

fn greet_requests_iter(num: usize) -> impl Stream<Item = SendMessageRequest> {
    tokio_stream::iter(0..=num).map(|i| SendMessageRequest {
        sender_id: format!("rishi:{}", i),
        reciever_id: "sabitahf".to_string(),
        ..Default::default()
    })
}

async fn bidirectional_streaming_echo(client: &mut SendServiceClient<Channel>, num: usize) {
    let in_stream = greet_requests_iter(num);

    let response = client.send_msg(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.state);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SendServiceClient::connect("http://0.0.0.0:3000")
        .await
        .unwrap();

    println!("Streaming echo:");
    tokio::time::sleep(Duration::from_secs(1)).await; //do not mess server println functions

    // Echo stream that sends 17 requests then graceful end that connection
    println!("\r\nBidirectional stream echo:");
    bidirectional_streaming_echo(&mut client, 5).await;

    Ok(())
}
