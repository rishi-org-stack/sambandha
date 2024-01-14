use crate::api::SendMessageRequest;

struct Service {}

impl Service {
    fn is_friend(&self, a: &str, b: &str) -> bool {
        todo!()
    }

    fn is_authorized(&self, friend_id: &str, pub_key: &str) -> bool {
        todo!()
    }

    fn send(&self, send_message_request: SendMessageRequest) {
        if !self.is_friend(
            send_message_request.sender_id.as_str(),
            send_message_request.reciever_id.as_str(),
        ) {}

        if !self.is_authorized(
            send_message_request.reciever_id.as_str(),
            send_message_request.pub_key.as_str(),
        ) {}

        //push msg in message queue
    }
}
