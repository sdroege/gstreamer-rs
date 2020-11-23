# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
specifically the [variant used by Rust](http://doc.crates.io/manifest.html#the-version-field).

## [0.16.5] - 2020-11-23
### Fixed
- Make sure to use `$crate` in more macros to allow them to work without
  anything special in scope already.
- Update documentation location.
- Don't panic if C code stores invalid seqnums in events and the seqnum is
  used directly or via the `Display` impl.
- Fix docs build for some crates on docs.rs.
- Fix `Debug` impl for `gst_video::VideoTimeCode` to print the correct type
  name.
- Fix plugin version to be 1.18 instead of 1.17 when compiling a plugin with
  `v1_18`.

### Added
- Event handling support in pad probes, that is returning
  `PadProbeReturn::Handled` for events.
- `EventRef::get_structure_mut()` getter that allows changing the events'
  structures.

### Changed
- Remove unnecessary `PhantomData` markers and use `repr(transparent)` instead
  of `repr(C)` where it is more correct.

## [0.16.4] - 2020-10-09
### Fixed
- Correctly implement `ExactSizeIterator` on the `AudioFormat` and
  `VideoFormat` iterators. Previously they returned the overall size instead
  of the remaining size, and they didn't implement `Iterator::size_hint()`.
- Don't implement `ExactSizeIterator` on the buffer `gst::Meta` iterator. The
  overall length is not known easily and the implementation would've simply
  panicked in the past.

### Added
- `gst::ClockID::wait_async_stream()` for async integration for clock waiting.
- `From` / `TryFrom` impls for converting between `gst::ClockTime` and
  `std::time::Duration`.

## [0.16.3] - 2020-09-08
### Fixed
- Reset vfuncs if calling `BaseTransformClass::configure()` multiple times.
- Fix `gst::debug_remove_default_log_function()` to actually remove the
  default log function.

### Added
- Some more new APIs added in 1.18.
- API for getting an owned buffer from a readable `gst_video::VideoFrame` /
  `VideoFrameRef`.

### Changed
- Updated bindings to 1.18.0. This stabilized GStreamer 1.18 support and any
  API behind the "v1_18" feature is considered stable now.
- Factor out some common code from `gst::Pad::ProbeInfo` code. This reduces
  the code generated for each pad probe considerably.
- Update paste dependency to 1.0 and pretty-hex to 0.2.

## [0.16.2] - 2020-07-27
### Fixed
- Use correct pointer for the plane data in `gst_audio::AudioBuffer`.

### Added
- Add `gst::GhostPad` convenience constructors that take a target pad, similar
  to the ones that existed in 0.15 and before.
- Add `gst::parse_bin_from_description_with_name` that allows setting a name
  for the created bin without having to use unsafe code in application code.

## [0.16.1] - 2020-07-10
### Fixed
- Allow calling `gst::DebugCategory::new()` before `gst::init()` again.

## [0.16.0] - 2020-07-06
### Added
- Updated bindings to 1.17.2, adding experimental 1.18 support. This can be
  opted-in via the "v1_18" feature flag but there might still be API changes
  in the newly added API.
- `gst::MemoryRef::dump()` for dumping contents of a memory.
- `gst::Bus::stream()` instead of a custom constructor on the `BusStream`.
- Use more accurate types for `Seqnum`, `GroupId` and `MetaSeqnum`. These are
  now proper wrapper types instead of plain integers, which makes mis-use
  harder.
- Provide `TryFrom` impls for conversion between `glib::DateTime` and
  `gst::DateTime`.
- Add `get_allocator()` functions to `gst_base::{Aggregator, BaseTransform,
  BaseSrc}`, and allow overriding `BaseSrc::alloc()`.
- Add subclassing bindings for `gst_base::PushSrc`.
- Add new `gst::BufferCursor` API that allows to handle a buffer as `Read`,
  `Write` and `Seek` and accesses the underlying memories of the buffer
  individually without mapping them all together.
- Add `gst::Plugin::get_plugin_name()`.
- Support for `gst_video::VideoAFDMeta` and `VideoBarMeta`.
- API for getting all / iterating over all `gst_audio::AudioFormat` and
  `gst_video::VideoFormat`.
- Bindings and subclassing bindings for `gst_video::VideoSink`.
- `gst::Pad` can be constructed via the builder pattern and `gst::PadBuilder`
  now, which allows to safely set the pad functions and various other fields
  during construction. The `PadBuilder` works on any `gst::Pad` subclass and
  also has special support for `GhostPad`s by allowing to set pad functions of
  the proxy pad.
- `gst::Message`, `gst::Event` and `gst::Query` type constructors are now on
  the specific target type instead of various `new_XXX()` functions on the
  basic type. E.g. `gst::message::Eos::new()`.
- Support for overriding `gst_audio::AudioSrc/Sink::reset()`.
- Support for overriding `gst_base::BaseParse::stop()`.
- Support for overriding `gst::Element::post_message()`.
- Added bindings for `gst::BufferList::foreach()` and `foreach_mut()`.
- Added bindings for `gst::Buffer::foreach_meta()` and `foreach_meta_mut()`.

### Fixed
- Allow using any `glib::Object` as target object for logging instead of just
  `gst::Object`.
- Remove restriction API from `gst_pbutils::EncodingContainerProfile`. They
  are supposed to be used only with the other encoding profiles.
- Return `&'static str` for various `gst::StructureRef` functions where the
  string is backed by a `glib::Quark`.
- Fix various `gst::DateTime` functions to actually return `Option`s.
- Add support for filling in a buffer passed to the `gst::Pad` getrange
  function, allow passing one in into `get_range()` and `pull_range()` and
  provide the corresponding API on `gst_base::BaseSrc` too.
- Allocator in audio/video `Decoder` base classes is optional and can return
  `None`.
- `gst_video::ValidVideoTimeCode::add_interval()` always returns a valid
  timecode again.
- Allow resolving a `gst::Promise` with `None` and also handle that correctly
  in the callback. This is allowed by the API.
- Allow calling various debugging related functions before `gst::init()`.
- Various enum/function versions were fixed to only show up if the
  corresponding version feature is enabled.
- `gst::Pad` function setters are marked unsafe now as changing the functions
  is not thread-safe.
- Remove `gst::Object::set_name()` as changing the name after construction
  generally causes problems and is potentially unsafe.
- Remove `gst::Pad::set_pad_template()` as changing the pad template after
  construction is generally unsafe.
- `gst::Pad::stream_lock()` borrows the pad now instead of taking a new
  reference.
- Unimplemented `Jitter` and `Buffer` queries were removed from the bindings.
  These are not implemented in C and only have a type registered.
- Various `LAST`, `NONE` variants of enums and flags were removed as these
  only make sense in C.
- Call the parent impl of various vfuncs that were omitted before to not
  require further subclasses of them to implement them but automatically call
  the parent ones.

### Changed
- Use `NonZeroU64/U32` for various ID types to allow further optimizations.
- Use `thiserror` crate for deriving error types.
- Switch from `lazy_static` to `once_cell`.
- Change various miniobject functions like `gst::Caps::append()` from taking
  the object by value to modifying it internally. This makes them easier to
  use and only applies to functions that are defined on the non-reference type
  and take ownership of the values passed in.
- Use `mem::ManuallyDrop` instead of `mem::forget()` everywhere.
- Replace most `mem::transmute()` calls with safer alternatives.
- `gst:StreamCollection` API was changed to the builder pattern for
  construction as the collection must not be changed after construction.
- `gst::ProxyPad` default functions are plain functions on `ProxyPad` now
  instead of trait functions to allow easier usage of them.
- Use proper error types in various `TryFrom` impls.
- `gst_video::VideoMeta::add()` returns a `Result` now instead of panicking.
- Various constructors were renamed from `new_with_XXX()` and `new_from_XXX()`
  to the more idiomatic `with_XXX()` and `from_XXX()`.
- Miniobject bindings are simplified now and there is no `gst::GstRc` type
  anymore, instead everything is directly implemented on the concrete types.
  As part of this the `gst::MiniObject` trait was also removed as it was
  unneeded now.

## [0.15.7] - 2020-06-08
### Fixed
- Allow multiple filter types per process with `gst::Iterator::filter()`.
- Check that `VideoInfo` is valid when creating a `VideoFrame`.
- Don't potentially dereference a `NULL` pointer when getting the format
  from an invalid `VideoInfo` or `AudioInfo`.
- Don't unmap borrowed `VideoFrameRef`s.

### Added
- `gst::ProtectionMeta`, `gst_video::VideoAffineTransformationMeta`,
  `VideoCropMeta` and `VideoRegionOfInterestMeta` bindings.
- Various new `gst_rtp::RTPBuffer` methods.
- `gst_audio::audio_buffer_truncate()`, `AudioMeta` and `AudioBuffer`
  bindings.

## [0.15.6] - 2020-05-28
### Fixed
- Assert that the data passed to `VideoCaptionMeta::add()` is not empty.
- Don't store strong references to the object in the bus, appsink and appsrc
  futures `Stream` / `Sink` adapters. This would keep them alive unnecessarily
  and would prevent the `Stream` / `Sink` to ever "finish" on its own.
- Handle receiving a `None` reply in the change function of `gst::Promise`.
  This is apparently valid. For backwards compatibility reasons this is
  currently replaced with an empty structure but in 0.16 the API will
  explicitly handle `None`.

### Added
- `gst::Stream::debug()` and `gst::StreamCollection::debug()` for converting
  into a structured string with the actual contents of each.
- `gst::Structure::from_iter()` and `gst::Caps::from_iter()` to create
  structures/caps from iterators.
- `gst::Event` support for getting/setting the `gst::Stream` in the
  `StreamStart` event.
- `gst_video::calculate_display_ratio()` and `::guess_framerate()`.
- Various video related `gst::CapsFeatures` in `gst_video`.
- `TryFrom`/`From` impls for converting between `gst::Structure` and
  `gst_video::VideoConverterConfig`.
- Various `glib::Value` trait impls for `SDPMessage`, `StructureRef`,
  `CapsFeatureRef` and all borrowed variants of miniobjects to be able to
  work with the borrowed, non-owned variants when handling `glib::Value`s.

## [0.15.5] - 2020-05-03
### Fixed
- Revert: Allow logging any `glib::Object` and not just `gst::Object`. This
  broke API in subtile ways and needs to wait until 0.16
- Replace `%` in log output with `%%` to prevent accidental C formatting
- Add missing manual traits to the documentation

### Added
- `BufferRef::peek_memory_mut()` to give a mutable reference to a given memory
- Different iterators for iterating over the memories of a buffer
- Support for `gst_audio::AudioClippingMeta`
- `gst::Plugin::get_plugin_name()` was added
- `gst::Element::get_current_clock_time()` and
  `gst::Element::get_current_running_time() helper functions
- `gst::State` and `StateChange` API for calculating next/previous state and
  convert from/to the components of a state change

### Changed
- Use `mem::ManuallyDrop` instead of `mem::forget` everywhere

## [0.15.4] - 2020-03-09
### Fixed
- Allow logging any `glib::Object` and not just `gst::Object`
- Fix floating reference handling in `RTSPMedia::take_pipeline()`
- Hold `GMutex` guards for the remainder of the function and warn if they're
  directly dropped
- Work around empty/any caps handling bugs in `Caps::fixate()`

### Added
- Add `BaseTransform::prepare_output_buffer()` subclassing support
- `RTSPServer`, `RTSPClient`, `RTSPMedia` and `RTSPMediaFactory` subclassing
  support
- Handle panicking in `appsrc`/`appsink` callbacks by posting an error message
  instead of killing the process

## [0.15.3] - 2020-02-15
### Fixed
- `UniqueFlowCombiner::clear()` should take a mutable reference.
- `AudioStreamAlign` doesn't require mutable references for getters anymore.
- Don't use bool return value of `gst_video_info_set_format()` and
  `gst_video_info_align()` with GStreamer < 1.11.1 as it returned void back
  then. We'd otherwise use some random value.
- Make `VideoInfo::align()` is available since 1.8.
- Fix changing/clearing of `AppSrc`, `AppSink` callbacks and `Bus` sync
  handler. Before 1.16.3 this was not thread-safe and caused crashes. When
  running with older versions changing them causes a panic now and unsetting
  the bus sync handler has not effect. With newer versions it works correctly.

### Added
- Add `Clone` impls for `BufferPoolConfig` and `PlayerConfig`.
- Add `VideoConverter` bindings.
- Add `Future`s variant for `gst::Promise` constructor.
- Add `Future`s variant for `gst_video::convert_sample_async()`.
- Add `submit_input_buffer()`, `generate_output()`, `before_transform()`,
  `copy_metadata()` and `transform_meta()` virtual method support for
  `BaseTransform`.
- Add `AppSink` `Stream` adapter and `AppSrc` `Sink` adapter for integrating
  both into Rust async contexts.

### Changed
- More generic implementations of `VideoFrame` / `VideoFrameRef` functions to
  allow usage in more generic contexts.

## [0.15.2] - 2020-01-30
### Fixed
- Fix another race condition in the `gst::Bus` `Stream` that could cause it to
  not wake up although a message is available.

## [0.15.1] - 2020-01-23
### Added
- Use static inner lifetime for `VideoCodecState<Readable>` so that it can be
  stored safely on the heap.
- Getters/setters for `BinFlags` on `gst::Bin`.
- `gst::Caps::builder_full()` for building caps with multiple structures
  conveniently.
- `gst::Element::call_async_future()` for asynchronously spawning a closure
  and returning a `Future` for awaiting its return value.

### Fixed
- Various clippy warnings.
- Getters/setters for `PadFlags` on `gst::Pad` now provide the correct
  behaviour.
- Take mutex before popping messages in the `gst::Bus` `Stream` to close a
  small race condition that could cause it to not be woken up.
- `gst::ChildProxy` implementers do not have to provide `child_added()` and
  `child_removed()` functions anymore but these are optional now.
- Manually implement `Debug` impls for various generic types where to `Debug`
  impl should not depend on their type parameters also implementing `Debug`.

## [0.15.0] - 2019-12-18
### Added
- `StructureRef::get_optional()` for returning `None` if the field does not
  exist instead of `Err`
- Bindings for `gstreamer-rtp` library, mostly `RTPBuffer`
- Support for writing `Preset`, `TagSetter`, `Clock`, `SystemClock` subclasses
- Bindings for `Typefind::get_length()`
- Bindings for `BaseSrcImpl::get_times()`
- Bindings (incl. subclassing) for `AudioSink` and `AudioSrc`
- Missing `Send`/`Sync` impl for various types

### Fixed
- Cleanup of cargo features/dependencies to improve build times
- Serde serialization with optional values.
  Attention: This changes the format of the serialization!
- `VideoEncoder`/`VideoDecoder` `proxy_getcaps()` can't return `None`
- Use non-panicking UTF8 conversion in log handler. We don't want to panic
  just because some C code printed a non-UTF8 string
- Re-rexport all traits from the crate level and also ensure that all traits
  are actually included in the preludes
- Actually export `is_video_overlay_prepare_window_handle_message()` function
- Use `FnMut` for the `appsink` callbacks instead of `Fn`
- `Promise` change function returns the actual reply to the promise now
  instead of just passing the promise itself
- Memory leak in `Iterator::filter()`
- `BinImpl::add()` takes ownership of floating references
- `DeviceImpl::create_element()` preserves floating flag
- `BinImpl::remove()` takes a strong reference of the element now as the last
  reference might be owned by the bin and otherwise we would potentially have
  a use-after-free afterwards
- `BaseParseFrame` and `VideoCodecFrame` take a `&mut self` now for various
  functions that actually change the frame

### Changed
- Minimum supported Rust version is 1.39
- Allow passing `None` to `VideoEncoder::finish_frame()`
- Various `to_string()` methods were moved into the `Display` trait impl and
  for some types `to_str()` was added to return a `&'static str`
- .gir files were updated to 1.16.2 release
- `Sample` constructor uses the builder pattern now
- `VideoMeta::add_full()` is simplified and requires parameters
- `BasetransformImpl::set_caps()` returns a `Result` instead of `bool`
- SDP data type getters for strings return an `Option` now as these can be
  `None` in practice although not allowed by the SDP spec
- Various functions returning `Option`s were changed to return `Results` if
  `None` actually signalled an error instead of just a missing value

### Removed
- "subclassing" and "futures" cargo features. These are enabled by default now

## [0.14.5] - 2019-09-17
### Added
- Support subclassing of `gst::Device`, `gst::DeviceProvider`,
  `gst_audio::AudioDecoder` and `::AudioEncoder`
- Support for `Element::set_clock` and `::provide_clock` virtual methods
- `ElementClass::add_metadata` was added
- `gst_video::VideoDecoder` and `::VideoEncoder` got support for `get_caps`,
  `negotiate`, `src/sink_query/event` and the `drain` virtual methods
- `Element::num_pads`, `::num_src_pads` and `::num_sink_pads` functions
- `gst_video::VideoDecoder` and `::VideoEncoder` got `get_allocator` bindings
- `gst::Iterator` implements `IntoIterator` now for providing
  `std::iter::Iterator<Item=<Result<T, IteratorError>>` adapter
- Error macros for audio/video decoder subclasses to handle decoding errors
  more gracefully and only actually error out after many consecutive errors

### Fixed
- Macros now also work in Rust 2018 edition without `#[macro_use]` but
  explicit imports
- The log handler unit test runs reliable in parallel with other tests
- Manually implement `Debug` for `gst::Iterator` to allow it for any `T`
  instead of `T: Debug`
- `Device::create_element` has correct reference count handling now
- Return `NotNegotiated` in the video codec base classes if setting the output
  state fails instead of `Error`

## [0.14.4] - 2019-08-14
### Added
- Bindings for adding/removing custom log functions
- Bindings for `calculate_linear_regression()`
- Constants for base class custom flow returns

### Fixed
- Ownership of pad in `Element::release_pad()` virtual method implementations

## [0.14.3] - 2019-07-16
### Added
- `Buffer::unset_flags()` for unsetting specific buffer flags
- `VideoBufferFlags` flags type and `VideoBufferExt::set_video_flags()`,
  `unset_video_flags()` and `get_video_flags()` for working with video buffer
  flags from safe code.

### Fixed
- Setting buffer flags does not override arbitrary other flags anymore but
  only sets the flags in question. This is necessary to not override extension
  buffer flags like `gst_video::VideoBufferFlags`.

## [0.14.2] - 2019-07-15
### Added
- Support for `ReferenceTimestampMeta`

## [0.14.1] - 2019-07-06
### Added
- Various new WebRTC enum types from 1.14.1/1.16.0

### Fixed
- Correctly generate interlaced `VideoInfo` by using
  `gst_video_info_set_interlaced_format()` instead of the generic function.
- serde serialization unit tests for `gst::format` succeed again now.

### Changed
- `Debug` impls for `VideoFormatInfo` and `AudioFormatInfo` now print all the
  details of the format instead of only the name, and the `Debug` impls for
  `VideoInfo` and `AudioInfo` also print the format now.

## [0.14.0] - 2019-06-24
### Added
- Bindings for `GLSyncMeta`.
- Bindings for setting/getting `TagScope` on a `TagList`
- Bindings for `GLDisplayWayland` and `GLDisplayX11` in addition to the
  already existing `GLDisplayEGL`
- Bindings for `Bus::pop_filtered()` and related functions
- Bindings for getting/setting `Object`, `Element`, `Bin`, `Pipeline` and
  `Plugin` flags
- Bindings for `VideoCaptionMeta`
- `Debug` impl of `Buffer` now also shows the metas of the buffers
- Expose flow return in `PadProbeInfo` for overriding the return value
- Bindings for `VideoDecoder` and `VideoEncoder`, including subclassing
  support
- Bindings for `Memory`, `Allocator` and `VideoBufferPool`
- Bindings for `VideoFormatInfo::pack` and `::unpack` for format conversion
- Bindings for `BaseParse`, including subclassing support
- Various new arithmetic operation impls for fractions, formatted values and
  `ClockTime`
- Bindings for `VideoInfo::align()`

### Changed
- The `SDPMessage` and `SDPMedia` bindings were completely rewritten as they
  were broken before and caused crashes in various usages. As part of this
  there's also some more convenience API available on these types, like
  iterators for example, and API to modify the `SDPMedia` contained in a
  `SDPMessage`.
- Update to GStreamer 1.16.
- Regenerate with latest gir.
- Run all autogenerated code through rustfmt after generation too.
- Updated to latest versions of GLib/GIO/etc crates.
- Updated to futures 0.3 / `std::future`
- `ProxyPad` default functions moved to an extension trait instead of plain
  functions on `ProxyPad`, making them more in sync with the default `Pad`
  functions
- GStreamer plugins are now exporting the new 1.14+ plugin symbols if they
  were configured for GStreamer 1.14+
- Arithmetic operations on formatted values and `ClockTime` do overflow checks
  now and replace the result with the `NONE` value on overflow
- `TryFrom`/`TryInto` traits are used in various places now instead of the
  previous ad-hoc implementations of them.
- Registering element/typefind/device monitor factories requires passing a
  value of `gst::Rank` now instead of an arbitrary `u32`

### Fixed
- Use correct type for destroying pad task closure data. This was previously
  using the wrong type, causing crashes at runtime.
- `DeviceAdded`/`DeviceRemoved` message getters are transfer full so we don't
  need to take an additional reference that would be leaked.
- `AppSink` callbacks are correctly marked as `Send` instead of `Send+Sync`,
  allowing a wider range of closures to be used for them.
- Handle `PadProbeReturn::Handled` return values from pad probes more
  correctly.
- `ToOwned::to_owned()` on miniobjects has to create copies instead of
  only increasing the reference count. Otherwise it was possible to create
  multiple mutable and immutable references to the same object at the same
  time.
- Various functions take references to owned miniobjects instead of borrowed
  references as it was otherwise possible to create multiple mutable or
  immutable references to the same object at the same time.
- `URIHandler::set_uri` does not accept `None` anymore as this is not allowed
  by the C function.
- Comparisons and addition of `TypeFindProbability` and `Rank` work correctly now
- Various `Display` implementations were fixed to not cause a stack overflow
  due to infinite recursion anymore
- Various `::to_string()` functions don't take ownership of C strings anymore
  that they do not own, which caused double frees before

### Removed
- MIKEY related bindings from the SDP library. The bindings were broken and
  until someone needs them these are not available anymore.

## [0.13.0] - 2019-02-22
### Added
- Subclassing infrastructure was moved directly into the bindings,
  making the `gst-plugin` crate deprecated. This involves many API
  changes but generally cleans up code and makes it more flexible.
  Take a look at the `gst-plugins-rs` crate for various examples.
- Bindings for GStreamer GL library
- Bindings for `CapsFeatures` and `Meta`
- Bindings for `ParentBufferMeta, `VideoMeta` and `VideoOverlayCompositionMeta`
- Bindings for `VideoOverlayComposition` and `VideoOverlayRectangle`
- Bindings for `VideoTimeCode`
- Bindings for `NetAddressMeta`
- Bindings for registering custom tags
- `UniqueFlowCombiner` and `UniqueAdapter` wrappers that make use of
  the Rust compile-time mutability checks and expose more API in a safe
  way, and as a side-effect implement `Sync` and `Send` now
- `Bus::add_watch_local()` and `gst_video::convert_frame_async_local()` that
  allows to use a closure that does not implement `Send` but can only be
  called from the thread owning the main context.
- More complete bindings for `Allocation` `Query`
- `pbutils` functions for codec descriptions
- `TagList::iter()` for iterating over all tags while getting a single
   value per tag. The old `::iter_tag_list()` function was renamed to
   `::iter_generic()` and still provides access to each value for a tag
- `Bus::iter()` and `Bus::iter_timed()` iterators around the
  corresponding `::pop*()` functions
- Getters for `VideoColorimetry` to access its fields
- `Debug` impls for various missing types.
- serde serialization of `Value` can also handle `Buffer` now
- Extensive comments to all examples with explanations
- Transmuxing example showing how to use `typefind`, `multiqueue` and
  dynamic pads
- basic-tutorial-12 was ported and added

### Changed
- Rust 1.31 is the minimum supported Rust version now
- Update to latest gir code generator and glib bindings
- Functions returning e.g. `gst::FlowReturn` or other "combined" enums
  were changed to return split enums like `Result<gst::FlowSuccess,
  gst::FlowError>` to allow usage of the standard Rust error handling.
- Various functions and callbacks returning `bool` or `Option<_>` were
  changed to return a `Result<_, glib::BoolError>` or
  `Result<_, gst::LoggableError>` or `Result<_, gst::ErrorMessage>` for
  better integration with Rust's error handling infrastructure.
- Some infallible functions returning `bool` were changed to return `()`.
- `MiniObject` subclasses are now newtype wrappers around the
   underlying `GstRc<FooRef>` wrapper. This does not change the
   API in any breaking way for the current usages, but allows
   `MiniObject`s to also be implemented in other crates and
   makes sure `rustdoc` places the documentation in the right places.
- `BinExt` extension trait was renamed to `GstBinExt` to prevent
  conflicts with `gtk::Bin` if both are imported
- `Buffer::from_slice()` can't possible return `None`

### Fixed
- `gst::tag::Album` is the album tag now instead of artist sortname
- Return `0` for the channel mask corresponding to negative
  `AudioChannelPosition`s.
- `PartialOrd` and related traits are implemented via pointer equality on
  `ClockId` instead of using the compare function. Two clock ids with the same
  timestamp are not necessarily the same.
- Various functions that are actually fallible are now returning an
  `Option<_>`.
- Various `clippy` warnings

## [0.12.2] - 2018-11-26
### Fixed
- PTP clock constructor actually creates a PTP instead of NTP clock

### Added
- Bindings for GStreamer Editing Services
- Bindings for GStreamer Check testing library
- Bindings for the encoding profile API (encodebin)
- VideoFrame, VideoInfo, AudioInfo, StructureRef implements Send and Sync now
- VideoFrame has a function to get the raw FFI pointer
- From impls from the Error/Success enums to the combined enums like
  FlowReturn
- Bin-to-dot file functions were added to the Bin trait
- gst_base::Adapter implements SendUnique now

### Changed
- All references were updated from GitHub to freedesktop.org GitLab
- Fix various links in the README.md
- Link to the correct location for the documentation
- Remove GitLab badge as that only works with gitlab.com currently

## [0.12.1] - 2018-09-21
### Added
- More complete bindings for the gst_video::VideoOverlay interface, especially
  gst_video::is_video_overlay_prepare_window_handle_message()

## [0.12.0] - 2018-09-08
### Added
- Bindings for the GStreamer SDP and WebRTC libraries
- Generic API for working with tags that is based on string tag names and
  glib::Value for the tag values
- Bindings for Aggregator and AggregatorPad
- Bindings for BaseTransform/BaseSrc::get_buffer_pool()
- Optional serde implementations for the basic GStreamer data flow and metadata types

### Changed
- Use ptr::NonNull in various places
- Updated to muldiv 0.2, num-rational 0.2
- Bus::create_watch() can't return None
- Remove CallbackGuard as unwinding across FFI boundaries is not undefined
  behaviour anymore but will directly cause a panic
- Changed from the futures to the futures-preview crate as an optional
  dependency
- Various Caps operations take a &CapsRef instead of &Caps
- "deep-notify" signal takes the whole ParamSpec as parameter instead of only
  the signal name
- Some structs were changed from empty struct to empty enums
- Pad probe code does not take an additional reference to the data anymore,
  potentially passing writable events/buffers into the probe
- ValueExt::compare() is implemented around std::cmp::Ordering now instead of
  a custom enum that was basically the same

### Fixed
- Pad::add_probe() can return None if an IDLE probe was already called and
  removed in the meantime
- Various compiler and clippy warnings

### Removed
- std::Iterator impl for gst::Iterator. It was awkward to use because the
  gst::Iterator could fail at each iteration

## [0.11.6] - 2018-08-27
### Fixed
- Build with NLL/two-phase borrows
- Explicitly define [bin] section for discoverer example to fix a cargo
  warning

### Added
- Add unsafe gst::deinit() function
- Ord/PartialOrd impls on gst::Seqnum
- Getter for current pad mode
- gst::Pad::sticky_events_foreach() for iterating over all sticky events
  in a thread-safe way

## [0.11.5] - 2018-07-24
### Fixed
- `gst::Bus`'s sync handler must unref every message if
  `gst::BusSyncReply::Drop` is returned, otherwise they are all leaked

## [0.11.4] - 2018-07-19
### Fixed
- `gst::Caps::subtract()` does not leak its arguments anymore
- `gst::Caps::get_structure()` gracefully returns `None` if the index
  is out of bounds instead of a `g_return_val_if_fail()`
- `gst::Structure::new()` has to give away ownership of the info structure
  but didn't. For 0.11 we internally copy, in 0.12 it will take the info
  structure by value
- Typefind tests don't fail anymore if the system has typefind factories
  without caps

### Added
- An additional assertion that ensures that miniobjects are actually
  writable before creating a mutable reference

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

## 0.8.0 - 2017-08-31

- Initial release of the autogenerated GStreamer bindings. Older versions
  (< 0.8.0) of the bindings can be found [here](https://github.com/arturoc/gstreamer1.0-rs).
  The API of the two is incompatible.

[Unreleased]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.5...HEAD
[0.16.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.4...0.16.5
[0.16.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.3...0.16.4
[0.16.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.2...0.16.3
[0.16.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.1...0.16.2
[0.16.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.0...0.16.1
[0.16.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.7...0.16.0
[0.15.7]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.6...0.15.7
[0.15.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.5...0.15.6
[0.15.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.4...0.15.5
[0.15.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.3...0.15.4
[0.15.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.2...0.15.3
[0.15.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.1...0.15.2
[0.15.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.0...0.15.1
[0.15.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.14.2...0.15.0
[0.14.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.14.1...0.14.2
[0.14.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.14.0...0.14.1
[0.14.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.13.0...0.14.0
[0.13.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.12.2...0.13.0
[0.12.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.12.1...0.12.2
[0.12.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.12.0...0.12.1
[0.12.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.6...0.12.0
[0.11.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.5...0.11.6
[0.11.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.4...0.11.5
[0.11.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.3...0.11.4
[0.11.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.2...0.11.3
[0.11.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.1...0.11.2
[0.11.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.0...0.11.1
[0.11.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.10.2...0.11.0
[0.10.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.10.1...0.10.2
[0.10.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.10.0...0.10.1
[0.10.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.9.1...0.10.0
[0.9.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.9.0...0.9.1
[0.9.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.8.1...0.9.0
[0.8.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.8.1...0.8.2
[0.8.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.8.0...0.8.1
