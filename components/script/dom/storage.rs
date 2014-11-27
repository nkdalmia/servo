/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::StorageBinding;
use dom::bindings::codegen::Bindings::StorageBinding::StorageMethods;
use dom::bindings::global::{GlobalRef, GlobalField};
use dom::bindings::js::{JSRef, Temporary};
use dom::bindings::utils::{Reflectable, Reflector, reflect_dom_object};
use servo_util::str::DOMString;
use servo_net::storage_task::StorageTaskMsg;
use std::comm::{channel, Receiver, Sender};
use url::Url;

#
[dom_struct]
pub struct Storage {
    reflector_: Reflector,
    global: GlobalField,
}

impl Storage {
    fn new_inherited(global: &GlobalRef) -> Storage {
        Storage {
            reflector_: Reflector::new(),
            global: GlobalField::from_rooted(global),
        }
    }

    pub fn new(global: &GlobalRef) -> Temporary<Storage> {
        reflect_dom_object(box Storage::new_inherited(global), global, StorageBinding::Wrap)
    }

    fn get_origin_as_string(&self, url: Url) -> String {
      let mut origin = "".to_string();
      origin.push_str(url.scheme.as_slice());
      origin.push_str("://");
      if url.domain() != None {
        origin.push_str(url.domain().unwrap().as_slice());
      }
      origin.push_str("/");
      if url.port() != None {
        origin.push_str(url.port().unwrap().to_string().as_slice());
      }
      return origin;
    }
}

impl<'a> StorageMethods for JSRef<'a, Storage> {
  fn Length(self) -> u32 {
    0
  }

  fn Key(self, index: u32) -> Option<DOMString> {
    //Return null for out of range index
    if index>= self.Length() {
      return None;
    }
    return None;
  }

  fn GetItem(self, name: DOMString) -> Option<DOMString> {
    /* Create a new Channel */
    let (sender, receiver): (Sender<Option<DOMString>>, Receiver<Option<DOMString>>) = channel();

    //Move to another function?
    /*Retrieve storage task instance */
    let global_root = self.global.root();
    let global_ref = global_root.root_ref();
    let url = global_ref.get_url();
    let origin = self.get_origin_as_string(url.clone());
    let storage_task = global_ref.storage_task();

    /* Send Get Request on Storage Task Channel */
    storage_task.send(StorageTaskMsg::GetItem(sender.clone(), origin.clone(), name));
    /* Wait for Reply on Self Channel */
    let item = receiver.recv();
    item
  }

  fn NamedGetter(self, name: DOMString, found: &mut bool) -> Option<DOMString> {
    let item = self.GetItem(name);
    *found = item.is_some();
    item
  }

  fn SetItem(self, name: DOMString, value: DOMString) {
    //update value only if the given name/value pair does not exist
    let item = self.GetItem(name.clone());
    if !item.is_some() || item.unwrap().as_slice() != value.as_slice() {
      let global_root = self.global.root();
      let global_ref = global_root.root_ref();
      let storage_task = global_ref.storage_task();
      let url = global_ref.get_url();
      let origin = self.get_origin_as_string(url.clone());
      storage_task.send(StorageTaskMsg::SetItem(origin.clone(), name, value));
    }
  }

  fn NamedSetter(self, name: DOMString, value: DOMString) {
    self.SetItem(name, value);
  }

  fn NamedCreator(self, name: DOMString, value: DOMString) {
    self.SetItem(name, value);
  }

  fn RemoveItem(self, name: DOMString) {
    if name.is_empty() {;
    }
  }

  fn NamedDeleter(self, name: DOMString) {
    self.RemoveItem(name);
  }

  fn Clear(self) {}
}

impl Reflectable for Storage {
  fn reflector<'a>(&'a self) -> &'a Reflector {
    &self.reflector_
  }
}
