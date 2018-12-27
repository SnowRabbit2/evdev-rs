use nix::libc::c_char;
use raw;
use std::fmt;
use std::ffi::{CStr, CString};
use super::event::*;

pub(crate) fn ptr_to_str(ptr: *const c_char) -> Option<&'static str> {
    let slice : Option<&CStr> = unsafe {
        if ptr.is_null() {
            return None
        }
        Some(CStr::from_ptr(ptr))
    };

    match slice {
        None => None,
        Some(s) => {
            let buf : &[u8] = s.to_bytes();
            Some(std::str::from_utf8(buf).unwrap())
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ptr_to_str(unsafe {
            raw::libevdev_event_type_get_name(self.as_raw())
        }).unwrap_or(""))
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ev_type, ev_code) = self.as_raw();
        write!(f, "{}", ptr_to_str(unsafe {
            raw::libevdev_event_code_get_name(ev_type, ev_code)
        }).unwrap_or(""))
    }
}

impl fmt::Display for INPUT_PROP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ptr_to_str(unsafe {
            raw::libevdev_property_get_name(self.as_raw())
        }).unwrap_or(""))
    }
}

impl Type {

    /// The given type constant for the passed name or Errno if not found.
    pub fn from_str(name: &str) -> Option<Type> {
        let name = CString::new(name).unwrap();
        let result = unsafe {
            raw::libevdev_event_type_from_name(name.as_ptr())
        };

        match result {
            -1 => None,
             k => Some(Type::from_raw(k as TypeRaw)),
        }
    }

    /// The max value defined for the given event type, e.g. ABS_MAX for a type
    /// of EV_ABS, or Errno for an invalid type.
    pub fn get_max(r#type: Type) -> Option<i32> {
        let result = unsafe {
            raw::libevdev_event_type_get_max(r#type.as_raw())
        };
    
        match result {
            -1 => None,
             k => Some(k),
        }
    }
}

impl Code {

    /// Look up an event code by its type and name. Event codes start with a fixed
    /// prefix followed by their name (eg., "ABS_X"). The prefix must be included in
    /// the name. It returns the constant assigned to the event code or Errno if not
    /// found.
    pub fn from_str(r#type: Type, name: &str) -> Option<Code> {
        let name = CString::new(name).unwrap();
        let result = unsafe {
            raw::libevdev_event_code_from_name(r#type.as_raw(), name.as_ptr())
        };
    
        match result {
            -1 => None,
             k => Some(Code::from_raw(r#type, k as u32)),
        }
    }
}

impl INPUT_PROP {

    /// Look up an input property by its name. Properties start with the fixed
    /// prefix "INPUT_PROP_" followed by their name (eg., "INPUT_PROP_POINTER").
    /// The prefix must be included in the name. It returns the constant assigned
    /// to the property or Errno if not found.
    pub fn from_str(name: &str) -> Option<INPUT_PROP> {
        let name = CString::new(name).unwrap();
        let result = unsafe {
            raw::libevdev_property_from_name(name.as_ptr())
        };
    
        match result {
            -1 => None,
             k => INPUT_PROP::from_raw(k as u32),
        }
    }
}
