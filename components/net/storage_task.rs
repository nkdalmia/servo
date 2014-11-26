/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::RefCell;
use std::comm::{channel, Receiver, Sender};
use std::collections::HashMap;
use url::Url;

use servo_util::str::DOMString;
use servo_util::task::spawn_named;

pub enum StorageTaskMsg {
    // Request the storage data associated with a particular URL
    //Length(Url),
    //Key(Url, u32),
    GetItem(Sender<Option<DOMString>>, Url, DOMString),
    SetItem(Url, DOMString, DOMString),
    //RemoveItem(Url, DOMString),
    //Clear(Url, DOMString),
    Exit
}

// Handle to a storage task
pub type StorageTask = Sender<StorageTaskMsg>;

// Create a StorageTask
pub fn new_storage_task(user_agent: Option<String>) -> StorageTask {
    let (setup_chan, setup_port) = channel();
    spawn_named("StorageManager", proc() {
        StorageManager::new(setup_port, user_agent).start();
    });
    setup_chan
}

struct StorageManager {
    from_client: Receiver<StorageTaskMsg>,
    user_agent: Option<String>,
    data: RefCell<HashMap<String, RefCell<HashMap<DOMString, DOMString>>>>,
}

impl StorageManager {
    fn new(from_client: Receiver<StorageTaskMsg> , user_agent: Option<String>) -> StorageManager {
        StorageManager {
            from_client: from_client,
            user_agent: user_agent,
            data: RefCell::new(HashMap::new()),
        }
    }
}

impl StorageManager {
    fn start(&self) {
        loop {
            match self.from_client.recv() {
              SetItem(url, name, value) => {
                  self.set_item(url, name, value)
              }
              GetItem(sender, url, name) => {
                  self.get_item(sender, url, name)
              }
              Exit => {
                break
              }
            }
        }
    }

    fn set_item(&self,  url: Url, name: DOMString, value: DOMString) {
        if !self.data.borrow().contains_key(&(url.to_string())) {
            self.data.borrow_mut().insert(url.to_string(), RefCell::new(HashMap::new()));
        }

        match self.data.borrow().get(&(url.to_string())) {
            Some(origin_data) => {
                origin_data.borrow_mut().insert(name, value);
            }
            _ => {
            }
        }
        self.print_data();
    }

    fn get_item(&self, sender: Sender<Option<DOMString>>, url: Url, name: DOMString) {
        println!("storage_task GET from {:s} | {:s}", url.to_string(), name);
        match self.data.borrow().get(&(url.to_string())) {
            Some(origin_data) => {
                match origin_data.borrow().get(&name) {
                    Some(value) => sender.send(Some(value.to_string())),
                    None => sender.send(None),
                }
            }
            _ => {
            }
        }
    }

    //for testing purpose only
    fn print_data(&self) {
        println!("");
        println!("Printing Storage Data: Start..");
        for (origin, group) in self.data.borrow().iter() {
            println!("Origin: {}", origin);
            for (key, value) in group.borrow().iter() {
                println!("key: {}; value: {}", key, value);
            }
        }
        println!("Printing Storage Data: End.");
    }

}
