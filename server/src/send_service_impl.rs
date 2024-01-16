use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::{
    api::{RegisterEventRequest, SendMessageRequest},
    message_queue::{self, Queue},
};

pub struct Service {
    queue: Arc<Mutex<Queue>>,
}

impl Service {
    pub fn new(q: Arc<Mutex<Queue>>) -> Service {
        Service { queue: q }
    }
    fn is_friend(&self, a: &str, b: &str) -> bool {
        true
    }

    fn is_authorized(&self, friend_id: &str, pub_key: &str) -> bool {
        true
    }

    pub fn send(&self, send_message_request: SendMessageRequest) -> Result<(), String> {
        if !self.is_friend(
            send_message_request.sender_id.as_str(),
            send_message_request.friend_id.as_str(),
        ) {
            return Err("you are not friend".to_string());
        }

        // if !self.is_authorized(
        //     send_message_request.reciever_id.as_str(),
        //     send_message_request.pub_key.as_str(),
        // ) {
        //     return Err("you are not authorized".to_string());
        // }
        self.queue
            .lock()
            .unwrap()
            .insert_msg(
                send_message_request.friend_id,
                send_message_request.sender_id,
            )
            .unwrap();

        self.queue.lock().unwrap().print();
        Ok(())
        //push msg in message queue
    }

    pub fn register_user(&self, register_user_request: RegisterEventRequest) -> Result<(), String> {
        self.queue
            .lock()
            .unwrap()
            .new_queue(register_user_request.phone)
            .unwrap();
        self.queue.lock().unwrap().print();
        Ok(())
    }
}
