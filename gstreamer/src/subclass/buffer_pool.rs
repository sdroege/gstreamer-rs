// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::{
    subclass::{prelude::*, InitializingObject},
    translate::*,
    Cast, StaticType,
};
use libc::c_char;

use crate::{BufferPool, BufferPoolAcquireParams, BufferPoolConfigRef};

pub trait BufferPoolImpl: BufferPoolImplExt + GstObjectImpl + Send + Sync {
    fn acquire_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError> {
        self.parent_acquire_buffer(params)
    }

    fn alloc_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError> {
        self.parent_alloc_buffer(params)
    }

    fn flush_start(&self) {
        self.parent_flush_start()
    }

    fn flush_stop(&self) {
        self.parent_flush_stop()
    }

    fn free_buffer(&self, buffer: crate::Buffer) {
        self.parent_free_buffer(buffer)
    }

    fn release_buffer(&self, buffer: crate::Buffer) {
        self.parent_release_buffer(buffer)
    }

    fn reset_buffer(&self, buffer: &mut crate::BufferRef) {
        self.parent_reset_buffer(buffer)
    }

    fn start(&self) -> bool {
        self.parent_start()
    }

    fn stop(&self) -> bool {
        self.parent_stop()
    }

    fn options() -> &'static [&'static str] {
        &[]
    }

    fn set_config(&self, config: &mut BufferPoolConfigRef) -> bool {
        self.parent_set_config(config)
    }
}

pub trait BufferPoolImplExt: ObjectSubclass {
    fn parent_acquire_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError>;

    fn parent_alloc_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError>;

    fn parent_free_buffer(&self, buffer: crate::Buffer);

    fn parent_release_buffer(&self, buffer: crate::Buffer);

    fn parent_reset_buffer(&self, buffer: &mut crate::BufferRef);

    fn parent_start(&self) -> bool;

    fn parent_stop(&self) -> bool;

    fn parent_set_config(&self, config: &mut BufferPoolConfigRef) -> bool;

    fn parent_flush_start(&self);

    fn parent_flush_stop(&self);
}

impl<T: BufferPoolImpl> BufferPoolImplExt for T {
    fn parent_acquire_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).acquire_buffer {
                let params_ptr = mut_override(params.to_glib_none().0);
                let mut buffer = std::ptr::null_mut();

                let result = f(
                    self.obj()
                        .unsafe_cast_ref::<crate::BufferPool>()
                        .to_glib_none()
                        .0,
                    &mut buffer,
                    params_ptr,
                );

                crate::FlowSuccess::try_from_glib(result).map(|_| from_glib_full(buffer))
            } else {
                Err(crate::FlowError::NotSupported)
            }
        }
    }

    fn parent_alloc_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).alloc_buffer {
                let params_ptr = mut_override(params.to_glib_none().0);
                let mut buffer = std::ptr::null_mut();

                let result = f(
                    self.obj()
                        .unsafe_cast_ref::<crate::BufferPool>()
                        .to_glib_none()
                        .0,
                    &mut buffer,
                    params_ptr,
                );

                crate::FlowSuccess::try_from_glib(result).map(|_| from_glib_full(buffer))
            } else {
                Err(crate::FlowError::NotSupported)
            }
        }
    }

    fn parent_free_buffer(&self, buffer: crate::Buffer) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).free_buffer {
                f(
                    self.obj()
                        .unsafe_cast_ref::<crate::BufferPool>()
                        .to_glib_none()
                        .0,
                    buffer.into_glib_ptr(),
                )
            }
        }
    }

    fn parent_release_buffer(&self, buffer: crate::Buffer) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).release_buffer {
                f(
                    self.obj()
                        .unsafe_cast_ref::<crate::BufferPool>()
                        .to_glib_none()
                        .0,
                    buffer.into_glib_ptr(),
                )
            }
        }
    }

    fn parent_reset_buffer(&self, buffer: &mut crate::BufferRef) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).reset_buffer {
                f(
                    self.obj()
                        .unsafe_cast_ref::<crate::BufferPool>()
                        .to_glib_none()
                        .0,
                    buffer.as_mut_ptr(),
                )
            }
        }
    }

    fn parent_start(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).start {
                let result = f(self
                    .obj()
                    .unsafe_cast_ref::<crate::BufferPool>()
                    .to_glib_none()
                    .0);

                from_glib(result)
            } else {
                true
            }
        }
    }

    fn parent_stop(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).stop {
                let result = f(self
                    .obj()
                    .unsafe_cast_ref::<crate::BufferPool>()
                    .to_glib_none()
                    .0);

                from_glib(result)
            } else {
                true
            }
        }
    }

    fn parent_set_config(&self, config: &mut BufferPoolConfigRef) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).set_config {
                let result = f(
                    self.obj()
                        .unsafe_cast_ref::<crate::BufferPool>()
                        .to_glib_none()
                        .0,
                    (*config).as_mut_ptr(),
                );

                from_glib(result)
            } else {
                false
            }
        }
    }

    fn parent_flush_start(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).flush_start {
                f(self
                    .obj()
                    .unsafe_cast_ref::<crate::BufferPool>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    fn parent_flush_stop(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBufferPoolClass;
            if let Some(f) = (*parent_class).flush_stop {
                f(self
                    .obj()
                    .unsafe_cast_ref::<crate::BufferPool>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

// Send+Sync wrapper around a NULL-terminated C string array
struct CStrV(*mut *const libc::c_char);
unsafe impl Send for CStrV {}
unsafe impl Sync for CStrV {}

impl Drop for CStrV {
    fn drop(&mut self) {
        unsafe { glib::ffi::g_strfreev(self.0 as *mut _) };
    }
}

unsafe impl<T: BufferPoolImpl> IsSubclassable<T> for BufferPool {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.acquire_buffer = Some(buffer_pool_acquire_buffer::<T>);
        klass.alloc_buffer = Some(buffer_pool_alloc_buffer::<T>);
        klass.release_buffer = Some(buffer_pool_release_buffer::<T>);
        klass.reset_buffer = Some(buffer_pool_reset_buffer::<T>);
        klass.start = Some(buffer_pool_start::<T>);
        klass.stop = Some(buffer_pool_stop::<T>);
        klass.get_options = Some(buffer_pool_get_options::<T>);
        klass.set_config = Some(buffer_pool_set_config::<T>);
        klass.flush_start = Some(buffer_pool_flush_start::<T>);
        klass.flush_stop = Some(buffer_pool_flush_stop::<T>);
        klass.free_buffer = Some(buffer_pool_free_buffer::<T>);
    }

    fn instance_init(instance: &mut InitializingObject<T>) {
        Self::parent_instance_init(instance);

        // Store the pool options in the instance data
        // for later retrieval in buffer_pool_get_options
        let options = T::options();
        let options = options.to_glib_full();
        instance.set_instance_data(T::type_(), CStrV(options));
    }
}

unsafe extern "C" fn buffer_pool_acquire_buffer<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
    buffer: *mut *mut ffi::GstBuffer,
    params: *mut ffi::GstBufferPoolAcquireParams,
) -> ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let params: Option<BufferPoolAcquireParams> = from_glib_none(params);

    match imp.acquire_buffer(params.as_ref()) {
        Ok(b) => {
            *buffer = b.into_glib_ptr();
            ffi::GST_FLOW_OK
        }
        Err(err) => err.into_glib(),
    }
}

unsafe extern "C" fn buffer_pool_alloc_buffer<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
    buffer: *mut *mut ffi::GstBuffer,
    params: *mut ffi::GstBufferPoolAcquireParams,
) -> ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let params: Option<BufferPoolAcquireParams> = from_glib_none(params);

    match imp.alloc_buffer(params.as_ref()) {
        Ok(b) => {
            *buffer = b.into_glib_ptr();
            ffi::GST_FLOW_OK
        }
        Err(err) => err.into_glib(),
    }
}

unsafe extern "C" fn buffer_pool_flush_start<T: BufferPoolImpl>(ptr: *mut ffi::GstBufferPool) {
    // the GstBufferPool implementation calls this
    // in finalize where the ref_count will already
    // be zero and we are actually destroyed
    // see: https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/1645
    if (*(ptr as *const glib::gobject_ffi::GObject)).ref_count == 0 {
        // flush_start is a no-op in GstBufferPool
        return;
    }

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.flush_start();
}

unsafe extern "C" fn buffer_pool_flush_stop<T: BufferPoolImpl>(ptr: *mut ffi::GstBufferPool) {
    // the GstBufferPool implementation calls this
    // in finalize where the ref_count will already
    // be zero and we are actually destroyed
    // see: https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/1645
    if (*(ptr as *const glib::gobject_ffi::GObject)).ref_count == 0 {
        // flush_stop is a no-op in GstBufferPool
        return;
    }

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.flush_stop();
}

unsafe extern "C" fn buffer_pool_free_buffer<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
    buffer: *mut ffi::GstBuffer,
) {
    // the GstBufferPool implementation calls this
    // in finalize where the ref_count will already
    // be zero and we are actually destroyed
    // see: https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/1645
    if (*(ptr as *const glib::gobject_ffi::GObject)).ref_count == 0 {
        // As a workaround we call free_buffer directly on the
        // GstBufferPool to prevent leaking the buffer
        // This will NOT call free_buffer on a subclass.
        let pool_class =
            glib::Class::<crate::BufferPool>::from_type(crate::BufferPool::static_type()).unwrap();
        let pool_class = pool_class.as_ref();
        if let Some(f) = pool_class.free_buffer {
            f(ptr, buffer)
        }
        return;
    }

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.free_buffer(from_glib_full(buffer));
}

unsafe extern "C" fn buffer_pool_release_buffer<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
    buffer: *mut ffi::GstBuffer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.release_buffer(from_glib_full(buffer));
}

unsafe extern "C" fn buffer_pool_reset_buffer<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
    buffer: *mut ffi::GstBuffer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.reset_buffer(crate::BufferRef::from_mut_ptr(buffer));
}

unsafe extern "C" fn buffer_pool_start<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.start().into_glib()
}

unsafe extern "C" fn buffer_pool_stop<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
) -> glib::ffi::gboolean {
    // the GstBufferPool implementation calls this
    // in finalize where the ref_count will already
    // be zero and we are actually destroyed
    // see: https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/1645
    if (*(ptr as *const glib::gobject_ffi::GObject)).ref_count == 0 {
        // As a workaround we call stop directly on the GstBufferPool
        // This is needed because the default implementation clears
        // the pool in stop.
        let pool_class =
            glib::Class::<crate::BufferPool>::from_type(crate::BufferPool::static_type()).unwrap();
        let pool_class = pool_class.as_ref();
        let result = if let Some(f) = pool_class.stop {
            f(ptr)
        } else {
            true.into_glib()
        };

        return result;
    }

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.stop().into_glib()
}

unsafe extern "C" fn buffer_pool_get_options<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
) -> *mut *const c_char {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    T::instance_data::<CStrV>(imp, T::type_())
        .unwrap_or(&CStrV(std::ptr::null_mut()))
        .0
}

unsafe extern "C" fn buffer_pool_set_config<T: BufferPoolImpl>(
    ptr: *mut ffi::GstBufferPool,
    config: *mut ffi::GstStructure,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    imp.set_config(BufferPoolConfigRef::from_glib_borrow_mut(config))
        .into_glib()
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    use super::*;
    use crate::prelude::*;

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct TestBufferPool;

        impl ObjectImpl for TestBufferPool {}
        impl GstObjectImpl for TestBufferPool {}
        impl BufferPoolImpl for TestBufferPool {
            fn options() -> &'static [&'static str] {
                &["TEST_OPTION"]
            }

            fn set_config(&self, config: &mut BufferPoolConfigRef) -> bool {
                let (caps, size, min_buffers, max_buffers) = config.params().unwrap();
                config.set_params(caps.as_ref(), size * 2, min_buffers, max_buffers);
                self.parent_set_config(config)
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestBufferPool {
            const NAME: &'static str = "TestBufferPool";
            type Type = super::TestBufferPool;
            type ParentType = BufferPool;
        }
    }

    glib::wrapper! {
        pub struct TestBufferPool(ObjectSubclass<imp::TestBufferPool>) @extends BufferPool, crate::Object;
    }

    impl Default for TestBufferPool {
        fn default() -> Self {
            glib::Object::new(&[])
        }
    }

    #[test]
    fn test_pool_options() {
        crate::init().unwrap();
        let pool = TestBufferPool::default();
        assert_eq!(pool.options(), vec!["TEST_OPTION"]);
    }

    #[test]
    fn test_pool_acquire() {
        crate::init().unwrap();
        let pool = TestBufferPool::default();
        let mut config = pool.config();
        config.set_params(None, 1024, 1, 1);
        pool.set_config(config).expect("failed to set pool config");
        pool.set_active(true).expect("failed to activate pool");
        let buffer = pool
            .acquire_buffer(None)
            .expect("failed to acquire buffer from pool");
        assert_eq!(buffer.size(), 2048);
    }

    #[test]
    fn test_pool_free_on_finalize() {
        crate::init().unwrap();
        let pool = TestBufferPool::default();
        let mut config = pool.config();
        config.set_params(None, 1024, 1, 1);
        pool.set_config(config).expect("failed to set pool config");
        pool.set_active(true).expect("failed to activate pool");
        let mut buffer = pool
            .acquire_buffer(None)
            .expect("failed to acquire buffer from pool");
        let finalized = Arc::new(AtomicBool::new(false));
        unsafe {
            ffi::gst_mini_object_weak_ref(
                buffer.make_mut().upcast_mut().as_mut_ptr(),
                Some(buffer_finalized),
                Arc::into_raw(finalized.clone()) as *mut _,
            )
        };
        // return the buffer to the pool
        std::mem::drop(buffer);
        // drop should finalize the buffer pool which frees all allocated buffers
        std::mem::drop(pool);
        assert!(finalized.load(Ordering::SeqCst));
    }

    #[test]
    fn test_pool_free_on_deactivate() {
        crate::init().unwrap();
        let pool = TestBufferPool::default();
        let mut config = pool.config();
        config.set_params(None, 1024, 1, 1);
        pool.set_config(config).expect("failed to set pool config");
        pool.set_active(true).expect("failed to activate pool");
        let mut buffer = pool
            .acquire_buffer(None)
            .expect("failed to acquire buffer from pool");
        let finalized = Arc::new(AtomicBool::new(false));
        unsafe {
            ffi::gst_mini_object_weak_ref(
                buffer.make_mut().upcast_mut().as_mut_ptr(),
                Some(buffer_finalized),
                Arc::into_raw(finalized.clone()) as *mut _,
            )
        };
        // return the buffer to the pool
        std::mem::drop(buffer);
        // de-activating a poll should free all buffers
        pool.set_active(false).expect("failed to de-activate pool");
        assert!(finalized.load(Ordering::SeqCst));
    }

    unsafe extern "C" fn buffer_finalized(
        data: *mut libc::c_void,
        _mini_object: *mut ffi::GstMiniObject,
    ) {
        let finalized = Arc::from_raw(data as *const AtomicBool);
        finalized.store(true, Ordering::SeqCst);
    }
}
