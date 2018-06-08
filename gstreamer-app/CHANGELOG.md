# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
specifically the [variant used by Rust](http://doc.crates.io/manifest.html#the-version-field).

## [0.11.3] - 2018-06-08
### Added
- `gst::Bus::remove_watch()` is now available to remove a bus watch again
- `fmt::Debug` impls for `AudioInfo` and `VideoInfo` were added
- `fmt::Debug` impls for mini objects also print the pointer value now to make
  it easier to track them in debug logs
- `PlayerVisualization` has accessors for the name and description fields now,
  without which there is no sensible way to use them or to set a player
  visualization

## [0.11.2] - 2018-05-09
### Fixed
- Work-around various floating reference handling changes between 1.12 and
  1.14 to be able to run with both versions without memory leaks or other
  reference count problems.
  This affects NetTimeProvider, BufferPool, DeviceMonitor, Stream,
  StreamCollection, and Player, NetClientClock, NetClock, PtpClock which were
  already previously fixed.

### Changed
- Change the appsrc need-data and all appsink callbacks to not require the
  Sync bound anymore and change from Fn to FnMut. They can only be called from
  a single thread at a time. This change is only done for the corresponding
  callbacks, not the signals.

## [0.11.1] - 2018-04-07
### Fixed
- Fix Structure::to_string() to not run into an infinite recursion but call
  the method on the contained StructureRef instead of on itself

## [0.11.0] - 2018-03-20
### Changed
- Updated everything to GStreamer 1.14.0
- Event, Message and Query types were refactored to improve usability.
  Especially newly constructed queries allow to directly use the type-specific
  functions to be used without first creating a view
- VideoFrameRef::copy_to_ref() and ::copy_plane_to_ref() are gone now and the
  original functions work with refs instead of full frames
- PadProbeId and NotifyIds are not Copy/Clone anymore and are taken by value
- GstPlayer has GstObject as parent class now

### Added
- GstPbutils, GstSdp, GstRtsp and GstRtspServer bindings
- GstPromise, GstAudioStreamAlign and various other 1.14 API
- GstVideoFilter and GstBufferPool bindings
- Element::call_async()
- Debug impl For Toc and TocEntry
- Various new examples (RTP FEC, RTSP server, tag usage, ...)

### Fixed
- Memory leak in gst_video::convert_sample_async()

## [0.10.2] - 2018-02-18
### Fixed
- Fix building of messages with custom fields for types that don't have a
  GstStructure

### Added
- VideoFrameRef::copy_to_ref() and ::copy_plane_to_ref(), which work with
  VideoFrameRefs instead of full VideoFrames
- Getters for the BaseSrc/Sink/Transform configured segment
- Document the gstreamer-player-1.0 dependency in the README.md

## [0.10.1] - 2018-01-03
### Fixed
- Don't require &mut self for TagSetterExtManual::add()

### Added
- A TagSetter example application
- Bindings for gst_video::convert_sample() and ::convert_sample_async()
- Bindings for gst_video::VideoRectangle
- Debug impl for Sample and ::with_buffer_list() constructor
- A borrowing version of VideoFrame: VideoFrameRef
- Bindings for GstVideoFilter

### Changed
- Deprecated Sample::get_info() in favour of ::get_structure()
- Player has gst::Object as another parent class now

## [0.10.0] - 2017-12-22
### Fixed
- Various clippy warnings
- Memory leak of the tag list in Toc::merge_tags()
- Property getters use Values of the correct type
- Event::get_structure(), Message::get_structure() and
  Query::get_structure() can return None for the structure
- Various other nullability fixes all over the API, changing functions to
  accept Option<> or returning Option<>, or only plain types
- Functions taking paths/filenames now actually take Paths instead of &strs
- Element::remove_pad() is not giving away a new reference to the pad
  anymore, which caused a memory leak of all pads ever removed
- Precision handling in ClockTime's Display impl
- Video/AudioInfo are only Send, not Sync

### Added
- Various enums now also derive useful traits like Copy, Clone and Hash in
  addition to PartialEq, Eq and Debug
- TagList::merge() and insert() for combining tag lists
- EventType gained many useful functions to work with event types and
  a PartialOrd impl to check expected event order of event types where it matters
- MessageRef/EventRef/QueryRef implement ToOwned
- Bindings for Registry and PluginFeature
- Event::set_running_time_offset() for adjusting the offset while events
  pass through the pipeline
- Event/Message GroupIds and Seqnums now have a newtype wrapper around u32
  instead of the plain value, making usage of them slightly more typesafe.
  Also add an "invalid" value for both, as exists in latest GStreamer now.
- FormattedValue, GenericFormattedValue and related types were
  implemented now, which allows more convenient and type-safe usage of
  formatted values (time, bytes, etc)
- Bindings for force-keyunit and still-frame events were added
- MappedBuffer/BufferMap now implement various other useful traits, including
  AsRef<[u8]>, AsMut, Deref, DerefMut, Debug, PartialEq and Eq
- Add VideoMultiviewFramePacking enum, and use it in Player
- Bindings for the GStreamer Net library, including PTP/NTP/network client
  clocks and the GStreamer NetClock provider for network synchronization of
  pipelines
- IteratorError implements std::error:Error
- Plugin::add_dependency() and ::add_dependency_simple() was added
- Rank and TypeFindProbability implement PartialOrd/Ord now
- Bindings for TypeFind, TypeFindFactory and the typefind helpers
- StreamCollection::iter() for iterating over all contained streams
- ErrorMessage type that can be used e.g. in a Result for passing an error
  message from somewhere to upper layers to then be posted on an element the
  same way gst_element_error!() would've done

### Changed
- Sample::new(), TagList::add(), Structure::set() and similar
  functions take the values (ToSendValue impls) by reference instead of value.
  They were not consumed by the function before.
- The Debug impls of various types, including Event/Buffer/Message/Query/Structure
  were improved to print all the fields, similar to what GST_PTR_FORMAT would
  do in C
- Switched to lazy_static 1.0
- Gap event and Duration tag are using ClockTimes now, as well as various
  Player signals
- Segment is now based on a generic type FormattedSegment that can
  take any format (time, bytes, etc) or a GenericFormattedValue for more
  type-safety and convenience. Also functions for "casting" between a generic
  segment and a segment with a specific format exist on this now
- AppSrc and AppSink now have a builder for the callbacks, making it
  unnecessary to always provide all callbacks even if only one is actually
  needed
- Various functions that returned bool for errors, are now returning a Result
- Player configuration is now a custom type with more convenient API
- Player VideoInfo uses a Fraction instead of (u32,u32) for the framerate and
  pixel-aspect-ratio
- VideoFrame API has more consistent API between writable and read-only
  variants
- Buffer::copy_into() was added, and ::copy_region() now takes a
  BufferCopyFlags parameter instead of always using the default flags
- ChildProxy::set_child_property() takes a &ToValue now to follow the API of
  Object::set_property() and improve usability
- Proxy/GhostPad default pad functions use the correct specific pad type now
  instead of a generic Pad
- Bus::add_signal_watch_full() takes a Priority for the priority instead of u32
- Clock::(un)adjust_with_calibration() takes no clock parameter anymore

### Removed
- FormatValue was removed in favour of GenericFormattedValue and the
  connected traits and specific format impls

## [0.9.1] - 2017-11-26
### Fixed
- Export `FlowError`/`FlowSuccess`, `ClockError`/`ClockSuccess`,
  `PadLinkError`/`PadLinkSuccess` too

## [0.9.0] - 2017-11-26
### Added
- Bindings for (outputting to) the GStreamer logging system
- Bindings for the GStreamer base library
- Bindings for all the `Pad` functions to override pad behaviour, and pad task
  functions
- Bindings for `StaticCaps` and `StaticPadTemplate`
- Bindings for `deep-notify` signal on `Object`
- Support for directly creating `Error`/`Warning`/`Info` `Messages` and posting them
  from an element with context information (file, line, module, etc.) similar
  to the C `GST_ELEMENT_ERROR` macro
- Support for setting custom fields in `Messages`/`Events` during construction
- Support for creating Buffers out of anything that is `AsRef<[u8]>` or
  `AsMut<[u8]>`
- Support for using the `Read` trait on `Adapter`
- Functions for getting all sink/src/all pads of an `Element`, and all children
  of a `Bin`
- Builder for `Caps` and `Structures` in addition to the existing functions
- `AppSrc`/`AppSink` implement `BaseSrc`/`BaseSink` and `URIHandler`
- Rust ports of the basic tutorials 1 to 8 from
  https://gstreamer.freedesktop.org/documentation/tutorials/
- "Getting started" and "Installation" sections to the README.md
- "dox" feature for generating documentation for all available configurations

### Fixed
- `StackTraceFlags` are only available since 1.12
- Worked around macOS requiring a `NSRunLoop` running on the main thread in all
  examples and tutorials, to be able to show a window or anything else

### Changed
- `ClockTime` is now a wrapper around `Option<u64>` to handle the
  `CLOCK_TIME_NONE` case better. This wrapper implements all the arithmetic
  and other traits as needed and ensures that no accidential calculations with
  `CLOCK_TIME_NONE` can happen
- "Values with format", like in `Duration`/`Position`/`Convert` queries or
  `Seek` events now return a `FormatValue` type. This contains the actual
  `Format` together with the value and does any required conversions. This
  also makes it harder to accidentially mix e.g. values in bytes and time
- `PadProbeId` does not implement `Clone`/`Copy` anymore
- Property notify watches return a custom type instead of ulong
- `Error`/`Warning`/`Info` `Messages` can only be created with specific kinds of
  `glib::Error` now. Using arbitrary ones does not work
- `Iterator` bindings were completely rewritten and provide the item type as a
  generic type parameter now, greatly simplifying its usage
- All `glib::Values` are now `glib::SendValue` instead, e.g. in `Caps` and
  `Structures`, as their content must be possible to send to different threads
  safely
- `Message::get_src()` can return `None`
- Allow `None` as `Caps` in `AppSrc`/`AppSink`
- Allow everything implementing `Into<Option<&str>>` to be used as a pad name
- Moved `copy()` from `GstRc` directly to `MiniObject`
- Success/Error enums (like `FlowReturn`, `PadLinkReturn`, `StateChangeReturn`) now
  implement an `into_result()` function that splits them into a `Result` with
  the good and bad cases. Also mark them as `#[must_use]` to make it harder to
  accidentially ignore errors.
- Error enums implement the `Error` trait

- Many examples use the `failure` crate for error handling now, cleaning up the
  error handling code quite a bit
- Lots of other code cleanup, compiler/clippy warning cleanup, etc.

## [0.8.2] - 2017-11-11
### Fixed
- Implement StaticType of BufferRef instead of Buffer. Buffer aka
  GstRc<BufferRef> already implements StaticType if BufferRef does, and
  without this it was not possible to use Buffers in GValues.
- Free memory of the appsink/appsrc callbacks with the correct type. It was
  crashing because of using the wrong type before.
- Fix documentation URLs in Cargo.toml.

### Added
- Installation instructions and links to documentation for getting started to
  README.md.

## [0.8.1] - 2017-09-15
### Added
- Implement Send+Sync for Query, Message and Event, and their corresponding
  Ref types.

### Fixed
- Constructor for gst_player::Player now works properly with GStreamer 1.12
  when passing a video renderer or signal dispatcher. There was a reference
  counting bug.
- Instead of returning &'static references from functions, return references
  with a generic, unbound lifetime instead.
  See https://github.com/rust-lang/rust/pull/42417#issue-233404573
- Various "unused external crate" warnings and clippy warnings everywhere.

### Changed
- Remove Cargo.lock from GIT, it's not very useful for library crates.
- Run everything through latest rustfmt-nightly.
- Use while-let (instead of loop and if-let) and CLOCK_TIME_NONE (instead of
  u64::MAX) in the examples.

## [0.8.0] - 2017-08-31

- Initial release of the autogenerated GStreamer bindings. Older versions
  (< 0.8.0) of the bindings can be found [here](https://github.com/arturoc/gstreamer1.0-rs).
  The API of the two is incompatible.

[Unreleased]: https://github.com/sdroege/gstreamer-rs/compare/0.11.3...HEAD
[0.11.3]: https://github.com/sdroege/gstreamer-rs/compare/0.11.2...0.11.3
[0.11.2]: https://github.com/sdroege/gstreamer-rs/compare/0.11.1...0.11.2
[0.11.1]: https://github.com/sdroege/gstreamer-rs/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/sdroege/gstreamer-rs/compare/0.10.2...0.11.0
[0.10.2]: https://github.com/sdroege/gstreamer-rs/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/sdroege/gstreamer-rs/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/sdroege/gstreamer-rs/compare/0.9.1...0.10.0
[0.9.1]: https://github.com/sdroege/gstreamer-rs/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/sdroege/gstreamer-rs/compare/0.8.1...0.9.0
[0.8.2]: https://github.com/sdroege/gstreamer-rs/compare/0.8.1...0.8.2
[0.8.1]: https://github.com/sdroege/gstreamer-rs/compare/0.8.0...0.8.1
