use block::ConcreteBlock;

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, nil, NSString};

/// An EventMask describes the type of event.
#[derive(Debug)]
pub enum EventMask {
    KeyDown
}

/// A wrapper over an `NSEvent`.
#[derive(Debug)]
pub struct EventMonitor(pub Id<Object>);

/// A wrapper over an `NSEvent`.
#[derive(Debug)]
pub struct Event(pub Id<Object>);

impl Event {
    pub(crate) fn new(objc: id) -> Self {
        Event(unsafe { Id::from_ptr(objc) })
    }

    pub fn characters(&self) -> String {
        // @TODO: Check here if key event, invalid otherwise.
        // @TODO: Figure out if we can just return &str here, since the Objective-C side
        // should... make it work, I think.
        let characters = NSString::retain(unsafe { msg_send![&*self.0, characters] });

        characters.to_string()
    }

    /*pub fn contains_modifier_flags(&self, flags: &[EventModifierFlag]) -> bool {
        let modifier_flags: NSUInteger = unsafe {
            msg_send![&*self.0, modifierFlags]
        };

        for flag in flags {
            let f: NSUInteger = flag.into();

        }

        false
    }*/

    /// Register an event handler with the system event stream. This method
    /// watches for events that occur _within the application_. Events outside
    /// of the application require installing a `monitor_global_events` handler.
    ///
    /// Note that in order to monitor all possible events, both local and global
    /// monitors are required - the streams don't mix.
    pub fn local_monitor<F>(_mask: EventMask, handler: F) -> EventMonitor
    where
        F: Fn(Event) -> Option<Event> + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |event: id| {
            let evt = Event::new(event);

            match handler(evt) {
                Some(mut evt) => &mut *evt.0,
                None => nil
            }
        });
        let block = block.copy();

        EventMonitor(unsafe {
            msg_send![class!(NSEvent), addLocalMonitorForEventsMatchingMask:1024
                handler:block]
        })
    }
}

use crate::foundation::NSUInteger;

#[derive(Clone, Copy, Debug)]
pub enum EventModifierFlag {
    CapsLock,
    Control,
    Option,
    Command,
    DeviceIndependentFlagsMask
}

impl From<EventModifierFlag> for NSUInteger {
    fn from(flag: EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000
        }
    }
}

impl From<&EventModifierFlag> for NSUInteger {
    fn from(flag: &EventModifierFlag) -> NSUInteger {
        match flag {
            EventModifierFlag::CapsLock => 1 << 16,
            EventModifierFlag::Control => 1 << 18,
            EventModifierFlag::Option => 1 << 19,
            EventModifierFlag::Command => 1 << 20,
            EventModifierFlag::DeviceIndependentFlagsMask => 0xffff0000
        }
    }
}
