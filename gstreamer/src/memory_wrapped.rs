// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{translate::*, StaticType};

use std::{alloc, mem, ptr};

use crate::Memory;

#[repr(C)]
struct WrappedMemory<T> {
    mem: ffi::GstMemory,

    // AsRef / AsMut values
    data: *mut u8,

    // Layout used for allocating this struct, literally `Layout::new<Self>`
    layout: alloc::Layout,

    // Offset from the beginning of the struct until `wrap`
    wrap_offset: usize,
    // `ptr::drop_in_place()` for `T`
    wrap_drop_in_place: unsafe fn(*mut T),
    wrap: T,
}

unsafe extern "C" fn free(_allocator: *mut ffi::GstAllocator, mem: *mut ffi::GstMemory) {
    let mem = mem as *mut WrappedMemory<()>;

    if (*mem).wrap_offset > 0 {
        let wrap = (mem as *mut u8).add((*mem).wrap_offset) as *mut ();
        ((*mem).wrap_drop_in_place)(wrap);
    }

    alloc::dealloc(mem as *mut u8, (*mem).layout);
}

unsafe extern "C" fn mem_map(
    mem: *mut ffi::GstMemory,
    _maxsize: usize,
    _flags: ffi::GstMapFlags,
) -> glib::ffi::gpointer {
    let mem = mem as *mut WrappedMemory<()>;

    (*mem).data as glib::ffi::gpointer
}

unsafe extern "C" fn mem_unmap(_mem: *mut ffi::GstMemory) {}

unsafe extern "C" fn mem_share(
    mem: *mut ffi::GstMemory,
    offset: isize,
    size: isize,
) -> *mut ffi::GstMemory {
    let mem = mem as *mut WrappedMemory<()>;

    // Basically a re-implementation of _sysmem_share()

    let parent = if (*mem).mem.parent.is_null() {
        mem
    } else {
        (*mem).mem.parent as *mut WrappedMemory<()>
    };

    // Offset and size are actually usizes and the API assumes that negative values simply wrap
    // around, so let's cast to usizes here and do wrapping arithmetic.
    let offset = offset as usize;
    let mut size = size as usize;

    let new_offset = (*mem).mem.offset.wrapping_add(offset);
    debug_assert!(new_offset < (*mem).mem.maxsize);

    if size == usize::MAX {
        size = (*mem).mem.size.wrapping_sub(offset);
    }
    debug_assert!(new_offset <= usize::MAX - size);
    debug_assert!(new_offset + size <= (*mem).mem.maxsize);

    let layout = alloc::Layout::new::<WrappedMemory<()>>();
    let sub = alloc::alloc(layout) as *mut WrappedMemory<()>;

    ffi::gst_memory_init(
        sub as *mut ffi::GstMemory,
        (*mem).mem.mini_object.flags | ffi::GST_MINI_OBJECT_FLAG_LOCK_READONLY,
        (*mem).mem.allocator,
        parent as *mut ffi::GstMemory,
        (*mem).mem.maxsize,
        (*mem).mem.align,
        new_offset,
        size,
    );
    ptr::write(ptr::addr_of_mut!((*sub).data), (*mem).data);
    ptr::write(ptr::addr_of_mut!((*sub).layout), layout);
    ptr::write(ptr::addr_of_mut!((*sub).wrap_offset), 0);
    ptr::write(ptr::addr_of_mut!((*sub).wrap_drop_in_place), |_| ());

    sub as *mut ffi::GstMemory
}

unsafe extern "C" fn mem_is_span(
    mem1: *mut ffi::GstMemory,
    mem2: *mut ffi::GstMemory,
    offset: *mut usize,
) -> glib::ffi::gboolean {
    let mem1 = mem1 as *mut WrappedMemory<()>;
    let mem2 = mem2 as *mut WrappedMemory<()>;

    // Basically a re-implementation of _sysmem_is_span()
    if !offset.is_null() {
        let parent = (*mem1).mem.parent as *mut WrappedMemory<()>;
        *offset = (*mem1).mem.offset - (*parent).mem.offset;
    }

    let is_span = (*mem1).data.add((*mem1).mem.offset).add((*mem1).mem.size)
        == (*mem2).data.add((*mem2).mem.offset);

    is_span.into_glib()
}

unsafe extern "C" fn class_init(class: glib::ffi::gpointer, _class_data: glib::ffi::gpointer) {
    let class = class as *mut ffi::GstAllocatorClass;

    (*class).free = Some(free);
}

unsafe extern "C" fn instance_init(
    obj: *mut glib::gobject_ffi::GTypeInstance,
    _class: glib::ffi::gpointer,
) {
    static ALLOCATOR_TYPE: &[u8] = b"RustGlobalAllocatorMemory\0";

    let allocator = obj as *mut ffi::GstAllocator;

    (*allocator).mem_type = ALLOCATOR_TYPE.as_ptr() as *const _;
    (*allocator).mem_map = Some(mem_map);
    (*allocator).mem_unmap = Some(mem_unmap);
    // mem_copy not set because the fallback already does the right thing
    (*allocator).mem_share = Some(mem_share);
    (*allocator).mem_is_span = Some(mem_is_span);

    // TODO: Could also implement alloc()
    (*allocator).object.flags |= ffi::GST_ALLOCATOR_FLAG_CUSTOM_ALLOC;
}

fn rust_allocator() -> &'static crate::Allocator {
    static RUST_ALLOCATOR: std::sync::OnceLock<crate::Allocator> = std::sync::OnceLock::new();

    RUST_ALLOCATOR.get_or_init(|| unsafe {
        struct TypeInfoWrap(glib::gobject_ffi::GTypeInfo);
        unsafe impl Send for TypeInfoWrap {}
        unsafe impl Sync for TypeInfoWrap {}

        static TYPE_INFO: TypeInfoWrap = TypeInfoWrap(glib::gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<ffi::GstAllocatorClass>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(class_init),
            class_finalize: None,
            class_data: ptr::null_mut(),
            instance_size: mem::size_of::<ffi::GstAllocator>() as u16,
            n_preallocs: 0,
            instance_init: Some(instance_init),
            value_table: ptr::null(),
        });

        let type_name = {
            let mut idx = 0;

            loop {
                let type_name = glib::gformat!("GstRsAllocator-{}", idx);
                if glib::gobject_ffi::g_type_from_name(type_name.as_ptr())
                    == glib::gobject_ffi::G_TYPE_INVALID
                {
                    break type_name;
                }
                idx += 1;
            }
        };

        let t = glib::gobject_ffi::g_type_register_static(
            crate::Allocator::static_type().into_glib(),
            type_name.as_ptr(),
            &TYPE_INFO.0,
            0,
        );

        assert!(t != glib::gobject_ffi::G_TYPE_INVALID);

        from_glib_none(
            glib::gobject_ffi::g_object_newv(t, 0, ptr::null_mut()) as *mut ffi::GstAllocator
        )
    })
}

impl Memory {
    #[doc(alias = "gst_memory_new_wrapped")]
    #[doc(alias = "gst_memory_new_wrapped_full")]
    #[inline]
    pub fn from_slice<T: AsRef<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        let len = slice.as_ref().len();
        unsafe {
            let layout = alloc::Layout::new::<WrappedMemory<T>>();
            let mem = alloc::alloc(layout) as *mut WrappedMemory<T>;

            ffi::gst_memory_init(
                mem as *mut ffi::GstMemory,
                ffi::GST_MINI_OBJECT_FLAG_LOCK_READONLY,
                rust_allocator().to_glib_none().0,
                ptr::null_mut(),
                len,
                0,
                0,
                len,
            );

            ptr::write(ptr::addr_of_mut!((*mem).wrap), slice);

            assert_eq!(len, (*mem).wrap.as_ref().len());
            let data = (*mem).wrap.as_ref().as_ptr();
            ptr::write(ptr::addr_of_mut!((*mem).data), mut_override(data));

            ptr::write(ptr::addr_of_mut!((*mem).layout), layout);

            let wrap_offset = ptr::addr_of!((*mem).wrap) as usize - mem as usize;
            ptr::write(ptr::addr_of_mut!((*mem).wrap_offset), wrap_offset);

            ptr::write(
                ptr::addr_of_mut!((*mem).wrap_drop_in_place),
                ptr::drop_in_place::<T>,
            );

            from_glib_full(mem as *mut ffi::GstMemory)
        }
    }

    #[doc(alias = "gst_memory_new_wrapped")]
    #[doc(alias = "gst_memory_new_wrapped_full")]
    #[inline]
    pub fn from_mut_slice<T: AsMut<[u8]> + Send + 'static>(mut slice: T) -> Self {
        assert_initialized_main_thread!();

        let len = slice.as_mut().len();
        unsafe {
            let layout = alloc::Layout::new::<WrappedMemory<T>>();
            let mem = alloc::alloc(layout) as *mut WrappedMemory<T>;

            ffi::gst_memory_init(
                mem as *mut ffi::GstMemory,
                0,
                rust_allocator().to_glib_none().0,
                ptr::null_mut(),
                len,
                0,
                0,
                len,
            );

            ptr::write(ptr::addr_of_mut!((*mem).wrap), slice);

            assert_eq!(len, (*mem).wrap.as_mut().len());
            let data = (*mem).wrap.as_mut().as_mut_ptr();
            ptr::write(ptr::addr_of_mut!((*mem).data), data);

            ptr::write(ptr::addr_of_mut!((*mem).layout), layout);

            let wrap_offset = ptr::addr_of!((*mem).wrap) as usize - mem as usize;
            ptr::write(ptr::addr_of_mut!((*mem).wrap_offset), wrap_offset);

            ptr::write(
                ptr::addr_of_mut!((*mem).wrap_drop_in_place),
                ptr::drop_in_place::<T>,
            );

            from_glib_full(mem as *mut ffi::GstMemory)
        }
    }
}
