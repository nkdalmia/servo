/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::RefCell;
use std::comm::{channel, Receiver, Sender};
use std::collections::HashMap;

use servo_util::str::DOMString;
use servo_util::task::spawn_named;

pub enum StorageTaskMsg {
    // Request the storage data associated with a particular origin
    //Length(String),
    //Key(String, u32),
    GetItem(Sender<Option<DOMString>>, String, DOMString),
    SetItem(String, DOMString, DOMString),
    //RemoveItem(String, DOMString),
    //Clear(String, DOMString),
    Exit
}

// Handle to a storage task
pub type StorageTask = Sender<StorageTaskMsg>;

// Create a StorageTask
pub fn new_storage_task() -> StorageTask {
    let (chan, port) = channel();
    spawn_named("StorageManager", proc() {
        StorageManager::new(port).start();
    });
    chan
}

struct StorageManager {
    port: Receiver<StorageTaskMsg>,
    data: RefCell<HashMap<String, RefCell<HashMap<DOMString, DOMString>>>>,
}

impl StorageManager {
    fn new(port: Receiver<StorageTaskMsg>) -> StorageManager {
        StorageManager {
            port: port,
            data: RefCell::new(HashMap::new()),
        }
    }
}

impl StorageManager {
    fn start(&self) {
        loop {
            match self.port.recv() {
              SetItem(origin, name, value) => {
                  self.set_item(origin, name, value)
              }
              GetItem(sender, origin, name) => {
                  self.get_item(sender, origin, name)
              }
              Exit => {
                break
              }
            }
        }
    }

    fn set_item(&self,  origin: String, name: DOMString, value: DOMString) {
        if !self.data.borrow().contains_key(&origin) {
            self.data.borrow_mut().insert(origin.clone(), RefCell::new(HashMap::new()));
        }

        match self.data.borrow().get(&origin) {
            Some(origin_data) => {
                origin_data.borrow_mut().insert(name, value);
            }
            _ => {
            }
        }
        self.print_data();
    }

    fn get_item(&self, sender: Sender<Option<DOMString>>, origin: String, name: DOMString) {
        println!("storage_task GET from {:s} | {:s}", origin, name);
        match self.data.borrow().get(&origin) {
            Some(origin_data) => {
                match origin_data.borrow().get(&name) {
                    Some(value) => sender.send(Some(value.to_string())),
                    None => sender.send(None),
                }
            }
            None => sender.send(None),
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
