
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::EventBinding::EventMethods;
use dom::bindings::codegen::Bindings::StorageEventBinding;
use dom::bindings::codegen::Bindings::StorageEventBinding::StorageEventMethods;
use dom::bindings::codegen::InheritTypes::{EventCast, StorageEventDerived};
use dom::bindings::error::Fallible;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JSRef, Temporary};
use dom::bindings::utils::{Reflectable, Reflector, reflect_dom_object};
use dom::event::{Event, StorageEventTypeId};
use dom::storage::Storage;

use servo_util::str::DOMString;

#[dom_struct]
pub struct StorageEvent {
    event: Event,
    key: Option<Option<DOMString>>,
    old_value: Option<Option<DOMString>>,
    new_value: Option<Option<DOMString>>,
    url: Option<DOMString>,
}

impl StorageEventDerived for Event {
    fn is_storageevent(&self) -> bool {
        *self.type_id() == StorageEventTypeId
    }
}

impl StorageEvent {
    fn new_inherited(key: Option<Option<DOMString>>, old_value: Option<Option<DOMString>>,
                     new_value: Option<Option<DOMString>>, url: Option<DOMString>)
                         -> StorageEvent {
        StorageEvent {
            event: Event::new_inherited(StorageEventTypeId),
            key: key,
            old_value: old_value,
            new_value: new_value,
            url: url,
        }
    }

    pub fn new(global: GlobalRef, type_: DOMString, bubbles: bool, cancelable: bool,
               key: Option<Option<DOMString>>, old_value: Option<Option<DOMString>>,
               new_value: Option<Option<DOMString>>, url: Option<DOMString>)
               -> Temporary<StorageEvent> {
                   println!("storage event initialised");
        let ev = reflect_dom_object(box StorageEvent::new_inherited(key, old_value, new_value, url),
                                    global,
                                    StorageEventBinding::Wrap).root();
        let event: JSRef<Event> = EventCast::from_ref(*ev);
        event.InitEvent(type_, bubbles, cancelable);
        Temporary::from_rooted(*ev)
    }

    pub fn Constructor(global: &GlobalRef,
                       type_: DOMString,
                       init: &StorageEventBinding::StorageEventInit)
                       -> Fallible<Temporary<StorageEvent>> {
        let ev = StorageEvent::new(*global, type_, init.parent.bubbles, init.parent.cancelable,
                                   init.key.clone(), init.oldValue.clone(), init.newValue.clone(), init.url.clone());
        Ok(ev)
    }
}

impl<'a> StorageEventMethods for JSRef<'a, StorageEvent> {
    fn GetKey(self) -> Option<DOMString> {
        self.key.clone().and_then(|item| item)
    }

    fn GetOldValue(self) -> Option<DOMString> {
        self.old_value.clone().and_then(|item| item)
    }

    fn GetNewValue(self) -> Option<DOMString> {
        self.new_value.clone().and_then(|item| item)
    }

    fn Url(self) -> DOMString {
        self.url.clone().map_or("".to_string(), (|item| item))
    }

    fn GetStorageArea(self) -> Option<Temporary<Storage>> {
        None
    }
}

impl Reflectable for StorageEvent {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.event.reflector()
    }
}
