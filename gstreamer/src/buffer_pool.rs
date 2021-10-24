// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AllocationParams;
use crate::Allocator;
use crate::BufferPool;
use crate::Structure;

use glib::prelude::*;
use glib::translate::*;

use std::mem;
use std::ops;
use std::ptr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferPoolConfig(Structure);

impl ops::Deref for BufferPoolConfig {
    type Target = crate::StructureRef;

    fn deref(&self) -> &crate::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for BufferPoolConfig {
    fn deref_mut(&mut self) -> &mut crate::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<crate::StructureRef> for BufferPoolConfig {
    fn as_ref(&self) -> &crate::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<crate::StructureRef> for BufferPoolConfig {
    fn as_mut(&mut self) -> &mut crate::StructureRef {
        self.0.as_mut()
    }
}

impl BufferPoolConfig {
    #[doc(alias = "gst_buffer_pool_config_add_option")]
    pub fn add_option(&mut self, option: &str) {
        unsafe {
            ffi::gst_buffer_pool_config_add_option(
                self.0.to_glib_none_mut().0,
                option.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_buffer_pool_config_has_option")]
    pub fn has_option(&self, option: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_config_has_option(
                self.0.to_glib_none().0,
                option.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_options")]
    #[doc(alias = "gst_buffer_pool_config_n_options")]
    pub fn options(&self) -> Vec<String> {
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

    #[doc(alias = "gst_buffer_pool_config_set_params")]
    pub fn set_params(
        &mut self,
        caps: Option<&crate::Caps>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
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

    #[doc(alias = "get_params")]
    #[doc(alias = "gst_buffer_pool_config_get_params")]
    pub fn params(&self) -> Option<(Option<crate::Caps>, u32, u32, u32)> {
        unsafe {
            let mut caps = ptr::null_mut();
            let mut size = mem::MaybeUninit::uninit();
            let mut min_buffers = mem::MaybeUninit::uninit();
            let mut max_buffers = mem::MaybeUninit::uninit();

            let ret: bool = from_glib(ffi::gst_buffer_pool_config_get_params(
                self.0.to_glib_none().0,
                &mut caps,
                size.as_mut_ptr(),
                min_buffers.as_mut_ptr(),
                max_buffers.as_mut_ptr(),
            ));
            if !ret {
                return None;
            }

            Some((
                from_glib_none(caps),
                size.assume_init(),
                min_buffers.assume_init(),
                max_buffers.assume_init(),
            ))
        }
    }

    #[doc(alias = "gst_buffer_pool_config_validate_params")]
    pub fn validate_params(
        &self,
        caps: Option<&crate::Caps>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_pool_config_validate_params(
                    self.0.to_glib_none().0,
                    caps.to_glib_none().0,
                    size,
                    min_buffers,
                    max_buffers,
                ),
                "Parameters are not valid in this context"
            )
        }
    }

    #[doc(alias = "get_allocator")]
    #[doc(alias = "gst_buffer_pool_config_get_allocator")]
    pub fn allocator(&self) -> Option<(Option<Allocator>, AllocationParams)> {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::MaybeUninit::zeroed();
            let ret = from_glib(ffi::gst_buffer_pool_config_get_allocator(
                self.0.to_glib_none().0,
                &mut allocator,
                params.as_mut_ptr(),
            ));
            if ret {
                Some((from_glib_none(allocator), params.assume_init().into()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_buffer_pool_config_set_allocator")]
    pub fn set_allocator(&self, allocator: Option<&Allocator>, params: Option<&AllocationParams>) {
        assert!(allocator.is_some() || params.is_some());
        unsafe {
            ffi::gst_buffer_pool_config_set_allocator(
                self.0.to_glib_none().0,
                allocator.to_glib_none().0,
                match params {
                    Some(val) => val.as_ptr(),
                    None => ptr::null(),
                },
            )
        }
    }
    // TODO: options iterator
}

#[derive(Debug)]
#[doc(alias = "GstBufferPoolAcquireParams")]
pub struct BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams);

unsafe impl Send for BufferPoolAcquireParams {}
unsafe impl Sync for BufferPoolAcquireParams {}

impl BufferPoolAcquireParams {
    pub fn with_flags(flags: crate::BufferPoolAcquireFlags) -> Self {
        skip_assert_initialized!();
        BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams {
            format: ffi::GST_FORMAT_UNDEFINED,
            start: -1,
            stop: -1,
            flags: flags.into_glib(),
            _gst_reserved: [ptr::null_mut(); 4],
        })
    }

    pub fn with_start_stop<T: crate::SpecificFormattedValue>(
        start: T,
        stop: T,
        flags: crate::BufferPoolAcquireFlags,
    ) -> Self {
        skip_assert_initialized!();
        unsafe {
            BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams {
                format: start.format().into_glib(),
                start: start.into_raw_value(),
                stop: stop.into_raw_value(),
                flags: flags.into_glib(),
                _gst_reserved: [ptr::null_mut(); 4],
            })
        }
    }

    pub fn flags(&self) -> crate::BufferPoolAcquireFlags {
        unsafe { from_glib(self.0.flags) }
    }

    pub fn format(&self) -> crate::Format {
        unsafe { from_glib(self.0.format) }
    }

    pub fn start(&self) -> crate::GenericFormattedValue {
        unsafe { crate::GenericFormattedValue::new(from_glib(self.0.format), self.0.start) }
    }

    pub fn stop(&self) -> crate::GenericFormattedValue {
        unsafe { crate::GenericFormattedValue::new(from_glib(self.0.format), self.0.stop) }
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

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstBufferPoolAcquireParams> for BufferPoolAcquireParams {
    type Storage = &'a Self;

    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *const ffi::GstBufferPoolAcquireParams, Self> {
        glib::translate::Stash(&self.0, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GstBufferPoolAcquireParams> for BufferPoolAcquireParams {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(
        &'a mut self,
    ) -> glib::translate::StashMut<'a, *mut ffi::GstBufferPoolAcquireParams, Self> {
        glib::translate::StashMut(&mut self.0, self)
    }
}

impl BufferPool {
    #[doc(alias = "gst_buffer_pool_new")]
    pub fn new() -> BufferPool {
        assert_initialized_main_thread!();
        let (major, minor, _, _) = crate::version();
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
    #[doc(alias = "get_config")]
    #[doc(alias = "gst_buffer_pool_get_config")]
    fn config(&self) -> BufferPoolConfig;
    #[doc(alias = "gst_buffer_pool_set_config")]
    fn set_config(&self, config: BufferPoolConfig) -> Result<(), glib::error::BoolError>;

    fn is_flushing(&self) -> bool;

    #[doc(alias = "gst_buffer_pool_acquire_buffer")]
    fn acquire_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError>;
    #[doc(alias = "gst_buffer_pool_release_buffer")]
    fn release_buffer(&self, buffer: crate::Buffer);
}

impl<O: IsA<BufferPool>> BufferPoolExtManual for O {
    fn config(&self) -> BufferPoolConfig {
        unsafe {
            let ptr = ffi::gst_buffer_pool_get_config(self.as_ref().to_glib_none().0);
            BufferPoolConfig(from_glib_full(ptr))
        }
    }

    fn set_config(&self, config: BufferPoolConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_pool_set_config(
                    self.as_ref().to_glib_none().0,
                    config.0.into_ptr()
                ),
                "Failed to set config",
            )
        }
    }

    fn is_flushing(&self) -> bool {
        unsafe {
            let stash = self.as_ref().to_glib_none();
            let ptr: *mut ffi::GstBufferPool = stash.0;

            from_glib((*ptr).flushing)
        }
    }

    fn acquire_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError> {
        let params_ptr = params.to_glib_none().0 as *mut _;

        unsafe {
            let mut buffer = ptr::null_mut();
            crate::FlowSuccess::try_from_glib(ffi::gst_buffer_pool_acquire_buffer(
                self.as_ref().to_glib_none().0,
                &mut buffer,
                params_ptr,
            ))
            .map(|_| from_glib_full(buffer))
        }
    }

    fn release_buffer(&self, buffer: crate::Buffer) {
        unsafe {
            ffi::gst_buffer_pool_release_buffer(self.as_ref().to_glib_none().0, buffer.into_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_pool() {
        crate::init().unwrap();

        let pool = crate::BufferPool::new();
        let mut config = pool.config();
        config.set_params(Some(&crate::Caps::builder("foo/bar").build()), 1024, 0, 2);
        pool.set_config(config).unwrap();

        pool.set_active(true).unwrap();

        let params =
            crate::BufferPoolAcquireParams::with_flags(crate::BufferPoolAcquireFlags::DONTWAIT);

        let _buf1 = pool.acquire_buffer(Some(&params)).unwrap();
        let buf2 = pool.acquire_buffer(Some(&params)).unwrap();

        assert!(pool.acquire_buffer(Some(&params)).is_err());

        drop(buf2);
        let _buf2 = pool.acquire_buffer(Some(&params)).unwrap();

        pool.set_active(false).unwrap();
    }
}
