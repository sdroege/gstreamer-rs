// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, translate::*};
use gst::LoggableError;
use gst::subclass::prelude::*;

use crate::{TagDemux, ffi};

pub trait TagDemuxImpl: ElementImpl + ObjectSubclass<Type: IsA<TagDemux>> {
    // rustdoc-stripper-ignore-next
    /// Minimum size required to identify a tag at the start and determine its total size.
    const MIN_START_SIZE: usize;

    // rustdoc-stripper-ignore-next
    /// Minimum size required to identify a tag at the end and determine its total size.
    const MIN_END_SIZE: usize;

    // rustdoc-stripper-ignore-next
    /// Identify tag and determine the size required to parse the tag
    fn identify_tag(&self, buffer: &gst::Buffer, start_tag: bool) -> Option<usize> {
        self.parent_identify_tag(buffer, start_tag)
    }

    // rustdoc-stripper-ignore-next
    /// Parse the tag once it is identified and its size is known:
    /// - Ok(size, tags) = GST_TAG_DEMUX_RESULT_OK
    /// - Ok(size, None) = GST_TAG_DEMUX_RESULT_AGAIN
    /// - Err(error_log) = GST_TAG_DEMUX_RESULT_BROKEN_TAG
    fn parse_tag(
        &self,
        buffer: &gst::Buffer,
        start_tag: bool,
    ) -> Result<(usize, Option<gst::TagList>), LoggableError> {
        self.parent_parse_tag(buffer, start_tag)
    }

    // rustdoc-stripper-ignore-next
    /// Merge start and end tags
    fn merge_tags(
        &self,
        start_tags: Option<&gst::TagList>,
        end_tags: Option<&gst::TagList>,
    ) -> Option<gst::TagList> {
        self.parent_merge_tags(start_tags, end_tags)
    }
}

pub trait TagDemuxImplExt: TagDemuxImpl {
    fn parent_identify_tag(&self, buffer: &gst::Buffer, start_tag: bool) -> Option<usize> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstTagDemuxClass;
            let f = (*parent_class)
                .identify_tag
                .expect("parent function `identify_tag` should be defined");
            let mut tag_size = 0u32;
            if from_glib(f(
                self.obj().unsafe_cast_ref::<TagDemux>().to_glib_none().0,
                buffer.to_glib_none().0,
                start_tag.into_glib(),
                &mut tag_size,
            )) {
                Some(tag_size as usize)
            } else {
                None
            }
        }
    }

    fn parent_parse_tag(
        &self,
        buffer: &gst::Buffer,
        start_tag: bool,
    ) -> Result<(usize, Option<gst::TagList>), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstTagDemuxClass;
            let f = (*parent_class)
                .parse_tag
                .expect("parent function `parse_tag` should be defined");
            let buffer_ptr = buffer.as_mut_ptr() as *mut _;
            let mut tag_size: u32 = buffer.as_ref().size() as u32;
            let mut tags_ref = ptr::null_mut();
            let ret = f(
                self.obj().unsafe_cast_ref::<TagDemux>().to_glib_none().0,
                buffer_ptr,
                start_tag.into_glib(),
                &mut tag_size,
                &mut tags_ref,
            );
            match ret {
                ffi::GST_TAG_DEMUX_RESULT_OK => {
                    Ok((tag_size as usize, Some(from_glib_full(tags_ref))))
                }
                ffi::GST_TAG_DEMUX_RESULT_AGAIN => Ok((tag_size as usize, None)),
                ffi::GST_TAG_DEMUX_RESULT_BROKEN_TAG => {
                    Err(gst::loggable_error!(gst::CAT_RUST, "Broken tag"))
                }
                _ => unreachable!(),
            }
        }
    }

    fn parent_merge_tags(
        &self,
        start_tags: Option<&gst::TagList>,
        end_tags: Option<&gst::TagList>,
    ) -> Option<gst::TagList> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstTagDemuxClass;
            if let Some(f) = (*parent_class).merge_tags {
                let ptr = f(
                    self.obj().unsafe_cast_ref::<TagDemux>().to_glib_none().0,
                    start_tags
                        .map(|taglist| taglist.as_ptr())
                        .unwrap_or(ptr::null_mut()),
                    end_tags
                        .map(|taglist| taglist.as_ptr())
                        .unwrap_or(ptr::null_mut()),
                );
                Option::<_>::from_glib_full(ptr)
            } else {
                match (start_tags, end_tags) {
                    (Some(start_tags), None) => Some(start_tags.clone()),

                    (None, Some(end_tags)) => Some(end_tags.clone()),

                    // This mirrors what GstTagDemux does by default:
                    // We merge in REPLACE mode, so put the less important tags first, which
                    // we'll just assume is the end tag (subclasses may change this behaviour
                    // or make it configurable by overriding the merge_tags vfunc.
                    (Some(start_tags), Some(end_tags)) => {
                        Some(end_tags.merge(start_tags, gst::TagMergeMode::Replace))
                    }

                    (None, None) => None,
                }
            }
        }
    }
}

impl<T: TagDemuxImpl> TagDemuxImplExt for T {}

unsafe impl<T: TagDemuxImpl> IsSubclassable<T> for TagDemux {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.identify_tag = Some(tagdemux_identify_tag::<T>);
        klass.parse_tag = Some(tagdemux_parse_tag::<T>);
        klass.merge_tags = Some(tagdemux_merge_tags::<T>);

        klass.min_start_size = T::MIN_START_SIZE as u32;
        klass.min_end_size = T::MIN_END_SIZE as u32;
    }
}

unsafe extern "C" fn tagdemux_identify_tag<T: TagDemuxImpl>(
    ptr: *mut ffi::GstTagDemux,
    buffer: *mut gst::ffi::GstBuffer,
    start_tag: glib::ffi::gboolean,
    tag_size: *mut u32,
) -> glib::ffi::gboolean {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, false, {
            match imp.identify_tag(&from_glib_borrow(buffer), from_glib(start_tag)) {
                Some(size) => {
                    *tag_size = size as u32;
                    true
                }
                None => false,
            }
        })
        .into_glib()
    }
}

unsafe extern "C" fn tagdemux_parse_tag<T: TagDemuxImpl>(
    ptr: *mut ffi::GstTagDemux,
    buffer: *mut gst::ffi::GstBuffer,
    start_tag: glib::ffi::gboolean,
    tag_size: *mut u32,
    tags: *mut *mut gst::ffi::GstTagList,
) -> ffi::GstTagDemuxResult {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, ffi::GST_TAG_DEMUX_RESULT_BROKEN_TAG, {
            match imp.parse_tag(&from_glib_borrow(buffer), from_glib(start_tag)) {
                Ok((size, Some(tag_list))) => {
                    *tag_size = size as u32;
                    *tags = tag_list.into_glib_ptr();
                    ffi::GST_TAG_DEMUX_RESULT_OK
                }

                Ok((size, None)) => {
                    *tag_size = size as u32;
                    ffi::GST_TAG_DEMUX_RESULT_AGAIN
                }

                Err(err) => {
                    err.log_with_imp(imp);
                    ffi::GST_TAG_DEMUX_RESULT_BROKEN_TAG
                }
            }
        })
    }
}

unsafe extern "C" fn tagdemux_merge_tags<T: TagDemuxImpl>(
    ptr: *mut ffi::GstTagDemux,
    start_tags: *const gst::ffi::GstTagList,
    end_tags: *const gst::ffi::GstTagList,
) -> *mut gst::ffi::GstTagList {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        gst::element_panic_to_error!(imp, None, {
            imp.merge_tags(
                Option::<_>::from_glib_borrow(start_tags).as_ref().as_ref(),
                Option::<_>::from_glib_borrow(end_tags).as_ref().as_ref(),
            )
        })
        .to_glib_full()
    }
}
