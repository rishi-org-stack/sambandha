use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

type UserQueue = HashMap<String, VecDeque<String>>;

pub struct Queue {
    data: Arc<Mutex<UserQueue>>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            data: Arc::new(Mutex::new(UserQueue::new())),
        }
    }

    pub fn new_queue(&self, user_id: String) -> Result<(), String> {
        self.data.lock().unwrap().insert(user_id, VecDeque::new());
        Ok(())
    }

    pub fn insert_msg(&mut self, user_id: String, msg: String) -> Result<(), String> {
        if !self.data.lock().unwrap().contains_key(&user_id) {
            return Err("user not found".to_string());
        }

        self.data
            .lock()
            .unwrap()
            .entry(user_id)
            .and_modify(|v| v.push_back(msg));
        Ok(())
    }

    pub fn next_msg(&mut self, user_id: String) -> Result<Option<String>, String> {
        if let Some(entry) = self.data.lock().unwrap().get_mut(&user_id) {
            if let Some(msg) = entry.pop_front() {
                Ok(Some(msg))
            } else {
                Ok(None)
            }
        } else {
            return Err("unhandled error".to_string());
        }
    }

    pub fn print(&self) {
        println!("queue: {:#?}", self.data.lock().unwrap())
    }
}
