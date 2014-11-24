
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::comm::{channel, Receiver, Sender};
use std::cell::RefCell;
use servo_util::str::DOMString;
use std::collections::HashMap;

use servo_util::task::spawn_named;

pub enum StorageTaskMsg {
    /// Request the data associated with a particular URL
    Set(DOMString, DOMString),
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
    data: RefCell<HashMap<DOMString, DOMString>>,
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
              Set(name, value) => {
                self.set(name, value)
              }
              Exit => {
                break
              }
            }
        }
    }

    fn set(&self, name: DOMString, value: DOMString) {
        println!("communicated");
        println!("{:s} {:s}", name, value);
        self.data.borrow_mut().insert(name, value);
        for (key, value) in self.data.borrow().iter() {
            println!("key: {}; value: {}", key, value); 
        }
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
