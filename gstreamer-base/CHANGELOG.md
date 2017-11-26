# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
specifically the [variant used by Rust](http://doc.crates.io/manifest.html#the-version-field).

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

[Unreleased]: https://github.com/sdroege/gstreamer-rs/compare/0.8.1...HEAD
[0.8.1]: https://github.com/sdroege/gstreamer-rs/compare/0.8.0...0.8.1
