pub mod pb {
    include!("gen/_.rs");
}

use std::time::Duration;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Channel, Request};

use pb::{
    messaging_service_client::MessagingServiceClient, RegisterEventRequest, SendMessageRequest,
    UserActionEvent,
};

fn greet_requests_iter(num: usize) -> impl Stream<Item = SendMessageRequest> {
    tokio_stream::iter(0..=num).map(|i| SendMessageRequest {
        sender_id: format!("rishi:{}", i),
        friend_id: "9874137031".to_string(),
        ..Default::default()
    })
}

async fn bidirectional_streaming_echo(client: &mut MessagingServiceClient<Channel>, num: usize) {
    let in_stream = greet_requests_iter(num);

    let response = client.send_event_handler(in_stream).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.state);
    }
}

async fn register_user(client: &mut MessagingServiceClient<Channel>) {
    client
        .register_event_handler(Request::new(RegisterEventRequest {
            name: "rishi".to_string(),
            phone: "9874137031".to_string(),
            bio: "engineer".to_string(),
        }))
        .await
        .unwrap();
}

async fn server_streaming_messag(client: &mut MessagingServiceClient<Channel>) {
    let response = client
        .recieve_msg_event_handler(UserActionEvent {
            user_id: "9874137031".to_string(),
            status: 1,
            ..Default::default()
        })
        .await
        .unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{:#?}`", received);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessagingServiceClient::connect("http://0.0.0.0:3000")
        .await
        .unwrap();

    println!("Streaming echo:");
    tokio::time::sleep(Duration::from_secs(1)).await;
    // register_user(&mut client).await;
    println!("\r\nBidirectional stream echo:");
    // bidirectional_streaming_echo(&mut client, 5).await;
    server_streaming_messag(&mut client).await;

    Ok(())
}
