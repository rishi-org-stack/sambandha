struct SendMessage {
    sender_id: String,
    reciever_id: String,
    pub_key: String,
    content: Content,
    message_id: String,
    sent_on: u64,
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
    message_id: String,
    state: SentMsgState,
    ack_on: u64,
}

struct StatusUpdate {
    user_id: String,
    status: UserStatus,
    on: u64,
}
