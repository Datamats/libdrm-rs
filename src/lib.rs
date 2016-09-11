#[macro_use]
extern crate ioctl;
extern crate core;
extern crate libc;
extern crate byteorder;

#[allow(dead_code)]
pub mod ffi;

#[allow(dead_code)]
pub mod drm;

#[allow(dead_code)]
pub mod drm_mode;

#[allow(dead_code)]
mod crtc;

#[allow(dead_code)]
mod encoder;

#[allow(dead_code)]
mod connector;

#[allow(dead_code)]
mod resources;
