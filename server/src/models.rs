struct Content {
    size: u32,
    data: String,
}

pub struct Message {
    sender_id: String,
    message_id: String,
    content: Content,
}
