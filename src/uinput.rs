use nix::libc;
use nix::libc::c_int;
use std::fs::File;
use std::os::unix::io::{RawFd, AsRawFd, FromRawFd, IntoRawFd};

use raw;
use event::*;
use util::*;
use super::{Device as EvDevice, Error, Result};

pub struct Device {
    raw: *mut raw::libevdev_uinput,
    owning: bool,
}

impl Device {
    fn do_create_from_device(device: &EvDevice, uinput_fd: RawFd, owning: bool) -> Result<Device> {
        let mut libevdev_uinput = 0 as *mut _;
        try_errno!(unsafe {
            raw::libevdev_uinput_create_from_device(
                device.raw,
                uinput_fd,
                &mut libevdev_uinput)
        });

        Ok(Device { raw: libevdev_uinput, owning })
    }

    pub fn create_from_device(device: &EvDevice) -> Result<Device> {
        Self::do_create_from_device(device, raw::LIBEVDEV_UINPUT_OPEN_MANAGED, false)
    }

    pub fn create_from_device_owning<F: IntoRawFd>(device: &EvDevice, uinput_file: F) -> Result<Device> {
        Self::do_create_from_device(device, uinput_file.into_raw_fd(), true)
    }

    pub fn create_from_device_borrowing<F: AsRawFd>(device: &EvDevice, uinput_file: &F) -> Result<Device> {
        Self::do_create_from_device(device, uinput_file.as_raw_fd(), false)
    }

    /// Returns the file descriptor associated with the uinput device
    pub unsafe fn fd(&self) -> RawFd {
        raw::libevdev_uinput_get_fd(self.raw)
    }

    /// Consumes this uinput device instance and returns the associated file
    pub fn into_file(self) -> File {
        unsafe {
            File::from_raw_fd(self.fd())
        }
    }

    pub fn file_clone(&self) -> Result<File> {
        unsafe {
            let fd = raw::libevdev_uinput_get_fd(self.raw);
            let newfd = libc::dup(fd);
            if newfd < 0 {
                Error::errno_from_i32(nix::errno::errno())
            }
            else {
                Ok(File::from_raw_fd(newfd))
            }
        }
    }

    string_getter!(syspath, libevdev_uinput_get_syspath,
                   devnode, libevdev_uinput_get_devnode);

    pub fn write_event<C: AsCodeRaw>(&self, code: C, value: i32) -> Result<()> {
        let (ev_type, ev_code) = code.as_code_raw();
        try_errno!(unsafe {
            raw::libevdev_uinput_write_event(self.raw, ev_type, ev_code, value as c_int)
        });

        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { raw::libevdev_uinput_destroy(self.raw) };
        if self.owning {
            unsafe { libc::close(raw::libevdev_uinput_get_fd(self.raw)) };
        }
    }
}
