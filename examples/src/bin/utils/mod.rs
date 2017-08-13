
extern crate gstreamer as gst;
use gst::*;

extern crate glib;

use std::fmt;

#[derive(Debug, Clone)]
pub enum ExampleError {
    InitFailed(glib::Error),
    ElementNotFound(&'static str),
    ElementLinkFailed(::std::string::String, ::std::string::String),
    SetStateError(::std::string::String),
    ElementError(::std::string::String, glib::Error, ::std::string::String),
    MissingFeature(&'static str),
}

impl fmt::Display for ExampleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExampleError::InitFailed(ref e) => {
                write!(f, "GStreamer initialization failed: {:?}", e)
            }
            ExampleError::ElementNotFound(e) => write!(f, "Element {} not found", e),
            ExampleError::ElementLinkFailed(ref e1, ref e2) => {
                write!(f, "Link failed between {} and {}", e1, e2)
            }
            ExampleError::SetStateError(ref state) => {
                write!(f, "Pipeline failed to switch to state {}", state)
            }
            ExampleError::ElementError(ref element, ref err, ref debug) => {
                write!(f, "Error from {}: {} ({:?})", element, err, debug)
            }
            ExampleError::MissingFeature(ref feature) => write!(
                f,
                "Feature {} is required. Please rebuild with --features {}",
                feature,
                feature
            ),
        }
    }
}

pub fn create_element(name: &'static str) -> Result<gst::Element, ExampleError> {
    gst::ElementFactory::make(name, None).ok_or_else(|| ExampleError::ElementNotFound(name))
}

pub fn link_elements(e1: &gst::Element, e2: &gst::Element) -> Result<(), ExampleError> {
    match gst::Element::link(e1, e2) {
        Ok(o) => Ok(o),
        Err(_) => Err(ExampleError::ElementLinkFailed(
            e1.get_name(),
            e2.get_name(),
        )),
    }
}

pub fn set_state(e: &gst::Pipeline, state: gst::State) -> Result<(), ExampleError> {
    if let gst::StateChangeReturn::Failure = e.set_state(state) {
        return Err(ExampleError::SetStateError(
            gst::Element::state_get_name(state).unwrap(),
        ));
    }
    Ok(())
}
