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

#[dom_struct]
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
}

impl<'a> StorageMethods for JSRef<'a, Storage> {
    fn Length(self) -> u32 {
        0
    }

    fn Key(self, index: u32) -> Option<DOMString> {

        //Return null for out of range index
        if index >= self.Length() {
            return None;
        }

        return None;
    }

    fn GetItem(self, name: DOMString) -> Option<DOMString> {
        if name.is_empty() {
            return None;
        }

        return None;
    }

    fn NamedGetter(self, name: DOMString, found: &mut bool) -> Option<DOMString> {
        let item = self.GetItem(name);
        *found = item.is_some();
        item
    }

    fn SetItem(self, name: DOMString, value: DOMString) {
        if name.is_empty() {
            println!("Name-Value pair: {:s} {:s}", name, value);
        } else {
            println!("Name-Value pair: {:s} {:s}", name, value);
            let global_root = self.global.root(); //.resoure_task();
            let global_ref = global_root.root_ref();
            //let win = global_ref.as_window();
            let storage_task = global_ref.storage_task();
            storage_task.send(StorageTaskMsg::Set(name, value));
        }

    }

    fn NamedSetter(self, name: DOMString, value: DOMString) {
        self.SetItem(name, value);
    }

    fn NamedCreator(self, name: DOMString, value: DOMString) {
        self.SetItem(name, value);
    }

    fn RemoveItem(self, name: DOMString) {
        if name.is_empty() {
            ;
        }
    }

    fn NamedDeleter(self, name: DOMString) {
        self.RemoveItem(name);
    }

    fn Clear(self) {
    }
}

impl Reflectable for Storage {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        &self.reflector_
    }
}
