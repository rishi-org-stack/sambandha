struct SendType {
    sender_id: Vec<u8>,
    reciever_id: Vec<u8>,
    pub_key: Vec<u8>,
    content: Content,
    message_id: Vec<u8>,
    sent_on: u128,
}

struct Content {
    size: u16,
    data: Vec<u128>,
}
enum SentMsgState {
    Delivered,
    Read,
}

enum UserStatus {
    Online,
    Offline,
}
struct SentAck {
    message_id: Vec<u8>,
    state: SentMsgState,
    ack_on: u128,
}

struct StatusUpdate {
    user_id: Vec<u8>,
    status: UserStatus,
    on: u128,
}

struct Message {
    sender_id: Vec<u8>,
    message_id: Vec<u8>,
    content: Content,
}

fn main() {}
