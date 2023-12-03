use gst::{prelude::*, subclass::prelude::*};
use gst_audio::subclass::prelude::*;

mod imp;

// This here defines the public interface of our element and implements
// the corresponding traits so that it behaves like any other gst::Element
//
// GObject
//     ╰──GstObject
//         ╰──GstElement
//             ╰──GstBaseTransform
//                 ╰──GstAudioFilter
//                     ╰──IirFilter
glib::wrapper! {
    pub struct IirFilter(ObjectSubclass<imp::IirFilter>) @extends gst_audio::AudioFilter, gst_base::BaseTransform, gst::Element, gst::Object;
}

/// Trait containing extension methods for `IirFilter`.
pub trait IirFilterExt: IsA<IirFilter> {
    // Sets the coefficients by getting access to the private struct and simply setting them
    fn set_coeffs(&self, a: Vec<f64>, b: Vec<f64>) {
        self.upcast_ref::<IirFilter>().imp().set_coeffs(a, b)
    }
}

impl<O: IsA<IirFilter>> IirFilterExt for O {}

/// Trait to implement in `IirFilter` subclasses.
pub trait IirFilterImpl: AudioFilterImpl {
    /// Called whenever the sample rate is changing.
    fn set_rate(&self, rate: u32) {
        self.parent_set_rate(rate);
    }
}

/// Trait containing extension methods for `IirFilterImpl`, specifically methods for chaining
/// up to the parent implementation of virtual methods.
pub trait IirFilterImplExt: IirFilterImpl {
    fn parent_set_rate(&self, rate: u32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = &*(data.as_ref().parent_class() as *mut Class);
            (parent_class.set_rate)(self.obj().unsafe_cast_ref(), rate)
        }
    }
}

impl<T: IirFilterImpl> IirFilterImplExt for T {}

/// Class struct for `IirFilter`.
#[repr(C)]
pub struct Class {
    parent: <<imp::IirFilter as ObjectSubclass>::ParentType as glib::ObjectType>::GlibClassType,

    set_rate: fn(&IirFilter, rate: u32),
}

unsafe impl ClassStruct for Class {
    type Type = imp::IirFilter;
}

impl std::ops::Deref for Class {
    type Target = glib::Class<<<Self as ClassStruct>::Type as ObjectSubclass>::ParentType>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.parent as *const _ as *const _) }
    }
}

unsafe impl<T: IirFilterImpl> IsSubclassable<T> for IirFilter {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let class = class.as_mut();

        class.set_rate = |obj, rate| unsafe {
            let imp = obj.unsafe_cast_ref::<T::Type>().imp();
            imp.set_rate(rate);
        };
    }
}
