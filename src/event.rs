use nix::libc::{c_uint, c_int};

pub type TypeRaw = c_uint;
pub type CodeValRaw = c_uint;
pub type CodeRaw = (TypeRaw, CodeValRaw);
pub type Value = c_int;

pub trait AsCodeRaw {
    fn as_code_raw(&self) -> CodeRaw;
}

pub trait AsCode {
    fn as_code(&self) -> Code;
}

include!("event-codes.rs");

impl AsCodeRaw for CodeRaw {
    fn as_code_raw(&self) -> CodeRaw {
        *self
    }
}

impl AsCodeRaw for Code {
    fn as_code_raw(&self) -> CodeRaw {
        self.as_raw()
    }
}

impl AsCode for Code {
    fn as_code(&self) -> Code {
        *self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeVal {
   pub seconds: i64,
   pub microseconds: i64,
}

impl TimeVal {
    fn from_raw(raw: nix::libc::timeval) -> TimeVal {
        TimeVal {
            seconds: raw.tv_sec,
            microseconds: raw.tv_usec,
        }
    }
    fn as_raw(&self) -> nix::libc::timeval {
        nix::libc::timeval {
            tv_sec: self.seconds,
            tv_usec: self.microseconds,
        }
    }
}

/// The event structure itself
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    /// The time at which event occured
    pub time: TimeVal,
    pub code: Code,
    pub value: Value,
}

impl Event {
    pub(crate) fn from_raw(raw: raw::input_event) -> Event {
        Event {
            time: TimeVal::from_raw(raw.time),
            code: Code::from_raw(Type::from_raw(raw.event_type as TypeRaw), raw.event_code as c_uint),
            value: raw.value,
        }
    }

    pub(crate) fn as_raw(&self) -> raw::input_event {
        let (r#type, ev_code) = self.code.as_code_raw();
        raw::input_event {
            time: self.time.as_raw(),
            event_type: r#type as u16,
            event_code: ev_code as u16,
            value: self.value,
        }
    }

    pub fn is_type(&self, r#type: &Type) -> bool {
        let ev = self.as_raw();
        unsafe {
            raw::libevdev_event_is_type(&ev, r#type.as_raw()) == 1
        }
    }

    pub fn is_code<C: AsCodeRaw>(&self, code: C) -> bool {
        let ev = self.as_raw();
        let (check_type, check_code) = code.as_code_raw();
        
        unsafe {
            raw::libevdev_event_is_code(&ev, check_type, check_code) == 1
        }
    }
}