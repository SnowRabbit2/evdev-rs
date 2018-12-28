//! Rust bindings to libevdev, an wrapper for evdev devices.
//!
//! This library intends to provide a safe interface to the libevdev library. It
//! will look for the library on the local system, and link to the installed copy.
//!
//! # Examples
//!
//! ## Intializing a evdev device
//!
//! ```
//! use evdev_rs::Device;
//! use std::fs::File;
//!
//! let f = File::open("/dev/input/event0").unwrap();
//!
//! let mut d = Device::new().unwrap();
//! d.set_fd(&f).unwrap();
//! ```
//!
//! ## Getting the next event
//!
//! ```rust,no_run
//! use evdev_rs::Device;
//! use std::fs::File;
//!
//! let f = File::open("/dev/input/event0").unwrap();
//!
//! let mut d = Device::new().unwrap();
//! d.set_fd(&f).unwrap();
//!
//! loop {
//!     let a = d.next_event(evdev_rs::NORMAL | evdev_rs::BLOCKING);
//!     match a {
//!         Ok(k) => println!("Event: time {}.{}, ++++++++++++++++++++ {} +++++++++++++++",
//!				              k.1.time.tv_sec,
//!				              k.1.time.tv_usec,
//!				              k.1.event_type),
//!         Err(e) => (),
//!     }
//! }
//! ```

extern crate evdev_sys as raw;
extern crate nix;
#[macro_use]
extern crate bitflags;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod event;
pub mod logging;
pub mod util;
#[macro_use]
mod macros;
pub mod uinput;

use nix::libc;
use nix::libc::{c_int, c_uint, c_void};
use nix::errno::Errno;
use std::any::Any;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::{RawFd, IntoRawFd, AsRawFd, FromRawFd};

use event::*;
use util::*;

pub enum GrabMode {
    /// Grab the device if not currently grabbed
    Grab = raw::LIBEVDEV_GRAB as isize,
    /// Ungrab the device if currently grabbed
    Ungrab = raw::LIBEVDEV_UNGRAB as isize,
}

bitflags! {
    pub struct ReadFlag: u32 {
        /// Process data in sync mode
        const SYNC = 1;
        /// Process data in normal mode
        const NORMAL = 2;
        /// Pretend the next event is a SYN_DROPPED and require the
        /// caller to sync
        const FORCE_SYNC = 4;
        /// The fd is not in O_NONBLOCK and a read may block
        const BLOCKING = 8;
    }
}

#[derive(PartialEq)]
pub enum ReadStatus {
    /// `next_event` has finished without an error and an event is available
    /// for processing.
    Success = raw::LIBEVDEV_READ_STATUS_SUCCESS as isize,
    /// Depending on the `next_event` read flag:
	/// libevdev received a SYN_DROPPED from the device, and the caller should
	/// now resync the device, or, an event has been read in sync mode.
    Sync = raw::LIBEVDEV_READ_STATUS_SYNC as isize,
}

pub enum LedState {
    /// Turn the LED on
    On = raw::LIBEVDEV_LED_ON as isize,
    /// Turn the LED off
    Off = raw::LIBEVDEV_LED_OFF as isize,
}

#[derive(Debug)]
pub struct DeviceId {
    pub bustype: BUS,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

#[derive(Debug)]
/// used by EVIOCGABS/EVIOCSABS ioctls
pub struct AbsInfo {
    /// latest reported value for the axis
    pub value: i32,
    /// specifies minimum value for the axis
    pub minimum: i32,
    /// specifies maximum value for the axis
    pub maximum: i32,
    /// specifies fuzz value that is used to filter noise from
    /// the event stream
    pub fuzz: i32,
    /// values that are within this value will be discarded by
    /// joydev interface and reported as 0 instead
    pub flat: i32,
    /// specifies resolution for the values reported for
    /// the axis
    pub resolution: i32,
}

/// Opaque struct representing an evdev device
pub struct Device {
    raw: *mut raw::libevdev,
    owning: bool,
}

#[derive(Debug)]
pub enum Error {
    Unspecified,
    Errno(nix::errno::Errno),
    NotAnAxis,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Device {
    /// Initialize a new libevdev device.
    ///
    /// This function only initializesthe struct to sane default values.
    /// To actually hook up the device to a kernel device, use `set_fd`.
    pub fn new() -> Option<Device> {
        let libevdev = unsafe {
            raw::libevdev_new()
        };

        if libevdev.is_null() {
            None
        }
        else {
            Some(Device {
                raw: libevdev,
                owning: false
            })
        }
    }

    fn new_from_fd(fd: RawFd, owning: bool) -> Result<Device> {
        let mut libevdev = 0 as *mut _;
        try_errno!(unsafe {
            raw::libevdev_new_from_fd(fd, &mut libevdev)
        });
        Ok(Device { raw: libevdev, owning })
    }

    /// Initialize a new libevdev device from the given IntoRawFd.
    ///
    /// The Device instance takes ownership of the file descriptor,
    /// the file descriptor will be close when the Device instance goes out of scope.
    /// See new_from_file_borrowing for a borrowing version.
    /// 
    /// This is essentially a shortcut for
    ///
    /// ```
    /// use evdev_rs::Device;
    ///
    /// let mut device = Device::new().unwrap();
    /// device.set_file_owning(file);
    /// ```
    pub fn new_from_file_owning<F: IntoRawFd>(file: F) -> Result<Device> {
        Self::new_from_fd(file.into_raw_fd(), true)
    }

    /// Initialize a new libevdev device from the given AsRawFd.
    ///
    /// The Device instance does not takes ownership of the file descriptor,
    /// the file descriptor will not be close when the Device instance goes out of scope.
    /// See new_from_file_owning for a version taking ownership.
    ///     
    /// This is a essentally a shortcut for
    ///
    /// ```
    /// use evdev_rs::Device;
    ///
    /// let mut device = Device::new().unwrap();
    /// device.set_file_borrowing(&file);
    /// ```
    pub fn new_from_file_borrowing<F: AsRawFd>(file: &F) -> Result<Device> {
        Self::new_from_fd(file.as_raw_fd(), false)
    }

    string_getter!(name, libevdev_get_name,
                   phys, libevdev_get_phys,
                   uniq, libevdev_get_uniq);
    string_setter!(set_name, libevdev_set_name,
                   set_phys, libevdev_set_phys,
                   set_uniq, libevdev_set_uniq);

    /// Returns the file descriptor associated with the device
    ///
    /// If the file descriptor hasn't been set yet then it return `None`
    /// 
    /// This function is marked as unsafe has it returns the file descriptor
    /// independently of the fact that it is owned by the device instance or not.
    /// Use with care.
    pub unsafe fn fd(&self) -> Option<RawFd> {
        match raw::libevdev_get_fd(self.raw) {
            0  => None,
            fd => Some(fd)
        }
    }

    /// Consumes this Device instance and returns the associated file
    ///
    /// If the file has not been set as owned by the Device instance
    /// (using new_from_file_owning or set_file_owning) then it return `None`
    pub fn into_file(self) -> Option<File> {
        if self.owning {
            unsafe {
                self.fd().map(|fd| File::from_raw_fd(fd))
            }
        }
        else {
            None
        }
    }

    /// Returns a (dup'd) clone of the associated file
    ///
    /// If the file has not been set (using new_from_file_* or set_file_*)
    /// then it return `None`
    pub fn file_clone(&self) -> Option<Result<File>> {
        unsafe {
            self.fd().map(|fd| {
                let newfd = libc::dup(fd);
                if newfd < 0 {
                    Err(Error::Errno(Errno::last()))
                }
                else {
                    Ok(File::from_raw_fd(newfd))
                }
            })
        }
    }

    fn set_fd(&mut self, fd: RawFd, owning: bool) -> Result<()> {
        try_errno!(unsafe {
            raw::libevdev_set_fd(self.raw, fd)
        });
        self.owning = owning;
        Ok(())
    }

    /// Set the file for this struct and initialize internal data.
    ///
    /// This function may only be called once per device. If the device changed and
    /// you need to re-read a device, use `new` method.
    ///
    /// The Device instance takes ownership of the file descriptor,
    /// the file descriptor will be close when the Device instance goes out of scope.
    /// See set_file_borrowing for a borrowing version.
    /// 
    /// Unless otherwise specified, evdev function behavior is undefined until
    /// a successfull call to `set_fd`.
    pub fn set_file_owning<F: IntoRawFd>(&mut self, file: F) -> Result<()> {
        self.set_fd(file.into_raw_fd(), true)
    }

    /// Set the file for this struct and initialize internal data.
    ///
    /// This function may only be called once per device. If the device changed and
    /// you need to re-read a device, use `new` method. If you need to change the file after
    /// closing and re-opening the same device, use `change_fd`.
    /// 
    /// The Device instance does not takes ownership of the file descriptor,
    /// the file descriptor will not be close when the Device instance goes out of scope.
    /// See set_file_owning for a version taking ownership.
    /// 
    /// Unless otherwise specified, evdev function behavior is undefined until
    /// a successfull call to `set_fd`.
    pub fn set_file_borrowing<F: AsRawFd>(&mut self, file: &F) -> Result<()> {
        self.set_fd(file.as_raw_fd(), false)
    }

    /// Change the fd for this device, without re-reading the actual device.
    ///
    /// If the fd changes after initializing the device, for example after a
    /// VT-switch in the X.org X server, this function updates the internal fd
    /// to the newly opened. No check is made that new fd points to the same
    /// device. If the device has changed, evdev's behavior is undefined.
    ///
    /// evdev device does not sync itself after changing the fd and keeps the current
    /// device state. Use next_event with the FORCE_SYNC flag to force a re-sync.
    ///
    /// Note that this method only makes sense if the original file descriptor is
    /// borrowed by the Device instance.
    /// 
    /// # Example
    ///
    /// ```rust,ignore
    /// dev.change_fd(new_fd);
    /// dev.next_event(evdev::FORCE_SYNC);
    /// while dev.next_event(evdev::SYNC).ok().unwrap().0 == ReadStatus::SYNC
    ///                             {} // noop
    /// ```
    /// After changing the fd, the device is assumed ungrabbed and a caller must
    /// call libevdev_grab() again.
    ///
    /// It is an error to call this function before calling set_fd().
    pub fn change_fd<F: AsRawFd>(&mut self, f: &F) -> Result<()>  {
        if self.owning {
            panic!("change_fd is not compatible with owned file")
        }

        try_evdev!(unsafe {
            raw::libevdev_change_fd(self.raw, f.as_raw_fd())
        })
    }

    /// Grab or ungrab the device through a kernel EVIOCGRAB.
    ///
    /// This prevents other clients (including kernel-internal ones such as
    /// rfkill) from receiving events from this device. This is generally a
    /// bad idea. Don't do this.Grabbing an already grabbed device, or
    /// ungrabbing an ungrabbed device is a noop and always succeeds.
    ///
    /// A grab is an operation tied to a file descriptor, not a device. If a
    /// client changes the file descriptor with libevdev_change_fd(), it must
    /// also re-issue a grab with libevdev_grab().
    pub fn grab(&mut self, grab: GrabMode) -> Result<()> {
        try_errno!(unsafe {
            raw::libevdev_grab(self.raw, grab as c_int)
        });
        Ok(())
    }

    /// Get the axis info for the given axis, as advertised by the kernel.
    ///
    /// Returns the `AbsInfo` for the given the code or None if :
    ///   * the code is not a valid EV_ABS code
    ///   * the device doesn't support this code
    pub fn abs_info<C: AsCodeRaw>(&self, code: C) -> Option<AbsInfo> {
        let (r#type, code) = code.as_code_raw();
        if Type::from_raw(r#type) != Type::ABS {
            return None;
        }

        let a = unsafe {
            raw::libevdev_get_abs_info(self.raw, code) 
        };
        if a.is_null() {
            return None
        }

        Some(unsafe {
            AbsInfo {
                value: (*a).value,
                minimum: (*a).minimum,
                maximum: (*a).maximum,
                fuzz: (*a).fuzz,
                flat: (*a).flat,
                resolution: (*a).resolution,
            }
        })
    }

    /// Change the abs info for the given EV_ABS event code, if the code exists.
    ///
    /// This function will succeed with no effect if `has_event_code` returns false for
    /// this code.
    /// 
    /// The code parameter must be a valid EV_ABS code or the function will returns an 
    /// error.
    pub fn set_abs_info<C: AsCodeRaw>(&self, code: C, absinfo: &AbsInfo) -> Result<()> {
        let (r#type, code) = code.as_code_raw();
        if Type::from_raw(r#type) != Type::ABS {
            return Err(Error::NotAnAxis);
        }

        let absinfo = raw::input_absinfo {
            value: absinfo.value,
            minimum: absinfo.minimum,
            maximum: absinfo.maximum,
            fuzz: absinfo.fuzz,
            flat: absinfo.flat,
            resolution: absinfo.resolution,
        };
        unsafe {
            raw::libevdev_set_abs_info(self.raw, code, &absinfo as *const _);
        }

        Ok(())
    }

    /// Returns `true` if device support the property and false otherwise
    pub fn has_property(&self, prop: INPUT_PROP) -> bool {
        unsafe {
            raw::libevdev_has_property(self.raw, prop.as_raw()) != 0
        }
    }

    /// Enables this property, a call to `set_fd` will overwrite any previously set values
    pub fn enable_property(&self, prop: INPUT_PROP) -> Result<()> {
        try_evdev!(unsafe {
            raw::libevdev_enable_property(self.raw, prop.as_raw()) as i32
        })
    }

    /// Returns `true` is the device support this event type and `false` otherwise
    pub fn has_event_type(&self, r#type: Type) -> bool {
        unsafe {
            raw::libevdev_has_event_type(self.raw, r#type.as_raw()) != 0
        }
    }

    /// Return `true` is the device support this event type and code and `false` otherwise
    pub fn has_event_code<C: AsCodeRaw>(&self, code: C) -> bool {
        let (r#type, code) = code.as_code_raw();
        unsafe {
            raw::libevdev_has_event_code(self.raw, r#type, code) != 0
        }
    }

    ///  Returns the current value of the event type.
    ///
    /// If the device supports this event type and code, the return value is
    /// set to the current value of this axis. Otherwise, `None` is returned.
    pub fn event_value<C: AsCodeRaw>(&self, code: C) -> Option<i32> {
        let mut value: i32 = 0;
        let (r#type, code) = code.as_code_raw();
        let valid = unsafe {
            raw::libevdev_fetch_event_value(self.raw, r#type, code, &mut value)
        };

        match valid {
            0 => None,
            _ => Some(value),
        }
    }

    /// Set the value for a given event type and code.
    ///
    /// This only makes sense for some event types, e.g. setting the value for
    /// EV_REL is pointless.
    ///
    /// This is a local modification only affecting only this representation of
    /// this device. A future call to event_value() will return this
    /// value, unless the value was overwritten by an event.
    ///
    /// If the device supports ABS_MT_SLOT, the value set for any ABS_MT_*
    /// event code is the value of the currently active slot. You should use
    /// `set_slot_value` instead.
    ///
    /// If the device supports ABS_MT_SLOT and the type is EV_ABS and the code is
    /// ABS_MT_SLOT, the value must be a positive number less then the number of
    /// slots on the device. Otherwise, `set_event_value` returns Err.
    pub fn set_event_value<C: AsCodeRaw>(&self, code: C, val: i32) -> Result<()> {
        let (r#type, code) = code.as_code_raw();
        try_evdev!(unsafe {
            raw::libevdev_set_event_value(self.raw, r#type, code, val as c_int)
        })
    }

    /// Check if there are events waiting for us.
    ///
    /// This function does not read an event off the fd and may not access the
    /// fd at all. If there are events queued internally this function will
    /// return non-zero. If the internal queue is empty, this function will poll
    /// the file descriptor for data.
    ///
    /// This is a convenience function for simple processes, most complex programs
    /// are expected to use select(2) or poll(2) on the file descriptor. The kernel
    /// guarantees that if data is available, it is a multiple of sizeof(struct
    /// input_event), and thus calling `next_event` when select(2) or
    /// poll(2) return is safe. You do not need `has_event_pending` if
    /// you're using select(2) or poll(2).
    pub fn has_event_pending(&self) -> bool {
        unsafe {
            raw::libevdev_has_event_pending(self.raw) > 0
        }
    }

    product_getter!(product_id, libevdev_get_id_product,
                    vendor_id, libevdev_get_id_vendor,
                    bustype, libevdev_get_id_bustype,
                    version, libevdev_get_id_version);

    product_setter!(set_product_id, libevdev_set_id_product,
                    set_vendor_id, libevdev_set_id_vendor,
                    set_bustype, libevdev_set_id_bustype,
                    set_version, libevdev_set_id_version);

    /// Return the driver version of a device already intialize with `set_fd`
    pub fn driver_version(&self) -> i32 {
        unsafe {
            raw::libevdev_get_driver_version(self.raw) as i32
        }
    }

    abs_getter!(abs_minimum, libevdev_get_abs_minimum,
                abs_maximum, libevdev_get_abs_maximum,
                abs_fuzz, libevdev_get_abs_fuzz,
                abs_flat, libevdev_get_abs_flat,
                abs_resolution, libevdev_get_abs_resolution);

    abs_setter!(set_abs_minimum, libevdev_set_abs_minimum,
                set_abs_maximum, libevdev_set_abs_maximum,
                set_abs_fuzz, libevdev_set_abs_fuzz,
                set_abs_flat, libevdev_set_abs_flat,
                set_abs_resolution, libevdev_set_abs_resolution);

    /// Return the current value of the code for the given slot.
    ///
    /// If the device supports this event code, the return value is
    /// is set to the current value of this axis. Otherwise, or
    /// if the event code is not an ABS_MT_* event code, `None` is returned
    pub fn slot_value<C: AsCodeRaw>(&self, slot: u32, code: C) -> Option<i32> {
        let (_, code) = code.as_code_raw();
        let mut value: i32 = 0;
        let valid = unsafe {
            raw::libevdev_fetch_slot_value(self.raw, slot as c_uint, code, &mut value)
        };

        match valid {
            0 => None,
            _ => Some(value),
        }
    }

    /// Set the value for a given code for the given slot.
    ///
    /// This is a local modification only affecting only this representation of
    /// this device. A future call to `slot_value` will return this value,
    /// unless the value was overwritten by an event.
    ///
    /// This function does not set event values for axes outside the ABS_MT range,
    /// use `set_event_value` instead.
    pub fn set_slot_value<C: AsCodeRaw>(&self, slot: u32, code: C, val: i32) -> Result<()> {
        let (_, code) = code.as_code_raw();
        try_errno!(unsafe {
            raw::libevdev_set_slot_value(self.raw, slot as c_uint, code, val as c_int)
        });

        Ok(())
    }

    /// Get the number of slots supported by this device.
    ///
    /// The number of slots supported, or `None` if the device does not provide
    /// any slots
    ///
    /// A device may provide ABS_MT_SLOT but a total number of 0 slots. Hence
    /// the return value of `None` for "device does not provide slots at all"
    pub fn num_slots(&self) -> Option<i32> {
        let result = unsafe {
            raw::libevdev_get_num_slots(self.raw)
        };

        match result  {
            -1 => None,
            slots => Some(slots),
        }
    }

    /// Get the currently active slot.
    ///
    /// This may differ from the value an ioctl may return at this time as
    /// events may have been read off the fd since changing the slot value
    /// but those events are still in the buffer waiting to be processed.
    /// The returned value is the value a caller would see if it were to
    /// process events manually one-by-one.
    pub fn current_slot(&self) -> Option<i32> {
        let result = unsafe {
            raw::libevdev_get_current_slot(self.raw)
        };

        match result {
            -1 => None,
            slots => Some(slots),
        }
    }

    /// Forcibly enable an event type on this device, even if the underlying
    /// device does not support it. While this cannot make the device actually
    /// report such events, it will now return true for libevdev_has_event_type().
    ///
    /// This is a local modification only affecting only this representation of
    /// this device.
    pub fn enable_event_type(&self, r#type: Type) -> Result<()> {
        try_evdev!(unsafe {
            raw::libevdev_enable_event_type(self.raw, r#type.as_raw())
        })
    }

    pub fn enable_event_code<C: AsCodeRaw>(&self, code: C, data: Option<&Any>) -> Result<()> {
        let (r#type, code) = code.as_code_raw();
        let data = match data {
            Some(data) => data as *const _ as *const c_void,
            None => std::ptr::null(),
        };

        try_evdev!(unsafe {
            raw::libevdev_enable_event_code(self.raw, r#type, code, data)
        })
    }

    /// Forcibly disable an event type on this device, even if the underlying
    /// device provides it. This effectively mutes the respective set of
    /// events. libevdev will filter any events matching this type and none will
    /// reach the caller. libevdev_has_event_type() will return false for this
    /// type.
    ///
    /// In most cases, a caller likely only wants to disable a single code, not
    /// the whole type. Use `disable_event_code` for that.
    ///
    /// Disabling EV_SYN will not work. In Peter's Words "Don't shoot yourself
    /// in the foot. It hurts".
    ///
    /// This is a local modification only affecting only this representation of
    /// this device.
    pub fn disable_event_type(&self, r#type: &Type) -> Result<()> {
        try_evdev!(unsafe {
            raw::libevdev_disable_event_type(self.raw, r#type.as_raw())
        })
    }

    /// Forcibly disable an event code on this device, even if the underlying
    /// device provides it. This effectively mutes the respective set of
    /// events. libevdev will filter any events matching this type and code and
    /// none will reach the caller. `has_event_code` will return false for
    /// this code.
    ///
    /// Disabling all event codes for a given type will not disable the event
    /// type. Use `disable_event_type` for that.
    ///
    /// This is a local modification only affecting only this representation of
    /// this device.
    ///
    /// Disabling codes of type EV_SYN will not work. Don't shoot yourself in the
    /// foot. It hurts.
    pub fn disable_event_code<C: AsCodeRaw>(&self, code: C) -> Result<()> {
        let (r#type, code) = code.as_code_raw();
        try_evdev!(unsafe {
            raw::libevdev_disable_event_code(self.raw, r#type, code)
        })
    }

    /// Set the device's EV_ABS axis to the value defined in the abs
    /// parameter. This will be written to the kernel.
    pub fn kernel_set_abs_info<C: AsCodeRaw>(&self, code: C, absinfo: &AbsInfo) -> Result<()> {
        let (r#type, code) = code.as_code_raw();
        if Type::from_raw(r#type) != Type::ABS {
            return Err(Error::NotAnAxis);
        }

        let absinfo = raw::input_absinfo {
            value: absinfo.value,
            minimum: absinfo.minimum,
            maximum: absinfo.maximum,
            fuzz: absinfo.fuzz,
            flat: absinfo.flat,
            resolution: absinfo.resolution,
        };

        try_errno!(unsafe {
            raw::libevdev_kernel_set_abs_info(self.raw, code, &absinfo as *const _)
        });
        Ok(())
    }

    /// Turn an LED on or off.
    ///
    /// enabling an LED requires write permissions on the device's file descriptor.
    pub fn kernel_set_led_value<C: AsCodeRaw>(&self, code: C, value: LedState) -> Result<()> {
        let (_, code) = code.as_code_raw();
        try_errno!(unsafe {
            raw::libevdev_kernel_set_led_value(self.raw, code, value as c_int)
        });

        Ok(())
    }

    /// Set the clock ID to be used for timestamps. Further events from this device
    /// will report an event time based on the given clock.
    ///
    /// This is a modification only affecting this representation of
    /// this device.
    pub fn set_clock_id(&self, clockid: i32) -> Result<()> {
        try_errno!(unsafe {
            raw::libevdev_set_clock_id(self.raw, clockid as c_int)
        });

        Ok(())
    }

    /// Get the next event from the device. This function operates in two different
    /// modes: normal mode or sync mode.
    ///
    /// In normal mode (when flags has `evdev::NORMAL` set), this function returns
    /// `ReadStatus::Success` and returns the event. If no events are available at
    /// this time, it returns `-EAGAIN` as `Err`.
    ///
    /// If the current event is an `EV_SYN::SYN_DROPPED` event, this function returns
    /// `ReadStatus::Sync` and is set to the `EV_SYN` event.The caller should now call
    /// this function with the `evdev::SYNC` flag set, to get the set of events that
    /// make up the device state delta. This function returns ReadStatus::Sync for
    /// each event part of that delta, until it returns `-EAGAIN` once all events
    /// have been synced.
    ///
    /// If a device needs to be synced by the caller but the caller does not call
    /// with the `evdev::SYNC` flag set, all events from the diff are dropped after
    /// evdev updates its internal state and event processing continues as normal.
    /// Note that the current slot and the state of touch points may have updated
    /// during the `SYN_DROPPED` event, it is strongly recommended that a caller
    /// ignoring all sync events calls `current_slot` and checks the
    /// `ABS_MT_TRACKING_ID` values for all slots.
    ///
    /// If a device has changed state without events being enqueued in evdev,
    /// e.g. after changing the file descriptor, use the `evdev::FORCE_SYNC` flag.
    /// This triggers an internal sync of the device and `next_event` returns
    /// `ReadStatus::Sync`.
    pub fn next_event(&self, flags: ReadFlag) -> Result<(ReadStatus, Event)> {
        let mut raw = raw::input_event {
            time: nix::libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            event_type: 0,
            event_code: 0,
            value: 0,
        };

        let result = unsafe {
            raw::libevdev_next_event(self.raw, flags.bits as c_uint, &mut raw)
        };

        let event = Event::from_raw(raw);

        match result {
            raw::LIBEVDEV_READ_STATUS_SUCCESS => Ok((ReadStatus::Success, event)),
            raw::LIBEVDEV_READ_STATUS_SYNC => Ok((ReadStatus::Sync, event)),
            error => Err(Error::Errno(Errno::from_i32(-error))),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            raw::libevdev_free(self.raw);
            if self.owning {
                libc::close(raw::libevdev_get_fd(self.raw));
            }
        }
    }
}
