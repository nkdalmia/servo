
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
    /// Request the storage data associated with a particular URL
    Length(Url),
    Key(Url, index),
    GetItem(Url, DOMString),
    SetItem(Url, DOMString, DOMString),
    RemoveItem(Url, DOMString),
    Clear(Url, DOMString),
    Exit
}

/// Handle to a storage task
pub type StorageTask = Sender<StorageTaskMsg>;

/// Create a StorageTask
pub fn new_storage_task(user_agent: Option<String>) -> StorageTask {
    println!("Creating Storage Task");
    let (setup_chan, setup_port) = channel();
    spawn_named("StorageManager", proc() {
        StorageManager::new(setup_port, user_agent).start();
    });
    setup_chan
}

struct StorageManager {
    from_client: Receiver<StorageTaskMsg>,
    user_agent: Option<String>,
    data: RefCell<HashMap<DOMString, RefCell<HashMap<DOMString, DOMString>>>>,
}

impl StorageManager {
    fn new(from_client: Receiver<StorageTaskMsg>, user_agent: Option<String>) -> StorageManager {
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
              SetItem(url,name, value) => {
                  self.SetItem(url, name, value)
              }
              GetItem(url,name) => {
                  self.GetItem(url, name)
              }
              Exit => {
                break
              }
            }
        }
    }

    fn getOriginMap(origin: Url) -> RefCell<HashMap<DOMString, DOMString>> {


    }

    fn setItem(&self,  url: Url, name: DOMString, value: DOMString) {
        println!("storage_task SET");
        println!("{:s} {:s} {:s}", url.to_string(), name, value);
        self.data.borrow_mut().insert(name, value);
        for (key, value) in self.data.borrow().iter() {
            println!("key: {}; value: {}", key, value);
        }
    }

    fn getItem(&self,  url: Url, name: DOMString) {
        println!("storage_task GET from {:s} | {:s}" ,url.to_string() ,name);
    }
}

#[test]
fn test_exit() {
    let storage_task = new_storage_task(None);
    storage_task.send(Exit);
}

#[test]
fn test_bad_scheme() {
    let storage_task = new_storage_task(None);
    storage_task.send(Set);
    storage_task.send(Exit);
}
