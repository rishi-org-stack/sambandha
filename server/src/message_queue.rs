use std::collections::{HashMap, VecDeque};

use crate::models;

type UserQueue = HashMap<String, VecDeque<models::Message>>;

struct Queue {
    data: UserQueue,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            data: HashMap::new(),
        }
    }

    pub fn new_queue(&mut self, user_id: String) -> Result<(), String> {
        self.data.insert(user_id, VecDeque::new());
        Ok(())
    }

    pub fn insert_msg(&mut self, user_id: String, msg: models::Message) -> Result<(), String> {
        if !self.data.contains_key(&user_id) {
            return Err("user not found".to_string());
        }

        self.data.entry(user_id).and_modify(|v| v.push_back(msg));
        Ok(())
    }

    pub fn next_msg(&mut self, user_id: String) -> Result<Option<models::Message>, String> {
        if let Some(entry) = self.data.get_mut(&user_id) {
            if let Some(msg) = entry.pop_front() {
                Ok(Some(msg))
            } else {
                Ok(None)
            }
        } else {
            return Err("unhandled error".to_string());
        }
    }
}
