// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use BufferPool;
use Structure;

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlib, ToGlibPtr, ToGlibPtrMut};
use glib::IsA;

use ffi;

use std::mem;
use std::ops;
use std::ptr;

#[derive(Debug, PartialEq, Eq)]
pub struct BufferPoolConfig(Structure);

impl ops::Deref for BufferPoolConfig {
    type Target = ::StructureRef;

    fn deref(&self) -> &::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for BufferPoolConfig {
    fn deref_mut(&mut self) -> &mut ::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<::StructureRef> for BufferPoolConfig {
    fn as_ref(&self) -> &::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<::StructureRef> for BufferPoolConfig {
    fn as_mut(&mut self) -> &mut ::StructureRef {
        self.0.as_mut()
    }
}

impl BufferPoolConfig {
    pub fn add_option(&mut self, option: &str) {
        unsafe {
            ffi::gst_buffer_pool_config_add_option(
                self.0.to_glib_none_mut().0,
                option.to_glib_none().0,
            );
        }
    }

    pub fn has_option(&self, option: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_config_has_option(
                self.0.to_glib_none().0,
                option.to_glib_none().0,
            ))
        }
    }

    pub fn get_options(&self) -> Vec<String> {
        unsafe {
            let n = ffi::gst_buffer_pool_config_n_options(self.0.to_glib_none().0) as usize;
            let mut options = Vec::with_capacity(n);

            for i in 0..n {
                options.push(from_glib_none(ffi::gst_buffer_pool_config_get_option(
                    self.0.to_glib_none().0,
                    i as u32,
                )));
            }

            options
        }
    }

    pub fn set_params<'a, T: Into<Option<&'a ::Caps>>>(
        &mut self,
        caps: T,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
        let caps = caps.into();

        unsafe {
            ffi::gst_buffer_pool_config_set_params(
                self.0.to_glib_none_mut().0,
                caps.to_glib_none().0,
                size,
                min_buffers,
                max_buffers,
            );
        }
    }

    pub fn get_params(&self) -> Option<(Option<::Caps>, u32, u32, u32)> {
        unsafe {
            let mut caps = ptr::null_mut();
            let mut size = mem::uninitialized();
            let mut min_buffers = mem::uninitialized();
            let mut max_buffers = mem::uninitialized();

            let ret: bool = from_glib(ffi::gst_buffer_pool_config_get_params(
                self.0.to_glib_none().0,
                &mut caps,
                &mut size,
                &mut min_buffers,
                &mut max_buffers,
            ));
            if !ret {
                return None;
            }

            Some((from_glib_none(caps), size, min_buffers, max_buffers))
        }
    }

    pub fn validate_params<'a, T: Into<Option<&'a ::Caps>>>(
        &self,
        caps: T,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) -> bool {
        let caps = caps.into();

        unsafe {
            from_glib(ffi::gst_buffer_pool_config_validate_params(
                self.0.to_glib_none().0,
                caps.to_glib_none().0,
                size,
                min_buffers,
                max_buffers,
            ))
        }
    }

    // TODO: get_allocator / set_allocator
    // TODO: options iterator
}

#[derive(Debug)]
pub struct BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams);

impl BufferPoolAcquireParams {
    pub fn with_flags(flags: ::BufferPoolAcquireFlags) -> Self {
        BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams {
            format: ffi::GST_FORMAT_UNDEFINED,
            start: -1,
            stop: -1,
            flags: flags.to_glib(),
            _gst_reserved: [ptr::null_mut(); 4],
        })
    }

    pub fn with_start_stop<T: ::SpecificFormattedValue>(
        start: T,
        stop: T,
        flags: ::BufferPoolAcquireFlags,
    ) -> Self {
        unsafe {
            BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams {
                format: start.get_format().to_glib(),
                start: start.to_raw_value(),
                stop: stop.to_raw_value(),
                flags: flags.to_glib(),
                _gst_reserved: [ptr::null_mut(); 4],
            })
        }
    }

    pub fn flags(&self) -> ::BufferPoolAcquireFlags {
        from_glib(self.0.flags)
    }

    pub fn format(&self) -> ::Format {
        from_glib(self.0.format)
    }

    pub fn start(&self) -> ::GenericFormattedValue {
        ::GenericFormattedValue::new(from_glib(self.0.format), self.0.start)
    }

    pub fn stop(&self) -> ::GenericFormattedValue {
        ::GenericFormattedValue::new(from_glib(self.0.format), self.0.stop)
    }
}

impl PartialEq for BufferPoolAcquireParams {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
            && self.start() == other.start()
            && self.stop() == other.stop()
    }
}

impl Eq for BufferPoolAcquireParams {}

impl BufferPool {
    pub fn new() -> BufferPool {
        assert_initialized_main_thread!();
        let (major, minor, _, _) = ::version();
        if (major, minor) > (1, 12) {
            unsafe { from_glib_full(ffi::gst_buffer_pool_new()) }
        } else {
            // Work-around for 1.14 switching from transfer-floating to transfer-full
            unsafe { from_glib_none(ffi::gst_buffer_pool_new()) }
        }
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new()
    }
}

pub trait BufferPoolExtManual: 'static {
    fn get_config(&self) -> BufferPoolConfig;
    fn set_config(&self, config: BufferPoolConfig) -> Result<(), glib::error::BoolError>;

    fn is_flushing(&self) -> bool;

    fn acquire_buffer<'a, P: Into<Option<&'a BufferPoolAcquireParams>>>(
        &self,
        params: P,
    ) -> Result<::Buffer, ::FlowError>;
    fn release_buffer(&self, buffer: ::Buffer);
}

impl<O: IsA<BufferPool>> BufferPoolExtManual for O {
    fn get_config(&self) -> BufferPoolConfig {
        unsafe {
            let ptr = ffi::gst_buffer_pool_get_config(self.to_glib_none().0);
            BufferPoolConfig(from_glib_full(ptr))
        }
    }

    fn set_config(&self, config: BufferPoolConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(
                ffi::gst_buffer_pool_set_config(self.to_glib_none().0, config.0.into_ptr()),
                "Failed to set config",
            )
        }
    }

    fn is_flushing(&self) -> bool {
        unsafe {
            let stash = self.to_glib_none();
            let ptr: *mut ffi::GstBufferPool = stash.0;

            from_glib((*ptr).flushing)
        }
    }

    fn acquire_buffer<'a, P: Into<Option<&'a BufferPoolAcquireParams>>>(
        &self,
        params: P,
    ) -> Result<::Buffer, ::FlowError> {
        let params = params.into();
        let params_ptr = match params {
            Some(params) => &params.0 as *const _ as *mut _,
            None => ptr::null_mut(),
        };

        unsafe {
            let mut buffer = ptr::null_mut();
            let ret: ::FlowReturn = from_glib(ffi::gst_buffer_pool_acquire_buffer(
                self.to_glib_none().0,
                &mut buffer,
                params_ptr,
            ));

            ret.into_result_value(|| from_glib_full(buffer))
        }
    }

    fn release_buffer(&self, buffer: ::Buffer) {
        unsafe {
            ffi::gst_buffer_pool_release_buffer(self.to_glib_none().0, buffer.into_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_pool() {
        ::init().unwrap();

        let pool = ::BufferPool::new();
        let mut config = pool.get_config();
        config.set_params(Some(&::Caps::new_simple("foo/bar", &[])), 1024, 0, 2);
        pool.set_config(config).unwrap();

        pool.set_active(true).unwrap();

        let params = ::BufferPoolAcquireParams::with_flags(::BufferPoolAcquireFlags::DONTWAIT);

        let _buf1 = pool.acquire_buffer(&params).unwrap();
        let buf2 = pool.acquire_buffer(&params).unwrap();

        assert!(pool.acquire_buffer(&params).is_err());

        drop(buf2);
        let _buf2 = pool.acquire_buffer(&params).unwrap();

        pool.set_active(false).unwrap();
    }
}
