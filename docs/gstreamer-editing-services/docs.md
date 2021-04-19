<!-- file * -->
<!-- struct Asset -->
A `Asset` in the GStreamer Editing Services represents a resources
that can be used. In particular, any class that implements the
`Extractable` interface may have some associated assets with a
corresponding `Asset:extractable-type`, from which its objects can be
extracted using `AssetExt::extract`. Some examples would be
`Clip`, `Formatter` and `TrackElement`.

All assets that are created within GES are stored in a cache; one per
each `Asset:id` and `Asset:extractable-type` pair. These assets can
be fetched, and initialized if they do not yet exist in the cache,
using `Asset::request`.

``` c
GESAsset *effect_asset;
GESEffect *effect;

// You create an asset for an effect
effect_asset = ges_asset_request (GES_TYPE_EFFECT, "agingtv", NULL);

// And now you can extract an instance of GESEffect from that asset
effect = GES_EFFECT (ges_asset_extract (effect_asset));

```

The advantage of using assets, rather than simply creating the object
directly, is that the currently loaded resources can be listed with
`ges_list_assets` and displayed to an end user. For example, to show
which media files have been loaded, and a standard list of effects. In
fact, the GES library already creates assets for `TransitionClip` and
`Formatter`, which you can use to list all the available transition
types and supported formats.

The other advantage is that `Asset` implements `MetaContainer`, so
metadata can be set on the asset, with some subclasses automatically
creating this metadata on initiation.

For example, to display information about the supported formats, you
could do the following:

```text
   GList *formatter_assets, *tmp;

   //  List all  the transitions
   formatter_assets = ges_list_assets (GES_TYPE_FORMATTER);

   // Print some infos about the formatter GESAsset
   for (tmp = formatter_assets; tmp; tmp = tmp->next) {
     g_print ("Name of the formatter: %s, file extension it produces: %s",
       ges_meta_container_get_string (
         GES_META_CONTAINER (tmp->data), GES_META_FORMATTER_NAME),
       ges_meta_container_get_string (
         GES_META_CONTAINER (tmp->data), GES_META_FORMATTER_EXTENSION));
   }

   g_list_free (transition_assets);

```

## ID

Each asset is uniquely defined in the cache by its
`Asset:extractable-type` and `Asset:id`. Depending on the
`Asset:extractable-type`, the `Asset:id` can be used to parametrise
the creation of the object upon extraction. By default, a class that
implements `Extractable` will only have a single associated asset,
with an `Asset:id` set to the type name of its objects. However, this
is overwritten by some implementations, which allow a class to have
multiple associated assets. For example, for `TransitionClip` the
`Asset:id` will be a nickname of the `TransitionClip:vtype`. You
should check the documentation for each extractable type to see if they
differ from the default.

Moreover, each `Asset:extractable-type` may also associate itself
with a specific asset subclass. In such cases, when their asset is
requested, an asset of this subclass will be returned instead.

## Managing

You can use a `Project` to easily manage the assets of a
`Timeline`.

## Proxies

Some assets can (temporarily) act as the `Asset:proxy` of another
asset. When the original asset is requested from the cache, the proxy
will be returned in its place. This can be useful if, say, you want
to substitute a `UriClipAsset` corresponding to a high resolution
media file with the asset of a lower resolution stand in.

An asset may even have several proxies, the first of which will act as
its default and be returned on requests, but the others will be ordered
to take its place once it is removed. You can add a proxy to an asset,
or set its default, using `AssetExt::set_proxy`, and you can remove
them with `AssetExt::unproxy`.

# Implements

[`AssetExt`](trait@crate::AssetExt), [`trait@glib::object::ObjectExt`]
<!-- trait AssetExt -->
Trait containing all `Asset` methods.

# Implementors

[`Asset`](struct@crate::Asset), [`Project`](struct@crate::Project)
<!-- impl Asset::fn needs_reload -->
Indicate that an existing `Asset` in the cache should be reloaded
upon the next request. This can be used when some condition has
changed, which may require that an existing asset should be updated.
For example, if an external resource has changed or now become
available.

Note, the asset is not immediately changed, but will only actually
reload on the next call to `Asset::request` or
`Asset::request_async`.
## `extractable_type`
The `Asset:extractable-type` of the asset that
needs reloading
## `id`
The `Asset:id` of the asset asset that needs
reloading

# Returns

`true` if the specified asset exists in the cache and could be
marked for reloading.
<!-- impl Asset::fn request -->
Returns an asset with the given properties. If such an asset already
exists in the cache (it has been previously created in GES), then a
reference to the existing asset is returned. Otherwise, a newly created
asset is returned, and also added to the cache.

If the requested asset has been loaded with an error, then `error` is
set, if given, and `None` will be returned instead.

Note that the given `id` may not be exactly the `Asset:id` that is
set on the returned asset. For instance, it may be adjusted into a
standard format. Or, if a `Extractable` type does not have its
extraction parametrised, as is the case by default, then the given `id`
may be ignored entirely and the `Asset:id` set to some standard, in
which case a `None` `id` can be given.

Similarly, the given `extractable_type` may not be exactly the
`Asset:extractable-type` that is set on the returned asset. Instead,
the actual extractable type may correspond to a subclass of the given
`extractable_type`, depending on the given `id`.

Moreover, depending on the given `extractable_type`, the returned asset
may belong to a subclass of `Asset`.

Finally, if the requested asset has a `Asset:proxy`, then the proxy
that is found at the end of the chain of proxies is returned (a proxy's
proxy will take its place, and so on, unless it has no proxy).

Some asset subclasses only support asynchronous construction of its
assets, such as `UriClip`. For such assets this method will fail, and
you should use `Asset::request_async` instead. In the case of
`UriClip`, you can use `UriClipAsset::request_sync` if you only
want to wait for the request to finish.
## `extractable_type`
The `Asset:extractable-type` of the asset
## `id`
The `Asset:id` of the asset

# Returns

A reference to the requested
asset, or `None` if an error occurred.
<!-- impl Asset::fn request_async -->
Requests an asset with the given properties asynchronously (see
`Asset::request`). When the asset has been initialized or fetched
from the cache, the given callback function will be called. The
asset can then be retrieved in the callback using the
`Asset::request_finish` method on the given `gio::AsyncResult`.

Note that the source object passed to the callback will be the
`Asset` corresponding to the request, but it may not have loaded
correctly and therefore can not be used as is. Instead,
`Asset::request_finish` should be used to fetch a usable asset, or
indicate that an error occurred in the asset's creation.

Note that the callback will be called in the `glib::MainLoop` running under
the same `glib::MainContext` that `ges_init` was called in. So, if you wish
the callback to be invoked outside the default `glib::MainContext`, you can
call `glib::MainContext::push_thread_default` in a new thread before
calling `ges_init`.

Example of an asynchronous asset request:
``` c
// The request callback
static void
asset_loaded_cb (GESAsset * source, GAsyncResult * res, gpointer user_data)
{
  GESAsset *asset;
  GError *error = NULL;

  asset = ges_asset_request_finish (res, &error);
  if (asset) {
   g_print ("The file: %s is usable as a GESUriClip",
       ges_asset_get_id (asset));
  } else {
   g_print ("The file: %s is *not* usable as a GESUriClip because: %s",
       ges_asset_get_id (source), error->message);
  }

  gst_object_unref (asset);
}

// The request:
ges_asset_request_async (GES_TYPE_URI_CLIP, some_uri, NULL,
   (GAsyncReadyCallback) asset_loaded_cb, user_data);
```
## `extractable_type`
The `Asset:extractable-type` of the asset
## `id`
The `Asset:id` of the asset
## `cancellable`
An object to allow cancellation of the
asset request, or `None` to ignore
## `callback`
A function to call when the initialization is finished
## `user_data`
Data to be passed to `callback`
<!-- impl Asset::fn request_finish -->
Fetches an asset requested by `Asset::request_async`, which
finalises the request.
## `res`
The task result to fetch the asset from

# Returns

The requested asset, or `None` if an error
occurred.
<!-- trait AssetExt::fn extract -->
Extracts a new `Asset:extractable-type` object from the asset. The
`Asset:id` of the asset may determine the properties and state of the
newly created object.

# Returns

A newly created object, or `None` if an
error occurred.
<!-- trait AssetExt::fn error -->
Retrieve the error that was set on the asset when it was loaded.

# Returns

The error set on `asset`, or
`None` if no error occurred when `asset` was loaded.
<!-- trait AssetExt::fn extractable_type -->
Gets the `Asset:extractable-type` of the asset.

# Returns

The extractable type of `self`.
<!-- trait AssetExt::fn id -->
Gets the `Asset:id` of the asset.

# Returns

The ID of `self`.
<!-- trait AssetExt::fn proxy -->
Gets the default `Asset:proxy` of the asset.

# Returns

The default proxy of `self`.
<!-- trait AssetExt::fn proxy_target -->
Gets the `Asset:proxy-target` of the asset.

Note that the proxy target may have loaded with an error, so you should
call `AssetExt::get_error` on the returned target.

# Returns

The asset that `self` is a proxy
of.
<!-- trait AssetExt::fn list_proxies -->
Get all the proxies that the asset has. The first item of the list will
be the default `Asset:proxy`. The second will be the proxy that is
'next in line' to be default, and so on.

# Returns

The list of proxies
that `self` has.
<!-- trait AssetExt::fn set_proxy -->
Sets the `Asset:proxy` for the asset.

If `proxy` is among the existing proxies of the asset (see
`AssetExt::list_proxies`) it will be moved to become the default
proxy. Otherwise, if `proxy` is not `None`, it will be added to the list
of proxies, as the new default. The previous default proxy will become
'next in line' for if the new one is removed, and so on. As such, this
will **not** actually remove the previous default proxy (use
`AssetExt::unproxy` for that).

Note that an asset can only act as a proxy for one other asset.

As a special case, if `proxy` is `None`, then this method will actually
remove **all** proxies from the asset.
## `proxy`
A new default proxy for `self`

# Returns

`true` if `proxy` was successfully set as the default for
`self`.
<!-- trait AssetExt::fn unproxy -->
Removes the proxy from the available list of proxies for the asset. If
the given proxy is the default proxy of the list, then the next proxy
in the available list (see `AssetExt::list_proxies`) will become the
default. If there are no other proxies, then the asset will no longer
have a default `Asset:proxy`.
## `proxy`
An existing proxy of `self`

# Returns

`true` if `proxy` was successfully removed from `self`'s proxy
list.
<!-- trait AssetExt::fn get_property_extractable_type -->
The `Extractable` object type that can be extracted from the asset.
<!-- trait AssetExt::fn set_property_extractable_type -->
The `Extractable` object type that can be extracted from the asset.
<!-- trait AssetExt::fn get_property_id -->
The ID of the asset. This should be unique amongst all assets with
the same `Asset:extractable-type`. Depending on the associated
`Extractable` implementation, this id may convey some information
about the `glib::object::Object` that should be extracted. Note that, as such, the
ID will have an expected format, and you can not choose this value
arbitrarily. By default, this will be set to the type name of the
`Asset:extractable-type`, but you should check the documentation
of the extractable type to see whether they differ from the
default behaviour.
<!-- trait AssetExt::fn set_property_id -->
The ID of the asset. This should be unique amongst all assets with
the same `Asset:extractable-type`. Depending on the associated
`Extractable` implementation, this id may convey some information
about the `glib::object::Object` that should be extracted. Note that, as such, the
ID will have an expected format, and you can not choose this value
arbitrarily. By default, this will be set to the type name of the
`Asset:extractable-type`, but you should check the documentation
of the extractable type to see whether they differ from the
default behaviour.
<!-- trait AssetExt::fn get_property_proxy -->
The default proxy for this asset, or `None` if it has no proxy. A
proxy will act as a substitute for the original asset when the
original is requested (see `Asset::request`).

Setting this property will not usually remove the existing proxy, but
will replace it as the default (see `AssetExt::set_proxy`).
<!-- trait AssetExt::fn set_property_proxy -->
The default proxy for this asset, or `None` if it has no proxy. A
proxy will act as a substitute for the original asset when the
original is requested (see `Asset::request`).

Setting this property will not usually remove the existing proxy, but
will replace it as the default (see `AssetExt::set_proxy`).
<!-- trait AssetExt::fn get_property_proxy_target -->
The asset that this asset is a proxy for, or `None` if it is not a
proxy for another asset.

Note that even if this asset is acting as a proxy for another asset,
but this asset is not the default `Asset:proxy`, then `proxy`-target
will *still* point to this other asset. So you should check the
`Asset:proxy` property of `target`-proxy before assuming it is the
current default proxy for the target.

Note that the `glib::object::Object::notify` for this property is emitted after
the `Asset:proxy` `glib::object::Object::notify` for the corresponding (if any)
asset it is now the proxy of/no longer the proxy of.
<!-- struct BaseEffect -->
A `BaseEffect` is some operation that applies an effect to the data
it receives.

## Time Effects

Some operations will change the timing of the stream data they receive
in some way. In particular, the `gst::Element` that they wrap could alter
the times of the segment they receive in a `gst::EventType::Segment` event,
or the times of a seek they receive in a `gst::EventType::Seek` event. Such
operations would be considered time effects since they translate the
times they receive on their source to different times at their sink,
and vis versa. This introduces two sets of time coordinates for the
event: (internal) sink coordinates and (internal) source coordinates,
where segment times are translated from the sink coordinates to the
source coordinates, and seek times are translated from the source
coordinates to the sink coordinates.

If you use such an effect in GES, you will need to inform GES of the
properties that control the timing with
`BaseEffectExt::register_time_property`, and the effect's timing
behaviour using `BaseEffectExt::set_time_translation_funcs`.

Note that a time effect should not have its
`TrackElement:has-internal-source` set to `true`.

In addition, note that GES only *fully* supports time effects whose
mapping from the source to sink coordinates (those applied to seeks)
obeys:

+ Maps the time `0` to `0`. So initial time-shifting effects are
 excluded.
+ Is monotonically increasing. So reversing effects, and effects that
 jump backwards in the stream are excluded.
+ Can handle a reasonable `gst::ClockTime`, relative to the project. So
 this would exclude a time effect with an extremely large speed-up
 that would cause the converted `gst::ClockTime` seeks to overflow.
+ Is 'continuously reversible'. This essentially means that for every
 time in the sink coordinates, we can, to 'good enough' accuracy,
 calculate the corresponding time in the source coordinates. Moreover,
 this should correspond to how segment times are translated from
 sink to source.
+ Only depends on the registered time properties, rather than the
 state of the `gst::Element` or the data it receives. This would exclude,
 say, an effect that would speedup if there is more red in the image
 it receives.

Note that a constant-rate-change effect that is not extremely fast or
slow would satisfy these conditions. For such effects, you may wish to
use `EffectClass::register_rate_property`.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`BaseEffectExt`](trait@crate::BaseEffectExt), [`TrackElementExt`](trait@crate::TrackElementExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait BaseEffectExt -->
Trait containing all `BaseEffect` methods.

# Implementors

[`BaseEffect`](struct@crate::BaseEffect), [`Effect`](struct@crate::Effect)
<!-- trait BaseEffectExt::fn is_time_effect -->
Get whether the effect is considered a time effect or not. An effect
with registered time properties or set translation functions is
considered a time effect.

Feature: `v1_18`


# Returns

`true` if `self` is considered a time effect.
<!-- trait BaseEffectExt::fn register_time_property -->
Register a child property of the effect as a property that, when set,
can change the timing of its input data. The child property should be
specified as in `TimelineElementExt::lookup_child`.

You should also set the corresponding time translation using
`BaseEffectExt::set_time_translation_funcs`.

Note that `self` must not be part of a clip, nor can it have
`TrackElement:has-internal-source` set to `true`.

Feature: `v1_18`

## `child_property_name`
The name of the child property to register as
a time property

# Returns

`true` if the child property was found and newly registered.
<!-- trait BaseEffectExt::fn set_time_translation_funcs -->
Set the time translation query functions for the time effect. If an
effect is a time effect, it will have two sets of coordinates: one
at its sink and one at its source. The given functions should be able
to translate between these two sets of coordinates. More specifically,
`source_to_sink_func` should *emulate* how the corresponding `gst::Element`
would translate the `gst::Segment` `time` field, and `sink_to_source_func`
should emulate how the corresponding `gst::Element` would translate the
seek query `start` and `stop` values, as used in `gst::ElementExt::seek`. As
such, `sink_to_source_func` should act as an approximate reverse of
`source_to_sink_func`.

Note, these functions will be passed a table of time properties, as
registered in `BaseEffectExt::register_time_property`, and their
values. The functions should emulate what the translation *would* be
*if* the time properties were set to the given values. They should not
use the currently set values.

Note that `self` must not be part of a clip, nor can it have
`TrackElement:has-internal-source` set to `true`.

Feature: `v1_18`

## `source_to_sink_func`
The function to use
for querying how a time is translated from the source coordinates to
the sink coordinates of `self`
## `sink_to_source_func`
The function to use
for querying how a time is translated from the sink coordinates to the
source coordinates of `self`
## `user_data`
Data to pass to both `source_to_sink_func` and
`sink_to_source_func`
## `destroy`
Method to call to destroy
`user_data`, or `None`

# Returns

`true` if the translation functions were set.
<!-- struct BaseTransitionClip -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`OperationClipExt`](trait@crate::OperationClipExt), [`ClipExt`](trait@crate::ClipExt), [`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- struct Clip -->
`Clip`-s are the core objects of a `Layer`. Each clip may exist in
a single layer but may control several `TrackElement`-s that span
several `Track`-s. A clip will ensure that all its children share the
same `TimelineElement:start` and `TimelineElement:duration` in
their tracks, which will match the `TimelineElement:start` and
`TimelineElement:duration` of the clip itself. Therefore, changing
the timing of the clip will change the timing of the children, and a
change in the timing of a child will change the timing of the clip and
subsequently all its siblings. As such, a clip can be treated as a
singular object in its layer.

For most uses of a `Timeline`, it is often sufficient to only
interact with `Clip`-s directly, which will take care of creating and
organising the elements of the timeline's tracks.

## Core Children

In more detail, clips will usually have some *core* `TrackElement`
children, which are created by the clip when it is added to a layer in
a timeline. The type and form of these core children will depend on the
clip's subclass. You can use `TrackElementExt::is_core` to determine
whether a track element is considered such a core track element. Note,
if a core track element is part of a clip, it will always be treated as
a core *child* of the clip. You can connect to the
`Container::child-added` signal to be notified of their creation.

When a child is added to a clip, the timeline will select its tracks
using `Timeline::select-tracks-for-object`. Note that it may be the
case that the child will still have no set `TrackElement:track`
after this process. For example, if the timeline does not have a track
of the corresponding `Track:track-type`. A clip can safely contain
such children, which may have their track set later, although they will
play no functioning role in the timeline in the meantime.

If a clip may create track elements with various
`TrackElement:track-type`(s), such as a `UriClip`, but you only
want it to create a subset of these types, you should set the
`Clip:supported-formats` of the clip to the subset of types. This
should be done *before* adding the clip to a layer.

If a clip will produce several core elements of the same
`TrackElement:track-type`, you should connect to the timeline's
`Timeline::select-tracks-for-object` signal to coordinate which
tracks each element should land in. Note, no two core children within a
clip can share the same `Track`, so you should not select the same
track for two separate core children. Provided you stick to this rule,
it is still safe to select several tracks for the same core child, the
core child will be copied into the additional tracks. You can manually
add the child to more tracks later using `ClipExt::add_child_to_track`.
If you do not wish to use a core child, you can always select no track.

The `TimelineElement:in-point` of the clip will control the
`TimelineElement:in-point` of its core children to be the same
value if their `TrackElement:has-internal-source` is set to `true`.

The `TimelineElement:max-duration` of the clip is the minimum
`TimelineElement:max-duration` of its core children. If you set its
value to anything other than its current value, this will also set the
`TimelineElement:max-duration` of all its core children to the same
value if their `TrackElement:has-internal-source` is set to `true`.
As a special case, whilst a clip does not yet have any core children,
its `TimelineElement:max-duration` may be set to indicate what its
value will be once they are created.

## Effects

Some subclasses (`SourceClip` and `BaseEffectClip`) may also allow
their objects to have additional non-core `BaseEffect`-s elements as
children. These are additional effects that are applied to the output
data of the core elements. They can be added to the clip using
`ClipExt::add_top_effect`, which will take care of adding the effect to
the timeline's tracks. The new effect will be placed between the clip's
core track elements and its other effects. As such, the newly added
effect will be applied to any source data **before** the other existing
effects. You can change the ordering of effects using
`ClipExt::set_top_effect_index`.

Tracks are selected for top effects in the same way as core children.
If you add a top effect to a clip before it is part of a timeline, and
later add the clip to a timeline, the track selection for the top
effects will occur just after the track selection for the core
children. If you add a top effect to a clip that is already part of a
timeline, the track selection will occur immediately. Since a top
effect must be applied on top of a core child, if you use
`Timeline::select-tracks-for-object`, you should ensure that the
added effects are destined for a `Track` that already contains a core
child.

In addition, if the core child in the track is not
`TrackElement:active`, then neither can any of its effects be
`TrackElement:active`. Therefore, if a core child is made in-active,
all of the additional effects in the same track will also become
in-active. Similarly, if an effect is set to be active, then the core
child will also become active, but other effects will be left alone.
Finally, if an active effect is added to the track of an in-active core
child, it will become in-active as well. Note, in contrast, setting a
core child to be active, or an effect to be in-active will *not* change
the other children in the same track.

### Time Effects

Some effects also change the timing of their data (see `BaseEffect`
for what counts as a time effect). Note that a `BaseEffectClip` will
refuse time effects, but a `Source` will allow them.

When added to a clip, time effects may adjust the timing of other
children in the same track. Similarly, when changing the order of
effects, making them (in)-active, setting their time property values
or removing time effects. These can cause the `Clip:duration-limit`
to change in value. However, if such an operation would ever cause the
`TimelineElement:duration` to shrink such that a clip's `Source` is
totally overlapped in the timeline, the operation would be prevented.
Note that the same can happen when adding non-time effects with a
finite `TimelineElement:max-duration`.

Therefore, when working with time effects, you should -- more so than
usual -- not assume that setting the properties of the clip's children
will succeed. In particular, you should use
`TimelineElementExt::set_child_property_full` when setting the time
properties.

If you wish to preserve the *internal* duration of a source in a clip
during these time effect operations, you can do something like the
following.

```c
void
do_time_effect_change (GESClip * clip)
{
  GList *tmp, *children;
  GESTrackElement *source;
  GstClockTime source_outpoint;
  GstClockTime new_end;
  GError *error = NULL;

  // choose some active source in a track to preserve the internal
  // duration of
  source = ges_clip_get_track_element (clip, NULL, GES_TYPE_SOURCE);

  // note its current internal end time
  source_outpoint = ges_clip_get_internal_time_from_timeline_time (
        clip, source, GES_TIMELINE_ELEMENT_END (clip), NULL);

  // handle invalid out-point

  // stop the children's control sources from clamping when their
  // out-point changes with a change in the time effects
  children = ges_container_get_children (GES_CONTAINER (clip), FALSE);

  for (tmp = children; tmp; tmp = tmp->next)
    ges_track_element_set_auto_clamp_control_source (tmp->data, FALSE);

  // add time effect, or set their children properties, or move them around
  ...
  // user can make sure that if a time effect changes one source, we should
  // also change the time effect for another source. E.g. if
  // "GstVideorate::rate" is set to 2.0, we also set "GstPitch::rate" to
  // 2.0

  // Note the duration of the clip may have already changed if the
  // duration-limit of the clip dropped below its current value

  new_end = ges_clip_get_timeline_time_from_internal_time (
        clip, source, source_outpoint, &error);
  // handle error

  if (!ges_timeline_elemnet_edit_full (GES_TIMELINE_ELEMENT (clip),
        -1, GES_EDIT_MODE_TRIM, GES_EDGE_END, new_end, &error))
    // handle error

  for (tmp = children; tmp; tmp = tmp->next)
    ges_track_element_set_auto_clamp_control_source (tmp->data, TRUE);

  g_list_free_full (children, gst_object_unref);
  gst_object_unref (source);
}
```

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ClipExt`](trait@crate::ClipExt), [`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait ClipExt -->
Trait containing all `Clip` methods.

# Implementors

[`Clip`](struct@crate::Clip), [`OperationClip`](struct@crate::OperationClip)
<!-- trait ClipExt::fn add_asset -->
Extracts a `TrackElement` from an asset and adds it to the clip.
This can be used to add effects that derive from the asset to the
clip, but this method is not intended to be used to create the core
elements of the clip.
## `asset`
An asset with `GES_TYPE_TRACK_ELEMENT` as its
`Asset:extractable-type`

# Returns

The newly created element, or
`None` if an error occurred.
<!-- trait ClipExt::fn add_child_to_track -->
Adds the track element child of the clip to a specific track.

If the given child is already in another track, this will create a copy
of the child, add it to the clip, and add this copy to the track.

You should only call this whilst a clip is part of a `Timeline`, and
for tracks that are in the same timeline.

This method is an alternative to using the
`Timeline::select-tracks-for-object` signal, but can be used to
complement it when, say, you wish to copy a clip's children from one
track into a new one.

When the child is a core child, it must be added to a track that does
not already contain another core child of the same clip. If it is not a
core child (an additional effect), then it must be added to a track
that already contains one of the core children of the same clip.

This method can also fail if the adding the track element to the track
would break a configuration rule of the corresponding `Timeline`,
such as causing three sources to overlap at a single time, or causing
a source to completely overlap another in the same track.

Feature: `v1_18`

## `child`
A child of `self`
## `track`
The track to add `child` to

# Returns

The element that was added to `track`, either
`child` or a copy of child, or `None` if the element could not be added.
<!-- trait ClipExt::fn add_top_effect -->
Add a top effect to a clip at the given index.

Unlike using `GESContainerExt::add`, this allows you to set the index
in advance. It will also check that no error occurred during the track
selection for the effect.

Note, only subclasses of `ClipClass` that have
`GES_CLIP_CLASS_CAN_ADD_EFFECTS` set to `true` (such as `SourceClip`
and `BaseEffectClip`) can have additional top effects added.

Note, if the effect is a time effect, this may be refused if the clip
would not be able to adapt itself once the effect is added.

Feature: `v1_18`

## `effect`
A top effect to add
## `index`
The index to add `effect` at, or -1 to add at the highest

# Returns

`true` if `effect` was successfully added to `self` at `index`.
<!-- trait ClipExt::fn find_track_element -->
Finds an element controlled by the clip. If `track` is given,
then only the track elements in `track` are searched for. If `type_` is
given, then this function searches for a track element of the given
`type_`.

Note, if multiple track elements in the clip match the given criteria,
this will return the element amongst them with the highest
`TimelineElement:priority` (numerically, the smallest). See
`ClipExt::find_track_elements` if you wish to find all such elements.
## `track`
The track to search in, or `None` to search in
all tracks
## `type_`
The type of track element to search for, or `G_TYPE_NONE` to
match any type

# Returns

The element controlled by
`self`, in `track`, and of the given `type_`, or `None` if no such element
could be found.
<!-- trait ClipExt::fn find_track_elements -->
Finds the `TrackElement`-s controlled by the clip that match the
given criteria. If `track` is given as `None` and `track_type` is given as
`TrackType::Unknown`, then the search will match all elements in any
track, including those with no track, and of any
`TrackElement:track-type`. Otherwise, if `track` is not `None`, but
`track_type` is `TrackType::Unknown`, then only the track elements in
`track` are searched for. Otherwise, if `track_type` is not
`TrackType::Unknown`, but `track` is `None`, then only the track
elements whose `TrackElement:track-type` matches `track_type` are
searched for. Otherwise, when both are given, the track elements that
match **either** criteria are searched for. Therefore, if you wish to
only find elements in a specific track, you should give the track as
`track`, but you should not give the track's `Track:track-type` as
`track_type` because this would also select elements from other tracks
of the same type.

You may also give `type_` to _further_ restrict the search to track
elements of the given `type_`.
## `track`
The track to search in, or `None` to search in
all tracks
## `track_type`
The track-type of the track element to search for, or
`TrackType::Unknown` to match any track type
## `type_`
The type of track element to search for, or `G_TYPE_NONE` to
match any type

# Returns

A list of all
the `TrackElement`-s controlled by `self`, in `track` or of the given
`track_type`, and of the given `type_`.
<!-- trait ClipExt::fn duration_limit -->
Gets the `Clip:duration-limit` of the clip.

Feature: `v1_18`


# Returns

The duration-limit of `self`.
<!-- trait ClipExt::fn get_internal_time_from_timeline_time -->
Convert the timeline time to an internal source time of the child.
This will take any time effects placed on the clip into account (see
`BaseEffect` for what time effects are supported, and how to
declare them in GES).

When `timeline_time` is above the `TimelineElement:start` of `self`,
this will return the internal time at which the content that appears at
`timeline_time` in the output of the timeline is created in `child`. For
example, if `timeline_time` corresponds to the current seek position,
this would let you know which part of a media file is being read.

This will be done assuming the clip has an indefinite end, so the
internal time may be beyond the current out-point of the child, or even
its `TimelineElement:max-duration`.

If, instead, `timeline_time` is below the current
`TimelineElement:start` of `self`, this will return what you would
need to set the `TimelineElement:in-point` of `child` to if you set
the `TimelineElement:start` of `self` to `timeline_time` and wanted
to keep the content of `child` currently found at the current
`TimelineElement:start` of `self` at the same timeline position. If
this would be negative, the conversion fails. This is useful for
determining what `TimelineElement:in-point` would result from a
`EditMode::Trim` to `timeline_time`.

Note that whilst a clip has no time effects, this second return is
equivalent to finding the internal time at which the content that
appears at `timeline_time` in the timeline can be found in `child` if it
had indefinite extent in both directions. However, with non-linear time
effects this second return will be more distinct.

In either case, the returned time would be appropriate to use for the
`TimelineElement:in-point` or `TimelineElement:max-duration` of the
child.

See `ClipExt::get_timeline_time_from_internal_time`, which performs the
reverse.

Feature: `v1_18`

## `child`
An `TrackElement:active` child of `self` with a
`TrackElement:track`
## `timeline_time`
A time in the timeline time coordinates

# Returns

The time in the internal coordinates of `child` corresponding
to `timeline_time`, or `GST_CLOCK_TIME_NONE` if the conversion could not
be performed.
<!-- trait ClipExt::fn layer -->
Gets the `Clip:layer` of the clip.

# Returns

The layer `self` is in, or `None` if
`self` is not in any layer.
<!-- trait ClipExt::fn supported_formats -->
Gets the `Clip:supported-formats` of the clip.

# Returns

The `TrackType`-s supported by `self`.
<!-- trait ClipExt::fn get_timeline_time_from_internal_time -->
Convert the internal source time from the child to a timeline time.
This will take any time effects placed on the clip into account (see
`BaseEffect` for what time effects are supported, and how to
declare them in GES).

When `internal_time` is above the `TimelineElement:in-point` of
`child`, this will return the timeline time at which the internal
content found at `internal_time` appears in the output of the timeline's
track. For example, this would let you know where in the timeline a
particular scene in a media file would appear.

This will be done assuming the clip has an indefinite end, so the
timeline time may be beyond the end of the clip, or even breaking its
`Clip:duration-limit`.

If, instead, `internal_time` is below the current
`TimelineElement:in-point` of `child`, this will return what you would
need to set the `TimelineElement:start` of `self` to if you set the
`TimelineElement:in-point` of `child` to `internal_time` and wanted to
keep the content of `child` currently found at the current
`TimelineElement:start` of `self` at the same timeline position. If
this would be negative, the conversion fails. This is useful for
determining what position to use in a `EditMode::Trim` if you wish
to trim to a specific point in the internal content, such as a
particular scene in a media file.

Note that whilst a clip has no time effects, this second return is
equivalent to finding the timeline time at which the content of `child`
at `internal_time` would be found in the timeline if it had indefinite
extent in both directions. However, with non-linear time effects this
second return will be more distinct.

In either case, the returned time would be appropriate to use in
`TimelineElementExt::edit` for `EditMode::Trim`, and similar, if
you wish to use a particular internal point as a reference. For
example, you could choose to end a clip at a certain internal
'out-point', similar to the `TimelineElement:in-point`, by
translating the desired end time into the timeline coordinates, and
using this position to trim the end of a clip.

See `ClipExt::get_internal_time_from_timeline_time`, which performs the
reverse, or `ClipExt::get_timeline_time_from_source_frame` which does
the same conversion, but using frame numbers.

Feature: `v1_18`

## `child`
An `TrackElement:active` child of `self` with a
`TrackElement:track`
## `internal_time`
A time in the internal time coordinates of `child`

# Returns

The time in the timeline coordinates corresponding to
`internal_time`, or `GST_CLOCK_TIME_NONE` if the conversion could not be
performed.
<!-- trait ClipExt::fn get_timeline_time_from_source_frame -->
Convert the source frame number to a timeline time. This acts the same
as `ClipExt::get_timeline_time_from_internal_time` using the core
children of the clip and using the frame number to specify the internal
position, rather than a timestamp.

The returned timeline time can be used to seek or edit to a specific
frame.

Note that you can get the frame timestamp of a particular clip asset
with `ClipAsset::get_frame_time`.

Feature: `v1_18`

## `frame_number`
The frame number to get the corresponding timestamp of
in the timeline coordinates

# Returns

The timestamp corresponding to `frame_number` in the core
children of `self`, in the timeline coordinates, or `GST_CLOCK_TIME_NONE`
if the conversion could not be performed.
<!-- trait ClipExt::fn get_top_effect_index -->
Gets the internal index of an effect in the clip. The index of effects
in a clip will run from 0 to n-1, where n is the total number of
effects. If two effects share the same `TrackElement:track`, the
effect with the numerically lower index will be applied to the source
data **after** the other effect, i.e. output data will always flow from
a higher index effect to a lower index effect.
## `effect`
The effect we want to get the index of

# Returns

The index of `effect` in `self`, or -1 if something went wrong.
<!-- trait ClipExt::fn top_effects -->
Gets the `BaseEffect`-s that have been added to the clip. The
returned list is ordered by their internal index in the clip. See
`ClipExt::get_top_effect_index`.

# Returns

A list of all
`BaseEffect`-s that have been added to `self`.
<!-- trait ClipExt::fn move_to_layer -->
See `ClipExt::move_to_layer_full`, which also gives an error.
## `layer`
The new layer

# Returns

`true` if `self` was successfully moved to `layer`.
<!-- trait ClipExt::fn move_to_layer_full -->
Moves a clip to a new layer. If the clip already exists in a layer, it
is first removed from its current layer before being added to the new
layer.

Feature: `v1_18`

## `layer`
The new layer

# Returns

`true` if `self` was successfully moved to `layer`.
<!-- trait ClipExt::fn remove_top_effect -->
Remove a top effect from the clip.

Note, if the effect is a time effect, this may be refused if the clip
would not be able to adapt itself once the effect is removed.

Feature: `v1_18`

## `effect`
The top effect to remove

# Returns

`true` if `effect` was successfully added to `self` at `index`.
<!-- trait ClipExt::fn set_supported_formats -->
Sets the `Clip:supported-formats` of the clip. This should normally
only be called by subclasses, which should be responsible for updating
its value, rather than the user.
## `supportedformats`
The `TrackType`-s supported by `self`
<!-- trait ClipExt::fn set_top_effect_index -->
See `ClipExt::set_top_effect_index_full`, which also gives an error.
## `effect`
An effect within `self` to move
## `newindex`
The index for `effect` in `self`

# Returns

`true` if `effect` was successfully moved to `newindex`.
<!-- trait ClipExt::fn set_top_effect_index_full -->
Set the index of an effect within the clip. See
`ClipExt::get_top_effect_index`. The new index must be an existing
index of the clip. The effect is moved to the new index, and the other
effects may be shifted in index accordingly to otherwise maintain the
ordering.

Feature: `v1_18`

## `effect`
An effect within `self` to move
## `newindex`
The index for `effect` in `self`

# Returns

`true` if `effect` was successfully moved to `newindex`.
<!-- trait ClipExt::fn split -->
See `ClipExt::split_full`, which also gives an error.
## `position`
The timeline position at which to perform the split

# Returns

The newly created clip resulting
from the splitting `self`, or `None` if `self` can't be split.
<!-- trait ClipExt::fn split_full -->
Splits a clip at the given timeline position into two clips. The clip
must already have a `Clip:layer`.

The original clip's `TimelineElement:duration` is reduced such that
its end point matches the split position. Then a new clip is created in
the same layer, whose `TimelineElement:start` matches the split
position and `TimelineElement:duration` will be set such that its end
point matches the old end point of the original clip. Thus, the two
clips together will occupy the same positions in the timeline as the
original clip did.

The children of the new clip will be new copies of the original clip's
children, so it will share the same sources and use the same
operations.

The new clip will also have its `TimelineElement:in-point` set so
that any internal data will appear in the timeline at the same time.
Thus, when the timeline is played, the playback of data should
appear the same. This may be complicated by any additional
`Effect`-s that have been placed on the original clip that depend on
the playback time or change the data consumption rate of sources. This
method will attempt to translate these effects such that the playback
appears the same. In such complex situations, you may get a better
result if you place the clip in a separate sub `Project`, which only
contains this clip (and its effects), and in the original layer
create two neighbouring `UriClip`-s that reference this sub-project,
but at a different `TimelineElement:in-point`.

Feature: `v1_18`

## `position`
The timeline position at which to perform the split, between
the start and end of the clip

# Returns

The newly created clip resulting
from the splitting `self`, or `None` if `self` can't be split.
<!-- trait ClipExt::fn get_property_duration_limit -->
The maximum `TimelineElement:duration` that can be *currently* set
for the clip, taking into account the `TimelineElement:in-point`,
`TimelineElement:max-duration`, `TrackElement:active`, and
`TrackElement:track` properties of its children, as well as any
time effects. If there is no limit, this will be set to
`GST_CLOCK_TIME_NONE`.

Note that whilst a clip has no children in any tracks, the limit will
be unknown, and similarly set to `GST_CLOCK_TIME_NONE`.

If the duration-limit would ever go below the current
`TimelineElement:duration` of the clip due to a change in the above
variables, its `TimelineElement:duration` will be set to the new
limit.

Feature: `v1_18`

<!-- trait ClipExt::fn get_property_layer -->
The layer this clip lies in.

If you want to connect to this property's `glib::object::Object::notify` signal,
you should connect to it with `g_signal_connect_after` since the
signal emission may be stopped internally.
<!-- trait ClipExt::fn get_property_supported_formats -->
The `TrackType`-s that the clip supports, which it can create
`TrackElement`-s for. Note that this can be a combination of
`TrackType` flags to indicate support for several
`TrackElement:track-type` elements.
<!-- trait ClipExt::fn set_property_supported_formats -->
The `TrackType`-s that the clip supports, which it can create
`TrackElement`-s for. Note that this can be a combination of
`TrackType` flags to indicate support for several
`TrackElement:track-type` elements.
<!-- struct Container -->
A `Container` is a timeline element that controls other
`TimelineElement`-s, which are its children. In particular, it is
responsible for maintaining the relative `TimelineElement:start` and
`TimelineElement:duration` times of its children. Therefore, if a
container is temporally adjusted or moved to a new layer, it may
accordingly adjust and move its children. Similarly, a change in one of
its children may prompt the parent to correspondingly change its
siblings.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait GESContainerExt -->
Trait containing all `Container` methods.

# Implementors

[`Clip`](struct@crate::Clip), [`Container`](struct@crate::Container), [`Group`](struct@crate::Group)
<!-- impl Container::fn group -->
Groups the containers into a single container by merging them. The
containers must all belong to the same `TimelineElement:timeline`.

If the elements are all `Clip`-s then this method will attempt to
combine them all into a single `Clip`. This should succeed if they:
share the same `TimelineElement:start`, `TimelineElement:duration`
and `TimelineElement:in-point`; exist in the same layer; and all of
the sources share the same `Asset`. If this fails, or one of the
elements is not a `Clip`, this method will try to create a `Group`
instead.
## `containers`

The `Container`-s to group

# Returns

The container created by merging
`containers`, or `None` if they could not be merged into a single
container.
<!-- trait GESContainerExt::fn add -->
Adds a timeline element to the container. The element will now be a
child of the container (and the container will be the
`TimelineElement:parent` of the added element), which means that it
is now controlled by the container. This may change the properties of
the child or the container, depending on the subclass.

Additionally, the children properties of the newly added element will
be shared with the container, meaning they can also be read and set
using `TimelineElementExt::get_child_property` and
`TimelineElementExt::set_child_property` on the container.
## `child`
The element to add as a child

# Returns

`true` if `child` was successfully added to `self`.
<!-- trait GESContainerExt::fn edit -->
Edits the container within its timeline.

# Deprecated since 1.18

use `TimelineElementExt::edit` instead.
## `layers`
A whitelist of layers
where the edit can be performed, `None` allows all layers in the
timeline
## `new_layer_priority`
The priority/index of the layer `self` should
be moved to. -1 means no move
## `mode`
The edit mode
## `edge`
The edge of `self` where the edit should occur
## `position`
The edit position: a new location for the edge of `self`
(in nanoseconds)

# Returns

`true` if the edit of `self` completed, `false` on failure.
<!-- trait GESContainerExt::fn get_children -->
Get the list of timeline elements contained in the container. If
`recursive` is `true`, and the container contains other containers as
children, then their children will be added to the list, in addition to
themselves, and so on.
## `recursive`
Whether to recursively get children in `self`

# Returns

The list of
`TimelineElement`-s contained in `self`.
<!-- trait GESContainerExt::fn remove -->
Removes a timeline element from the container. The element will no
longer be controlled by the container.
## `child`
The child to remove

# Returns

`true` if `child` was successfully removed from `self`.
<!-- trait GESContainerExt::fn ungroup -->
Ungroups the container by splitting it into several containers
containing various children of the original. The rules for how the
container splits depends on the subclass. A `Group` will simply split
into its children. A `Clip` will split into one `Clip` per
`TrackType` it overlaps with (so an audio-video clip will split into
an audio clip and a video clip), where each clip contains all the
`TrackElement`-s from the original clip with a matching
`TrackElement:track-type`.

If `recursive` is `true`, and the container contains other containers as
children, then they will also be ungrouped, and so on.
## `recursive`
Whether to recursively ungroup `self`

# Returns

The list of
new `Container`-s created from the splitting of `self`.
<!-- trait GESContainerExt::fn connect_child_added -->
Will be emitted after a child is added to the container. Usually,
you should connect with `g_signal_connect_after` since the signal
may be stopped internally.
## `element`
The child that was added
<!-- trait GESContainerExt::fn connect_child_removed -->
Will be emitted after a child is removed from the container.
## `element`
The child that was removed
<!-- trait GESContainerExt::fn get_property_height -->
The span of the container's children's `TimelineElement:priority`
values, which is the number of integers that lie between (inclusive)
the minimum and maximum priorities found amongst the container's
children (maximum - minimum + 1).
<!-- enum Edge -->
The edges of an object contain in a `Timeline` or `Track`
<!-- enum Edge::variant Start -->
Represents the start of an object.
<!-- enum Edge::variant End -->
Represents the end of an object.
<!-- enum Edge::variant None -->
Represent the fact we are not working with any edge of an
 object.
<!-- enum EditMode -->
When a single timeline element is edited within its timeline at some
position, using `TimelineElementExt::edit`, depending on the edit
mode, its `TimelineElement:start`, `TimelineElement:duration` or
`TimelineElement:in-point` will be adjusted accordingly. In addition,
any clips may change `Clip:layer`.

Each edit can be broken down into a combination of three basic edits:

+ MOVE: This moves the start of the element to the edit position.
+ START-TRIM: This cuts or grows the start of the element, whilst
 maintaining the time at which its internal content appears in the
 timeline data output. If the element is made shorter, the data that
 appeared at the edit position will still appear in the timeline at
 the same time. If the element is made longer, the data that appeared
 at the previous start of the element will still appear in the
 timeline at the same time.
+ END-TRIM: Similar to START-TRIM, but the end of the element is cut or
 grown.

In particular, when editing a `Clip`:

+ MOVE: This will set the `TimelineElement:start` of the clip to the
 edit position.
+ START-TRIM: This will set the `TimelineElement:start` of the clip
 to the edit position. To keep the end time the same, the
 `TimelineElement:duration` of the clip will be adjusted in the
 opposite direction. In addition, the `TimelineElement:in-point` of
 the clip will be shifted such that the content that appeared at the
 new or previous start time, whichever is latest, still appears at the
 same timeline time. For example, if a frame appeared at the start of
 the clip, and the start of the clip is reduced, the in-point of the
 clip will also reduce such that the frame will appear later within
 the clip, but at the same timeline position.
+ END-TRIM: This will set the `TimelineElement:duration` of the clip
 such that its end time will match the edit position.

When editing a `Group`:

+ MOVE: This will set the `Group:start` of the clip to the edit
 position by shifting all of its children by the same amount. So each
 child will maintain their relative positions.
+ START-TRIM: If the group is made shorter, this will START-TRIM any
 clips under the group that start after the edit position to the same
 edit position. If the group is made longer, this will START-TRIM any
 clip under the group whose start matches the start of the group to
 the same edit position.
+ END-TRIM: If the group is made shorter, this will END-TRIM any clips
 under the group that end after the edit position to the same edit
 position. If the group is made longer, this will END-TRIM any clip
 under the group whose end matches the end of the group to the same
 edit position.

When editing a `TrackElement`, if it has a `Clip` parent, this
will be edited instead. Otherwise it is edited in the same way as a
`Clip`.

The layer priority of a `Group` is the lowest layer priority of any
`Clip` underneath it. When a group is edited to a new layer
priority, it will shift all clips underneath it by the same amount,
such that their relative layers stay the same.

If the `Timeline` has a `Timeline:snapping-distance`, then snapping
may occur for some of the edges of the **main** edited element:

+ MOVE: The start or end edge of *any* `Source` under the element may
 be snapped.
+ START-TRIM: The start edge of a `Source` whose start edge touches
 the start edge of the element may snap.
+ END-TRIM: The end edge of a `Source` whose end edge touches the end
 edge of the element may snap.

These edges may snap with either the start or end edge of *any* other
`Source` in the timeline that is not also being moved by the element,
including those in different layers, if they are within the
`Timeline:snapping-distance`. During an edit, only up to one snap can
occur. This will shift the edit position such that the snapped edges
will touch once the edit has completed.

Note that snapping can cause an edit to fail where it would have
otherwise succeeded because it may push the edit position such that the
edit would result in an unsupported timeline configuration. Similarly,
snapping can cause an edit to succeed where it would have otherwise
failed.

For example, in `EditMode::Ripple` acting on `Edge::None`, the
main element is the MOVED toplevel of the edited element. Any source
under the main MOVED toplevel may have its start or end edge snapped.
Note, these sources cannot snap with each other. The edit may also
push other elements, but any sources under these elements cannot snap,
nor can they be snapped with. If a snap does occur, the MOVE of the
toplevel *and* all other elements pushed by the ripple will be shifted
by the same amount such that the snapped edges will touch.

You can also find more explanation about the behaviour of those modes at:
[trim, ripple and roll](http://pitivi.org/manual/trimming.html)
and [clip management](http://pitivi.org/manual/usingclips.html).
<!-- enum EditMode::variant Normal -->
The element is edited the normal way (default).
 If acting on the element as a whole (`Edge::None`), this will MOVE
 the element by MOVING its toplevel. When acting on the start of the
 element (`Edge::Start`), this will only MOVE the element, but not
 its toplevel parent. This can allow you to move a `Clip` or
 `Group` to a new start time or layer within its container group,
 without effecting other members of the group. When acting on the end
 of the element (`Edge::End`), this will END-TRIM the element,
 leaving its toplevel unchanged.
<!-- enum EditMode::variant Ripple -->
The element is edited in ripple mode: moving
 itself as well as later elements, keeping their relative times. This
 edits the element the same as `EditMode::Normal`. In addition, if
 acting on the element as a whole, or the start of the element, any
 toplevel element in the same timeline (including different layers)
 whose start time is later than the *current* start time of the MOVED
 element will also be MOVED by the same shift as the edited element.
 If acting on the end of the element, any toplevel element whose start
 time is later than the *current* end time of the edited element will
 also be MOVED by the same shift as the change in the end of the
 edited element. These additional elements will also be shifted by
 the same shift in layers as the edited element.
<!-- enum EditMode::variant Roll -->
The element is edited in roll mode: swapping its
 content for its neighbour's, or vis versa, in the timeline output.
 This edits the element the same as `EditMode::Trim`. In addition,
 any neighbours are also TRIMMED at their opposite edge to the same
 timeline position. When acting on the start of the element, a
 neighbour is any earlier element in the timeline whose end time
 matches the *current* start time of the edited element. When acting on
 the end of the element, a neighbour is any later element in the
 timeline whose start time matches the *current* start time of the
 edited element. In addition, a neighbour have a `Source` at its
 end/start edge that shares a track with a `Source` at the start/end
 edge of the edited element. Basically, a neighbour is an element that
 can be extended, or cut, to have its content replace, or be replaced
 by, the content of the edited element. Acting on the element as a
 whole (`Edge::None`) is not defined. The element can not shift
 layers under this mode.
<!-- enum EditMode::variant Trim -->
The element is edited in trim mode. When acting
 on the start of the element, this will START-TRIM it. When acting on
 the end of the element, this will END-TRIM it. Acting on the element
 as a whole (`Edge::None`) is not defined.
<!-- enum EditMode::variant Slide -->
The element is edited in slide mode (not yet
 implemented): moving the element replacing or consuming content on
 each end. When acting on the element as a whole, this will MOVE the
 element, and TRIM any neighbours on either side. A neighbour is
 defined in the same way as in `EditMode::Roll`, but they may be on
 either side of the edited elements. Elements at the end with be
 START-TRIMMED to the new end position of the edited element. Elements
 at the start will be END-TRIMMED to the new start position of the
 edited element. Acting on the start or end of the element
 (`Edge::Start` and `Edge::End`) is not defined. The element can
 not shift layers under this mode.
<!-- struct Effect -->
Currently we only support effects with N sinkpads and one single srcpad.
Apart from `gesaudiomixer` and `gescompositor` which can be used as effects
and where sinkpads will be requested as needed based on the timeline topology
GES will always request at most one sinkpad per effect (when required).

> Note: GES always adds converters (`audioconvert ! audioresample !
> audioconvert` for audio effects and `videoconvert` for video effects) to
> make it simpler for end users.

# Implements

[`EffectExt`](trait@crate::EffectExt), [`BaseEffectExt`](trait@crate::BaseEffectExt), [`TrackElementExt`](trait@crate::TrackElementExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait EffectExt -->
Trait containing all `Effect` methods.

# Implementors

[`Effect`](struct@crate::Effect)
<!-- impl Effect::fn new -->
Creates a new `Effect` from the description of the bin. It should be
possible to determine the type of the effect through the element
'klass' metadata of the GstElements that will be created.
In that corner case, you should use:
`Asset::request` (GES_TYPE_EFFECT, "audio your ! bin ! description", NULL);
and extract that asset to be in full control.
## `bin_description`
The gst-launch like bin description of the effect

# Returns

a newly created `Effect`, or `None` if something went
wrong.
<!-- trait EffectExt::fn get_property_bin_description -->
The description of the effect bin with a gst-launch-style
pipeline description.

Example: "videobalance saturation=1.5 hue=+0.5"
<!-- trait EffectExt::fn set_property_bin_description -->
The description of the effect bin with a gst-launch-style
pipeline description.

Example: "videobalance saturation=1.5 hue=+0.5"
<!-- struct Extractable -->
A `glib::object::Object` that implements the `Extractable` interface can be
extracted from a `Asset` using `AssetExt::extract`.

Each extractable type will have its own way of interpreting the
`Asset:id` of an asset (or, if it is associated with a specific
subclass of `Asset`, the asset subclass may handle the
interpretation of the `Asset:id`). By default, the requested asset
`Asset:id` will be ignored by a `Extractable` and will be set to
the type name of the extractable instead. Also by default, when the
requested asset is extracted, the returned object will simply be a
newly created default object of that extractable type. You should check
the documentation for each extractable type to see if they differ from
the default.

After the object is extracted, it will have a reference to the asset it
came from, which you can retrieve using `Extractable::get_asset`.

# Implements

[`ExtractableExt`](trait@crate::ExtractableExt), [`trait@glib::object::ObjectExt`]
<!-- trait ExtractableExt -->
Trait containing all `Extractable` methods.

# Implementors

[`BaseEffect`](struct@crate::BaseEffect), [`BaseTransitionClip`](struct@crate::BaseTransitionClip), [`Clip`](struct@crate::Clip), [`Container`](struct@crate::Container), [`Effect`](struct@crate::Effect), [`Extractable`](struct@crate::Extractable), [`Group`](struct@crate::Group), [`Layer`](struct@crate::Layer), [`OperationClip`](struct@crate::OperationClip), [`TimelineElement`](struct@crate::TimelineElement), [`Timeline`](struct@crate::Timeline), [`TrackElement`](struct@crate::TrackElement), [`TransitionClip`](struct@crate::TransitionClip), [`UriClip`](struct@crate::UriClip)
<!-- trait ExtractableExt::fn asset -->
Get the asset that has been set on the extractable object.

# Returns

The asset set on `self`, or `None`
if no asset has been set.
<!-- trait ExtractableExt::fn id -->
Gets the `Asset:id` of some associated asset. It may be the case
that the object has no set asset, or even that such an asset does not
yet exist in the GES cache. Instead, this will return the asset
`Asset:id` that is _compatible_ with the current state of the object,
as determined by the `Extractable` implementer. If it was indeed
extracted from an asset, this should return the same as its
corresponding asset `Asset:id`.

# Returns

The `Asset:id` of some associated `Asset`
that is compatible with `self`'s current state.
<!-- trait ExtractableExt::fn set_asset -->
Sets the asset for this extractable object.

When an object is extracted from an asset using `AssetExt::extract` its
asset will be automatically set. Note that many classes that implement
`Extractable` will automatically create their objects using assets
when you call their `new` methods. However, you can use this method to
associate an object with a compatible asset if it was created by other
means and does not yet have an asset. Or, for some implementations of
`Extractable`, you can use this to change the asset of the given
extractable object, which will lead to a change in its state to
match the new asset `Asset:id`.
## `asset`
The asset to set

# Returns

`true` if `asset` could be successfully set on `self`.
<!-- struct Group -->
A `Group` controls one or more `Container`-s (usually `Clip`-s,
but it can also control other `Group`-s). Its children must share
the same `Timeline`, but can otherwise lie in separate `Layer`-s
and have different timings.

To initialise a group, you may want to use `Container::group`,
and similarly use `GESContainerExt::ungroup` to dispose of it.

A group will maintain the relative `TimelineElement:start` times of
its children, as well as their relative layer `Layer:priority`.
Therefore, if one of its children has its `TimelineElement:start`
set, all other children will have their `TimelineElement:start`
shifted by the same amount. Similarly, if one of its children moves to
a new layer, the other children will also change layers to maintain the
difference in their layer priorities. For example, if a child moves
from a layer with `Layer:priority` 1 to a layer with priority 3, then
another child that was in a layer with priority 0 will move to the
layer with priority 2.

The `Group:start` of a group refers to the earliest start
time of its children. If the group's `Group:start` is set, all the
children will be shifted equally such that the earliest start time
will match the set value. The `Group:duration` of a group is the
difference between the earliest start time and latest end time of its
children. If the group's `Group:duration` is increased, the children
whose end time matches the end of the group will be extended
accordingly. If it is decreased, then any child whose end time exceeds
the new end time will also have their duration decreased accordingly.

A group may span several layers, but for methods such as
`TimelineElementExt::get_layer_priority` and
`TimelineElementExt::edit` a group is considered to have a layer
priority that is the highest `Layer:priority` (numerically, the
smallest) of all the layers it spans.

# Implements

[`GroupExt`](trait@crate::GroupExt), [`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait GroupExt -->
Trait containing all `Group` methods.

# Implementors

[`Group`](struct@crate::Group)
<!-- impl Group::fn new -->
Created a new empty group. You may wish to use
`Container::group` instead, which can return a different
`Container` subclass if possible.

# Returns

The new empty group.
<!-- trait GroupExt::fn get_property_duration -->
An overwrite of the `TimelineElement:duration` property. For a
`Group`, this is the difference between the earliest
`TimelineElement:start` time and the latest end time (given by
`TimelineElement:start` + `TimelineElement:duration`) amongst
its children.
<!-- trait GroupExt::fn set_property_duration -->
An overwrite of the `TimelineElement:duration` property. For a
`Group`, this is the difference between the earliest
`TimelineElement:start` time and the latest end time (given by
`TimelineElement:start` + `TimelineElement:duration`) amongst
its children.
<!-- trait GroupExt::fn get_property_in_point -->
An overwrite of the `TimelineElement:in-point` property. This has
no meaning for a group and should not be set.
<!-- trait GroupExt::fn set_property_in_point -->
An overwrite of the `TimelineElement:in-point` property. This has
no meaning for a group and should not be set.
<!-- trait GroupExt::fn get_property_max_duration -->
An overwrite of the `TimelineElement:max-duration` property. This
has no meaning for a group and should not be set.
<!-- trait GroupExt::fn set_property_max_duration -->
An overwrite of the `TimelineElement:max-duration` property. This
has no meaning for a group and should not be set.
<!-- trait GroupExt::fn get_property_priority -->
An overwrite of the `TimelineElement:priority` property.
Setting `TimelineElement` priorities is deprecated as all priority
management is now done by GES itself.
<!-- trait GroupExt::fn set_property_priority -->
An overwrite of the `TimelineElement:priority` property.
Setting `TimelineElement` priorities is deprecated as all priority
management is now done by GES itself.
<!-- trait GroupExt::fn get_property_start -->
An overwrite of the `TimelineElement:start` property. For a
`Group`, this is the earliest `TimelineElement:start` time
amongst its children.
<!-- trait GroupExt::fn set_property_start -->
An overwrite of the `TimelineElement:start` property. For a
`Group`, this is the earliest `TimelineElement:start` time
amongst its children.
<!-- struct Layer -->
`Layer`-s are responsible for collecting and ordering `Clip`-s.

A layer within a timeline will have an associated priority,
corresponding to their index within the timeline. A layer with the
index/priority 0 will have the highest priority and the layer with the
largest index will have the lowest priority (the order of priorities,
in this sense, is the _reverse_ of the numerical ordering of the
indices). `TimelineExt::move_layer` should be used if you wish to
change how layers are prioritised in a timeline.

Layers with higher priorities will have their content priorities
over content from lower priority layers, similar to how layers are
used in image editing. For example, if two separate layers both
display video content, then the layer with the higher priority will
have its images shown first. The other layer will only have its image
shown if the higher priority layer has no content at the given
playtime, or is transparent in some way. Audio content in separate
layers will simply play in addition.

# Implements

[`LayerExt`](trait@crate::LayerExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt)
<!-- trait LayerExt -->
Trait containing all `Layer` methods.

# Implementors

[`Layer`](struct@crate::Layer)
<!-- impl Layer::fn new -->
Creates a new layer.

# Returns

A new layer.
<!-- trait LayerExt::fn add_asset -->
See `LayerExt::add_asset_full`, which also gives an error.
## `asset`
The asset to extract the new clip from
## `start`
The `TimelineElement:start` value to set on the new clip
If `start == #GST_CLOCK_TIME_NONE`, it will be added to the end
of `self`, i.e. it will be set to `self`'s duration
## `inpoint`
The `TimelineElement:in-point` value to set on the new
clip
## `duration`
The `TimelineElement:duration` value to set on the new
clip
## `track_types`
The `Clip:supported-formats` to set on the the new
clip, or `TrackType::Unknown` to use the default

# Returns

The newly created clip.
<!-- trait LayerExt::fn add_asset_full -->
Extracts a new clip from an asset and adds it to the layer with
the given properties.

Feature: `v1_18`

## `asset`
The asset to extract the new clip from
## `start`
The `TimelineElement:start` value to set on the new clip
If `start == #GST_CLOCK_TIME_NONE`, it will be added to the end
of `self`, i.e. it will be set to `self`'s duration
## `inpoint`
The `TimelineElement:in-point` value to set on the new
clip
## `duration`
The `TimelineElement:duration` value to set on the new
clip
## `track_types`
The `Clip:supported-formats` to set on the the new
clip, or `TrackType::Unknown` to use the default

# Returns

The newly created clip.
<!-- trait LayerExt::fn add_clip -->
See `LayerExt::add_clip_full`, which also gives an error.
## `clip`
The clip to add

# Returns

`true` if `clip` was properly added to `self`, or `false`
if `self` refused to add `clip`.
<!-- trait LayerExt::fn add_clip_full -->
Adds the given clip to the layer. If the method succeeds, the layer
will take ownership of the clip.

This method will fail and return `false` if `clip` already resides in
some layer. It can also fail if the additional clip breaks some
compositional rules (see `TimelineElement`).

Feature: `v1_18`

## `clip`
The clip to add

# Returns

`true` if `clip` was properly added to `self`, or `false`
if `self` refused to add `clip`.
<!-- trait LayerExt::fn get_active_for_track -->
Gets whether the layer is active for the given track. See
`LayerExt::set_active_for_tracks`.

Feature: `v1_18`

## `track`
The `Track` to check if `self` is currently active for

# Returns

`true` if `self` is active for `track`, or `false` otherwise.
<!-- trait LayerExt::fn is_auto_transition -->
Gets the `Layer:auto-transition` of the layer.

# Returns

`true` if transitions are automatically added to `self`.
<!-- trait LayerExt::fn clips -->
Get the `Clip`-s contained in this layer.

# Returns

A list of clips in
`self`.
<!-- trait LayerExt::fn get_clips_in_interval -->
Gets the clips within the layer that appear between `start` and `end`.
## `start`
Start of the interval
## `end`
End of the interval

# Returns

A list of `Clip`-s
that intersect the interval `[start, end)` in `self`.
<!-- trait LayerExt::fn duration -->
Retrieves the duration of the layer, which is the difference
between the start of the layer (always time 0) and the end (which will
be the end time of the final clip).

# Returns

The duration of `self`.
<!-- trait LayerExt::fn priority -->
Get the priority of the layer. When inside a timeline, this is its
index in the timeline. See `TimelineExt::move_layer`.

# Returns

The priority of `self` within its timeline.
<!-- trait LayerExt::fn timeline -->
Gets the timeline that the layer is a part of.

# Returns

The timeline that `self`
is currently part of, or `None` if it is not associated with any
timeline.
<!-- trait LayerExt::fn is_empty -->
Convenience method to check if the layer is empty (doesn't contain
any `Clip`), or not.

# Returns

`true` if `self` is empty, `false` if it contains at least
one clip.
<!-- trait LayerExt::fn remove_clip -->
Removes the given clip from the layer.
## `clip`
The clip to remove

# Returns

`true` if `clip` was removed from `self`, or `false` if the
operation failed.
<!-- trait LayerExt::fn set_active_for_tracks -->
Activate or deactivate track elements in `tracks` (or in all tracks if `tracks`
is `None`).

When a layer is deactivated for a track, all the `TrackElement`-s in
the track that belong to a `Clip` in the layer will no longer be
active in the track, regardless of their individual
`TrackElement:active` value.

Note that by default a layer will be active for all of its
timeline's tracks.

Feature: `v1_18`

## `active`
Whether elements in `tracks` should be active or not
## `tracks`
The list of
tracks `self` should be (de-)active in, or `None` to include all the tracks
in the `self`'s timeline

# Returns

`true` if the operation worked `false` otherwise.
<!-- trait LayerExt::fn set_auto_transition -->
Sets `Layer:auto-transition` for the layer. Use
`TimelineExt::set_auto_transition` if you want all layers within a
`Timeline` to have `Layer:auto-transition` set to `true`. Use this
method if you want different values for different layers (and make sure
to keep `Timeline:auto-transition` as `false` for the corresponding
timeline).
## `auto_transition`
Whether transitions should be automatically added to
the layer
<!-- trait LayerExt::fn set_priority -->
Sets the layer to the given priority. See `Layer:priority`.

# Deprecated since 1.16

use `TimelineExt::move_layer` instead. This deprecation means
that you will not need to handle layer priorities at all yourself, GES
will make sure there is never 'gaps' between layer priorities.
## `priority`
The priority to set
<!-- trait LayerExt::fn connect_active_changed -->
Will be emitted whenever the layer is activated or deactivated
for some `Track`. See `LayerExt::set_active_for_tracks`.

Feature: `v1_18`

## `active`
Whether `layer` has been made active or de-active in the `tracks`
## `tracks`
A list of `Track`
which have been activated or deactivated
<!-- trait LayerExt::fn connect_clip_added -->
Will be emitted after the clip is added to the layer.
## `clip`
The clip that was added
<!-- trait LayerExt::fn connect_clip_removed -->
Will be emitted after the clip is removed from the layer.
## `clip`
The clip that was removed
<!-- trait LayerExt::fn get_property_auto_transition -->
Whether to automatically create a `TransitionClip` whenever two
`Source`-s that both belong to a `Clip` in the layer overlap.
See `Timeline` for what counts as an overlap.

When a layer is added to a `Timeline`, if this property is left as
`false`, but the timeline's `Timeline:auto-transition` is `true`, it
will be set to `true` as well.
<!-- trait LayerExt::fn set_property_auto_transition -->
Whether to automatically create a `TransitionClip` whenever two
`Source`-s that both belong to a `Clip` in the layer overlap.
See `Timeline` for what counts as an overlap.

When a layer is added to a `Timeline`, if this property is left as
`false`, but the timeline's `Timeline:auto-transition` is `true`, it
will be set to `true` as well.
<!-- trait LayerExt::fn get_property_priority -->
The priority of the layer in the `Timeline`. 0 is the highest
priority. Conceptually, a timeline is a stack of layers,
and the priority of the layer represents its position in the stack. Two
layers should not have the same priority within a given GESTimeline.

Note that the timeline needs to be committed (with `TimelineExt::commit`)
for the change to be taken into account.

# Deprecated since 1.16

use `TimelineExt::move_layer` instead. This deprecation means
that you will not need to handle layer priorities at all yourself, GES
will make sure there is never 'gaps' between layer priorities.
<!-- trait LayerExt::fn set_property_priority -->
The priority of the layer in the `Timeline`. 0 is the highest
priority. Conceptually, a timeline is a stack of layers,
and the priority of the layer represents its position in the stack. Two
layers should not have the same priority within a given GESTimeline.

Note that the timeline needs to be committed (with `TimelineExt::commit`)
for the change to be taken into account.

# Deprecated since 1.16

use `TimelineExt::move_layer` instead. This deprecation means
that you will not need to handle layer priorities at all yourself, GES
will make sure there is never 'gaps' between layer priorities.
<!-- struct OperationClip -->
Operations are any kind of object that both outputs AND consumes data.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ClipExt`](trait@crate::ClipExt), [`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- struct Pipeline -->
A `Pipeline` can take an audio-video `Timeline` and conveniently
link its `Track`-s to an internal `playsink` element, for
preview/playback, and an internal `encodebin` element, for rendering.
You can switch between these modes using `GESPipelineExt::set_mode`.

You can choose the specific audio and video sinks used for previewing
the timeline by setting the `Pipeline:audio-sink` and
`Pipeline:video-sink` properties.

You can set the encoding and save location used in rendering by calling
`GESPipelineExt::set_render_settings`.

# Implements

[`GESPipelineExt`](trait@crate::GESPipelineExt), [`trait@gst::PipelineExt`], [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GESPipelineExt -->
Trait containing all `Pipeline` methods.

# Implementors

[`Pipeline`](struct@crate::Pipeline)
<!-- impl Pipeline::fn new -->
Creates a new pipeline.

# Returns

The newly created pipeline.
<!-- trait GESPipelineExt::fn mode -->
Gets the `Pipeline:mode` of the pipeline.

# Returns

The current mode of `self`.
<!-- trait GESPipelineExt::fn get_thumbnail -->
Gets a sample from the pipeline of the currently displayed image in
preview, in the specified format.

Note that if you use "ANY" caps for `caps`, then the current format of
the image is used. You can retrieve these caps from the returned sample
with `gst::Sample::get_caps`.
## `caps`
Some caps to specifying the desired format, or
`GST_CAPS_ANY` to use the native format

# Returns

A sample of `self`'s current image preview in
the format given by `caps`, or `None` if an error prevented fetching the
sample.
<!-- trait GESPipelineExt::fn get_thumbnail_rgb24 -->
Gets a sample from the pipeline of the currently displayed image in
preview, in the 24-bit "RGB" format and of the desired width and
height.

See `GESPipelineExt::get_thumbnail`.
## `width`
The requested pixel width of the image, or -1 to use the native
size
## `height`
The requested pixel height of the image, or -1 to use the
native size

# Returns

A sample of `self`'s current image preview in
the "RGB" format, scaled to `width` and `height`, or `None` if an error
prevented fetching the sample.
<!-- trait GESPipelineExt::fn preview_get_audio_sink -->
Gets the `Pipeline:audio-sink` of the pipeline.

# Returns

The audio sink used by `self` for preview.
<!-- trait GESPipelineExt::fn preview_get_video_sink -->
Gets the `Pipeline:video-sink` of the pipeline.

# Returns

The video sink used by `self` for preview.
<!-- trait GESPipelineExt::fn preview_set_audio_sink -->
Sets the `Pipeline:audio-sink` of the pipeline.
## `sink`
A audio sink for `self` to use for preview
<!-- trait GESPipelineExt::fn preview_set_video_sink -->
Sets the `Pipeline:video-sink` of the pipeline.
## `sink`
A video sink for `self` to use for preview
<!-- trait GESPipelineExt::fn save_thumbnail -->
Saves the currently displayed image of the pipeline in preview to the
given location, in the specified dimensions and format.
## `width`
The requested pixel width of the image, or -1 to use the native
size
## `height`
The requested pixel height of the image, or -1 to use the
native size
## `format`
The desired mime type (for example, "image/jpeg")
## `location`
The path to save the thumbnail to

# Returns

`true` if `self`'s current image preview was successfully saved
to `location` using the given `format`, `height` and `width`.
<!-- trait GESPipelineExt::fn set_mode -->
Sets the `Pipeline:mode` of the pipeline.

Note that the pipeline will be set to `gst::State::Null` during this call to
perform the necessary changes. You will need to set the state again yourself
after calling this.

> **NOTE**: [Rendering settings](ges_pipeline_set_render_settings) need to be
> set before setting `mode` to `PipelineFlags::Render` or
> `PipelineFlags::SmartRender`, the call to this method will fail
> otherwise.
## `mode`
The mode to set for `self`

# Returns

`true` if the mode of `self` was successfully set to `mode`.
<!-- trait GESPipelineExt::fn set_render_settings -->
Specifies encoding setting to be used by the pipeline to render its
`Pipeline:timeline`, and where the result should be written to.

This method **must** be called before setting the pipeline mode to
`PipelineFlags::Render`.
## `output_uri`
The URI to save the `Pipeline:timeline` rendering
result to
## `profile`
The encoding to use for rendering the `Pipeline:timeline`

# Returns

`true` if the settings were successfully set on `self`.
<!-- trait GESPipelineExt::fn set_timeline -->
Takes the given timeline and sets it as the `Pipeline:timeline` for
the pipeline.

Note that you should only call this method once on a given pipeline
because a pipeline can not have its `Pipeline:timeline` changed after
it has been set.
## `timeline`
The timeline to set for `self`

# Returns

`true` if `timeline` was successfully given to `self`.
<!-- trait GESPipelineExt::fn get_property_audio_filter -->
The audio filter(s) to apply during playback in preview mode,
immediately before the `Pipeline:audio-sink`. This exposes the
`playsink:audio-filter` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn set_property_audio_filter -->
The audio filter(s) to apply during playback in preview mode,
immediately before the `Pipeline:audio-sink`. This exposes the
`playsink:audio-filter` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn get_property_audio_sink -->
The audio sink used for preview. This exposes the
`playsink:audio-sink` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn set_property_audio_sink -->
The audio sink used for preview. This exposes the
`playsink:audio-sink` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn get_property_mode -->
The pipeline's mode. In preview mode (for audio or video, or both)
the pipeline can display the timeline's content to an end user. In
rendering mode the pipeline can encode the timeline's content and
save it to a file.
<!-- trait GESPipelineExt::fn set_property_mode -->
The pipeline's mode. In preview mode (for audio or video, or both)
the pipeline can display the timeline's content to an end user. In
rendering mode the pipeline can encode the timeline's content and
save it to a file.
<!-- trait GESPipelineExt::fn get_property_timeline -->
The timeline used by this pipeline, whose content it will play and
render, or `None` if the pipeline does not yet have a timeline.

Note that after you set the timeline for the first time, subsequent
calls to change the timeline will fail.
<!-- trait GESPipelineExt::fn set_property_timeline -->
The timeline used by this pipeline, whose content it will play and
render, or `None` if the pipeline does not yet have a timeline.

Note that after you set the timeline for the first time, subsequent
calls to change the timeline will fail.
<!-- trait GESPipelineExt::fn get_property_video_filter -->
The video filter(s) to apply during playback in preview mode,
immediately before the `Pipeline:video-sink`. This exposes the
`playsink:video-filter` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn set_property_video_filter -->
The video filter(s) to apply during playback in preview mode,
immediately before the `Pipeline:video-sink`. This exposes the
`playsink:video-filter` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn get_property_video_sink -->
The video sink used for preview. This exposes the
`playsink:video-sink` property of the internal `playsink`.
<!-- trait GESPipelineExt::fn set_property_video_sink -->
The video sink used for preview. This exposes the
`playsink:video-sink` property of the internal `playsink`.
<!-- struct PipelineFlags -->
The various modes a `Pipeline` can be configured to.
<!-- struct PipelineFlags::const AUDIO_PREVIEW -->
Output the `Pipeline:timeline`'s
audio to the soundcard
<!-- struct PipelineFlags::const VIDEO_PREVIEW -->
Output the `Pipeline:timeline`'s
video to the screen
<!-- struct PipelineFlags::const FULL_PREVIEW -->
Output both the `Pipeline:timeline`'s
audio and video to the soundcard and screen (default)
<!-- struct PipelineFlags::const RENDER -->
Render the `Pipeline:timeline` with
forced decoding (the underlying `encodebin` has its
`encodebin:avoid-reencoding` property set to `false`)
<!-- struct PipelineFlags::const SMART_RENDER -->
Render the `Pipeline:timeline`,
avoiding decoding/reencoding (the underlying `encodebin` has its
`encodebin:avoid-reencoding` property set to `true`).
> NOTE: Smart rendering can not work in tracks where `Track:mixing`
> is enabled.
<!-- struct Project -->
The `Project` is used to control a set of `Asset` and is a
`Asset` with `GES_TYPE_TIMELINE` as `extractable_type` itself. That
means that you can extract `Timeline` from a project as followed:


```text
 GESProject *project;
 GESTimeline *timeline;

 project = ges_project_new ("file:///path/to/a/valid/project/uri");

 // Here you can connect to the various signal to get more infos about
 // what is happening and recover from errors if possible
 ...

 timeline = ges_asset_extract (GES_ASSET (project));
```

The `Project` class offers a higher level API to handle `Asset`-s.
It lets you request new asset, and it informs you about new assets through
a set of signals. Also it handles problem such as missing files/missing
`gst::Element` and lets you try to recover from those.

## Subprojects

In order to add a subproject, the only thing to do is to add the subproject
to the main project:

``` c
ges_project_add_asset (project, GES_ASSET (subproject));
```
then the subproject will be serialized in the project files. To use
the subproject in a timeline, you should use a `UriClip` with the
same subproject URI.

When loading a project with subproject, subprojects URIs will be temporary
writable local files. If you want to edit the subproject timeline,
you should retrieve the subproject from the parent project asset list and
extract the timeline with `AssetExt::extract` and save it at
the same temporary location.

# Implements

[`ProjectExt`](trait@crate::ProjectExt), [`AssetExt`](trait@crate::AssetExt), [`trait@glib::object::ObjectExt`]
<!-- trait ProjectExt -->
Trait containing all `Project` methods.

# Implementors

[`Project`](struct@crate::Project)
<!-- impl Project::fn new -->
Creates a new `Project` and sets its uri to `uri` if provided. Note that
if `uri` is not valid or `None`, the uri of the project will then be set
the first time you save the project. If you then save the project to
other locations, it will never be updated again and the first valid URI is
the URI it will keep refering to.
## `uri`
The uri to be set after creating the project.

# Returns

A newly created `Project`
<!-- trait ProjectExt::fn add_asset -->
Adds a `Asset` to `self`, the project will keep a reference on
`asset`.
## `asset`
A `Asset` to add to `self`

# Returns

`true` if the asset could be added `false` it was already
in the project
<!-- trait ProjectExt::fn add_encoding_profile -->
Adds `profile` to the project. It lets you save in what format
the project has been renders and keep a reference to those formats.
Also, those formats will be saves to the project file when possible.
## `profile`
A `gst_pbutils::EncodingProfile` to add to the project. If a profile with
the same name already exists, it will be replaced

# Returns

`true` if `profile` could be added, `false` otherwize
<!-- trait ProjectExt::fn add_formatter -->
Adds a formatter as used to load `self`

Feature: `v1_18`

## `formatter`
A formatter used by `self`
<!-- trait ProjectExt::fn create_asset -->
Create and add a `Asset` to `self`. You should connect to the
"asset-added" signal to get the asset when it finally gets added to
`self`
## `id`
The id of the asset to create and add to `self`
## `extractable_type`
The `glib::Type` of the asset to create

# Returns

`true` if the asset started to be added `false` it was already
in the project
<!-- trait ProjectExt::fn create_asset_sync -->
Create and add a `Asset` to `self`. You should connect to the
"asset-added" signal to get the asset when it finally gets added to
`self`
## `id`
The id of the asset to create and add to `self`
## `extractable_type`
The `glib::Type` of the asset to create

# Returns

The newly created `Asset` or `None`.
<!-- trait ProjectExt::fn get_asset -->
## `id`
The id of the asset to retrieve
## `extractable_type`
The extractable_type of the asset
to retrieve from `object`

# Returns

The `Asset` with
`id` or `None` if no asset with `id` as an ID
<!-- trait ProjectExt::fn loading_assets -->
Get the assets that are being loaded

# Returns

A set of loading asset
that will be added to `self`. Note that those Asset are *not* loaded yet,
and thus can not be used
<!-- trait ProjectExt::fn uri -->
Retrieve the uri that is currently set on `self`

# Returns

a newly allocated string representing uri.
<!-- trait ProjectExt::fn list_assets -->
List all `asset` contained in `self` filtering per extractable_type
as defined by `filter`. It copies the asset and thus will not be updated
in time.
## `filter`
Type of assets to list, `GES_TYPE_EXTRACTABLE` will list
all assets

# Returns

The list of
`Asset` the object contains
<!-- trait ProjectExt::fn list_encoding_profiles -->
Lists the encoding profile that have been set to `self`. The first one
is the latest added.

# Returns

The
list of `gst_pbutils::EncodingProfile` used in `self`
<!-- trait ProjectExt::fn load -->
Loads `self` into `timeline`
## `timeline`
A blank timeline to load `self` into

# Returns

`true` if the project could be loaded `false` otherwize.
<!-- trait ProjectExt::fn remove_asset -->
remove a `asset` to from `self`.
## `asset`
A `Asset` to remove from `self`

# Returns

`true` if the asset could be removed `false` otherwise
<!-- trait ProjectExt::fn save -->
Save the timeline of `self` to `uri`. You should make sure that `timeline`
is one of the timelines that have been extracted from `self`
(using ges_asset_extract (`self`);)
## `timeline`
The `Timeline` to save, it must have been extracted from `self`
## `uri`
The uri where to save `self` and `timeline`
## `formatter_asset`
The formatter asset to
use or `None`. If `None`, will try to save in the same format as the one
from which the timeline as been loaded or default to the best formatter
as defined in `ges_find_formatter_for_uri`
## `overwrite`
`true` to overwrite file if it exists

# Returns

`true` if the project could be save, `false` otherwize
<!-- trait ProjectExt::fn connect_asset_added -->
## `asset`
The `Asset` that has been added to `project`
<!-- trait ProjectExt::fn connect_asset_loading -->
## `asset`
The `Asset` that started loading
<!-- trait ProjectExt::fn connect_asset_removed -->
## `asset`
The `Asset` that has been removed from `project`
<!-- trait ProjectExt::fn connect_error_loading -->

Feature: `v1_18`

## `timeline`
The timeline that failed loading
## `error`
The `glib::Error` defining the error that occured
<!-- trait ProjectExt::fn connect_error_loading_asset -->
Informs you that a `Asset` could not be created. In case of
missing GStreamer plugins, the error will be set to `GST_CORE_ERROR`
`gst::CoreError::MissingPlugin`
## `error`
The `glib::Error` defining the error that occured, might be `None`
## `id`
The `id` of the asset that failed loading
## `extractable_type`
The `extractable_type` of the asset that
failed loading
<!-- trait ProjectExt::fn connect_loaded -->
## `timeline`
The `Timeline` that completed loading
<!-- trait ProjectExt::fn connect_loading -->

Feature: `v1_18`

## `timeline`
The `Timeline` that started loading
<!-- trait ProjectExt::fn connect_missing_uri -->

```text
static gchar
source_moved_cb (GESProject *project, GError *error, GESAsset *asset_with_error)
{
  return g_strdup ("file:///the/new/uri.ogg");
}

static int
main (int argc, gchar ** argv)
{
  GESTimeline *timeline;
  GESProject *project = ges_project_new ("file:///some/uri.xges");

  g_signal_connect (project, "missing-uri", source_moved_cb, NULL);
  timeline = ges_asset_extract (GES_ASSET (project));
}
```
## `error`
The error that happened
## `wrong_asset`
The asset with the wrong ID, you should us it and its content
only to find out what the new location is.

# Returns

The new URI of `wrong_asset`
<!-- struct Timeline -->
`Timeline` is the central object for any multimedia timeline.

A timeline is composed of a set of `Track`-s and a set of
`Layer`-s, which are added to the timeline using
`TimelineExt::add_track` and `TimelineExt::append_layer`, respectively.

The contained tracks define the supported types of the timeline
and provide the media output. Essentially, each track provides an
additional source `gst::Pad`.

Most usage of a timeline will likely only need a single `AudioTrack`
and/or a single `VideoTrack`. You can create such a timeline with
`Timeline::new_audio_video`. After this, you are unlikely to need to
work with the tracks directly.

A timeline's layers contain `Clip`-s, which in turn control the
creation of `TrackElement`-s, which are added to the timeline's
tracks. See `Timeline::select-tracks-for-object` if you wish to have
more control over which track a clip's elements are added to.

The layers are ordered, with higher priority layers having their
content prioritised in the tracks. This ordering can be changed using
`TimelineExt::move_layer`.

## Editing

See `TimelineElement` for the various ways the elements of a timeline
can be edited.

If you change the timing or ordering of a timeline's
`TimelineElement`-s, then these changes will not actually be taken
into account in the output of the timeline's tracks until the
`TimelineExt::commit` method is called. This allows you to move its
elements around, say, in response to an end user's mouse dragging, with
little expense before finalising their effect on the produced data.

## Overlaps and Auto-Transitions

There are certain restrictions placed on how `Source`-s may overlap
in a `Track` that belongs to a timeline. These will be enforced by
GES, so the user will not need to keep track of them, but they should
be aware that certain edits will be refused as a result if the overlap
rules would be broken.

Consider two `Source`-s, `A` and `B`, with start times `startA` and
`startB`, and end times `endA` and `endB`, respectively. The start
time refers to their `TimelineElement:start`, and the end time is
their `TimelineElement:start` + `TimelineElement:duration`. These
two sources *overlap* if:

+ they share the same `TrackElement:track` (non `None`), which belongs
 to the timeline;
+ they share the same `GES_TIMELINE_ELEMENT_LAYER_PRIORITY`; and
+ `startA < endB` and `startB < endA `.

Note that when `startA = endB` or `startB = endA` then the two sources
will *touch* at their edges, but are not considered overlapping.

If, in addition, `startA < startB < endA`, then we can say that the
end of `A` overlaps the start of `B`.

If, instead, `startA <= startB` and `endA >= endB`, then we can say
that `A` fully overlaps `B`.

The overlap rules for a timeline are that:

1. One source cannot fully overlap another source.
2. A source can only overlap the end of up to one other source at its
 start.
3. A source can only overlap the start of up to one other source at its
 end.

The last two rules combined essentially mean that at any given timeline
position, only up to two `Source`-s may overlap at that position. So
triple or more overlaps are not allowed.

If you switch on `Timeline:auto-transition`, then at any moment when
the end of one source (the first source) overlaps the start of another
(the second source), a `TransitionClip` will be automatically created
for the pair in the same layer and it will cover their overlap. If the
two elements are edited in a way such that the end of the first source
no longer overlaps the start of the second, the transition will be
automatically removed from the timeline. However, if the two sources
still overlap at the same edges after the edit, then the same
transition object will be kept, but with its timing and layer adjusted
accordingly.

## Saving

To save/load a timeline, you can use the `TimelineExt::load_from_uri`
and `TimelineExt::save_to_uri` methods that use the default format.

## Playing

A timeline is a `gst::Bin` with a source `gst::Pad` for each of its
tracks, which you can fetch with `TimelineExt::get_pad_for_track`. You
will likely want to link these to some compatible sink `gst::Element`-s to
be able to play or capture the content of the timeline.

You can use a `Pipeline` to easily preview/play the timeline's
content, or render it to a file.

# Implements

[`TimelineExt`](trait@crate::TimelineExt), [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt)
<!-- trait TimelineExt -->
Trait containing all `Timeline` methods.

# Implementors

[`Timeline`](struct@crate::Timeline)
<!-- impl Timeline::fn new -->
Creates a new empty timeline.

# Returns

The new timeline.
<!-- impl Timeline::fn new_audio_video -->
Creates a new timeline containing a single `AudioTrack` and a
single `VideoTrack`.

# Returns

The new timeline, or `None` if the tracks
could not be created and added.
<!-- impl Timeline::fn from_uri -->
Creates a timeline from the given URI.
## `uri`
The URI to load from

# Returns

A new timeline if the uri was loaded
successfully, or `None` if the uri could not be loaded.
<!-- trait TimelineExt::fn add_layer -->
Add a layer to the timeline.

If the layer contains `Clip`-s, then this may trigger the creation of
their core track element children for the timeline's tracks, and the
placement of the clip's children in the tracks of the timeline using
`Timeline::select-tracks-for-object`. Some errors may occur if this
would break one of the configuration rules of the timeline in one of
its tracks. In such cases, some track elements would fail to be added
to their tracks, but this method would still return `true`. As such, it
is advised that you only add clips to layers that already part of a
timeline. In such situations, `LayerExt::add_clip` is able to fail if
adding the clip would cause such an error.

# Deprecated since 1.18

This method requires you to ensure the layer's
`Layer:priority` will be unique to the timeline. Use
`TimelineExt::append_layer` and `TimelineExt::move_layer` instead.
## `layer`
The layer to add

# Returns

`true` if `layer` was properly added.
<!-- trait TimelineExt::fn add_track -->
Add a track to the timeline.

If the timeline already contains clips, then this may trigger the
creation of their core track element children for the track, and the
placement of the clip's children in the track of the timeline using
`Timeline::select-tracks-for-object`. Some errors may occur if this
would break one of the configuration rules for the timeline in the
track. In such cases, some track elements would fail to be added to the
track, but this method would still return `true`. As such, it is advised
that you avoid adding tracks to timelines that already contain clips.
## `track`
The track to add

# Returns

`true` if `track` was properly added.
<!-- trait TimelineExt::fn append_layer -->
Append a newly created layer to the timeline. The layer will
be added at the lowest `Layer:priority` (numerically, the highest).

# Returns

The newly created layer.
<!-- trait TimelineExt::fn commit -->
Commit all the pending changes of the clips contained in the
timeline.

When changes happen in a timeline, they are not immediately executed
internally, in a way that effects the output data of the timeline. You
should call this method when you are done with a set of changes and you
want them to be executed.

Any pending changes will be executed in the backend. The
`Timeline::commited` signal will be emitted once this has completed.
You should not try to change the state of the timeline, seek it or add
tracks to it before receiving this signal. You can use
`TimelineExt::commit_sync` if you do not want to perform other tasks in
the mean time.

Note that all the pending changes will automatically be executed when
the timeline goes from `gst::State::Ready` to `gst::State::Paused`, which is
usually triggered by a corresponding state changes in a containing
`Pipeline`.

# Returns

`true` if pending changes were committed, or `false` if nothing
needed to be committed.
<!-- trait TimelineExt::fn commit_sync -->
Commit all the pending changes of the clips contained in the
timeline and wait for the changes to complete.

See `TimelineExt::commit`.

# Returns

`true` if pending changes were committed, or `false` if nothing
needed to be committed.
<!-- trait TimelineExt::fn is_auto_transition -->
Gets `Timeline:auto-transition` for the timeline.

# Returns

The auto-transition of `self_`.
<!-- trait TimelineExt::fn duration -->
Get the current `Timeline:duration` of the timeline

# Returns

The current duration of `self`.
<!-- trait TimelineExt::fn get_element -->
Gets the element contained in the timeline with the given name.
## `name`
The name of the element to find

# Returns

The timeline element in `self`
with the given `name`, or `None` if it was not found.
<!-- trait TimelineExt::fn get_frame_at -->
This method allows you to convert a timeline `gst::ClockTime` into its
corresponding `FrameNumber` in the timeline's output.

Feature: `v1_18`

## `timestamp`
The timestamp to get the corresponding frame number of

# Returns

The frame number `timestamp` corresponds to.
<!-- trait TimelineExt::fn get_frame_time -->
This method allows you to convert a timeline output frame number into a
timeline `gst::ClockTime`. For example, this time could be used to seek to a
particular frame in the timeline's output, or as the edit position for
an element within the timeline.

Feature: `v1_18`

## `frame_number`
The frame number to get the corresponding timestamp of in the
 timeline coordinates

# Returns

The timestamp corresponding to `frame_number` in the output of `self`.
<!-- trait TimelineExt::fn groups -->
Get the list of `Group`-s present in the timeline.

# Returns

The list of
groups that contain clips present in `self`'s layers.
Must not be changed.
<!-- trait TimelineExt::fn get_layer -->
Retrieve the layer whose index in the timeline matches the given
priority.
## `priority`
The priority/index of the layer to find

# Returns

The layer with the given
`priority`, or `None` if none was found.

Since 1.6
<!-- trait TimelineExt::fn layers -->
Get the list of `Layer`-s present in the timeline.

# Returns

The list of
layers present in `self` sorted by priority.
<!-- trait TimelineExt::fn get_pad_for_track -->
Search for the `gst::Pad` corresponding to the given timeline's track.
You can link to this pad to receive the output data of the given track.
## `track`
A track

# Returns

The pad corresponding to `track`,
or `None` if there is an error.
<!-- trait TimelineExt::fn snapping_distance -->
Gets the `Timeline:snapping-distance` for the timeline.

# Returns

The snapping distance (in nanoseconds) of `self`.
<!-- trait TimelineExt::fn get_track_for_pad -->
Search for the `Track` corresponding to the given timeline's pad.
## `pad`
A pad

# Returns

The track corresponding to `pad`,
or `None` if there is an error.
<!-- trait TimelineExt::fn tracks -->
Get the list of `Track`-s used by the timeline.

# Returns

The list of tracks
used by `self`.
<!-- trait TimelineExt::fn is_empty -->
Check whether the timeline is empty or not.

# Returns

`true` if `self` is empty.
<!-- trait TimelineExt::fn load_from_uri -->
Loads the contents of URI into the timeline.
## `uri`
The URI to load from

# Returns

`true` if the timeline was loaded successfully from `uri`.
<!-- trait TimelineExt::fn move_layer -->
Moves a layer within the timeline to the index given by
`new_layer_priority`.
An index of 0 corresponds to the layer with the highest priority in a
timeline. If `new_layer_priority` is greater than the number of layers
present in the timeline, it will become the lowest priority layer.

Feature: `v1_16`

## `layer`
A layer within `self`, whose priority should be changed
## `new_layer_priority`
The new index for `layer`
<!-- trait TimelineExt::fn paste_element -->
Paste an element inside the timeline. `element` **must** be the return of
`TimelineElementExt::copy` with `deep=TRUE`,
and it should not be changed before pasting. `element` itself is not
placed in the timeline, instead a new element is created, alike to the
originally copied element. Note that the originally copied element must
also lie within `self`, at both the point of copying and pasting.

Pasting may fail if it would place the timeline in an unsupported
configuration.

After calling this function `element` should not be used. In particular,
`element` can **not** be pasted again. Instead, you can copy the
returned element and paste that copy (although, this is only possible
if the paste was successful).

See also `TimelineElementExt::paste`.
## `element`
The element to paste
## `position`
The position in the timeline `element` should be pasted to,
i.e. the `TimelineElement:start` value for the pasted element.
## `layer_priority`
The layer into which the element should be pasted.
-1 means paste to the same layer from which `element` has been copied from

# Returns

The newly created element, or
`None` if pasting fails.
<!-- trait TimelineExt::fn remove_layer -->
Removes a layer from the timeline.
## `layer`
The layer to remove

# Returns

`true` if `layer` was properly removed.
<!-- trait TimelineExt::fn remove_track -->
Remove a track from the timeline.
## `track`
The track to remove

# Returns

`true` if `track` was properly removed.
<!-- trait TimelineExt::fn save_to_uri -->
Saves the timeline to the given location. If `formatter_asset` is `None`,
the method will attempt to save in the same format the timeline was
loaded from, before defaulting to the formatter with highest rank.
## `uri`
The location to save to
## `formatter_asset`
The formatter asset to use, or `None`
## `overwrite`
`true` to overwrite file if it exists

# Returns

`true` if `self` was successfully saved to `uri`.
<!-- trait TimelineExt::fn set_auto_transition -->
Sets `Timeline:auto-transition` for the timeline. This will also set
the corresponding `Layer:auto-transition` for all of the timeline's
layers to the same value. See `LayerExt::set_auto_transition` if you
wish to set the layer's `Layer:auto-transition` individually.
## `auto_transition`
Whether transitions should be automatically added
to `self`'s layers
<!-- trait TimelineExt::fn set_snapping_distance -->
Sets `Timeline:snapping-distance` for the timeline. This new value
will only effect future snappings and will not be used to snap the
current element positions within the timeline.
## `snapping_distance`
The snapping distance to use (in nanoseconds)
<!-- trait TimelineExt::fn connect_commited -->
This signal will be emitted once the changes initiated by
`TimelineExt::commit` have been executed in the backend. Use
`TimelineExt::commit_sync` if you do not want to have to connect
to this signal.
<!-- trait TimelineExt::fn connect_group_added -->
Will be emitted after the group is added to to the timeline. This can
happen when grouping with `ges_container_group`, or by adding
containers to a newly created group.

Note that this should not be emitted whilst a timeline is being
loaded from its `Project` asset. You should connect to the
project's `Project::loaded` signal if you want to know which groups
were created for the timeline.
## `group`
The group that was added to `timeline`
<!-- trait TimelineExt::fn connect_group_removed -->
Will be emitted after the group is removed from the timeline through
`ges_container_ungroup`. Note that `group` will no longer contain its
former children, these are held in `children`.

Note that if a group is emptied, then it will no longer belong to the
timeline, but this signal will **not** be emitted in such a case.
## `group`
The group that was removed from `timeline`
## `children`
A list
of `Container`-s that _were_ the children of the removed `group`
<!-- trait TimelineExt::fn connect_layer_added -->
Will be emitted after the layer is added to the timeline.

Note that this should not be emitted whilst a timeline is being
loaded from its `Project` asset. You should connect to the
project's `Project::loaded` signal if you want to know which
layers were created for the timeline.
## `layer`
The layer that was added to `timeline`
<!-- trait TimelineExt::fn connect_layer_removed -->
Will be emitted after the layer is removed from the timeline.
## `layer`
The layer that was removed from `timeline`
<!-- trait TimelineExt::fn connect_select_element_track -->
Simplified version of `Timeline::select-tracks-for-object` which only
allows `track_element` to be added to a single `Track`.

Feature: `v1_18`

## `clip`
The clip that `track_element` is being added to
## `track_element`
The element being added

# Returns

A track to put `track_element` into, or `None` if
it should be discarded.
<!-- trait TimelineExt::fn connect_select_tracks_for_object -->
This will be emitted whenever the timeline needs to determine which
tracks a clip's children should be added to. The track element will
be added to each of the tracks given in the return. If a track
element is selected to go into multiple tracks, it will be copied
into the additional tracks, under the same clip. Note that the copy
will *not* keep its properties or state in sync with the original.

Connect to this signal once if you wish to control which element
should be added to which track. Doing so will overwrite the default
behaviour, which adds `track_element` to all tracks whose
`Track:track-type` includes the `track_element`'s
`TrackElement:track-type`.

Note that under the default track selection, if a clip would produce
multiple core children of the same `TrackType`, it will choose
one of the core children arbitrarily to place in the corresponding
tracks, with a warning for the other core children that are not
placed in the track. For example, this would happen for a `UriClip`
that points to a file that contains multiple audio streams. If you
wish to choose the stream, you could connect to this signal, and use,
say, `UriSourceAssetExt::get_stream_info` to choose which core
source to add.

When a clip is first added to a timeline, its core elements will
be created for the current tracks in the timeline if they have not
already been created. Then this will be emitted for each of these
core children to select which tracks, if any, they should be added
to. It will then be called for any non-core children in the clip.

In addition, if a new track element is ever added to a clip in a
timeline (and it is not already part of a track) this will be emitted
to select which tracks the element should be added to.

Finally, as a special case, if a track is added to the timeline
*after* it already contains clips, then it will request the creation
of the clips' core elements of the corresponding type, if they have
not already been created, and this signal will be emitted for each of
these newly created elements. In addition, this will also be released
for all other track elements in the timeline's clips that have not
yet been assigned a track. However, in this final case, the timeline
will only check whether the newly added track appears in the track
list. If it does appear, the track element will be added to the newly
added track. All other tracks in the returned track list are ignored.

In this latter case, track elements that are already part of a track
will not be asked if they want to be copied into the new track. If
you wish to do this, you can use `ClipExt::add_child_to_track`.

Note that the returned `glib::PtrArray` should own a new reference to each
of its contained `Track`. The timeline will set the `GDestroyNotify`
free function on the `glib::PtrArray` to dereference the elements.
## `clip`
The clip that `track_element` is being added to
## `track_element`
The element being added

# Returns

An array of
`Track`-s that `track_element` should be added to, or `None` to
not add the element to any track.
<!-- trait TimelineExt::fn connect_snapping_ended -->
Will be emitted whenever a snapping event ends. After a snap event
has started (see `Timeline::snapping-started`), it can later end
because either another timeline edit has occurred (which may or may
not have created a new snapping event), or because the timeline has
been committed.
## `obj1`
The first element that was snapping
## `obj2`
The second element that was snapping
## `position`
The position where the two objects were to be snapped to
<!-- trait TimelineExt::fn connect_snapping_started -->
Will be emitted whenever an element's movement invokes a snapping
event during an edit (usually of one of its ancestors) because its
start or end point lies within the `Timeline:snapping-distance` of
another element's start or end point.

See `EditMode` to see what can snap during an edit.

Note that only up to one snapping-started signal will be emitted per
element edit within a timeline.
## `obj1`
The first element that is snapping
## `obj2`
The second element that is snapping
## `position`
The position where the two objects will snap to
<!-- trait TimelineExt::fn connect_track_added -->
Will be emitted after the track is added to the timeline.

Note that this should not be emitted whilst a timeline is being
loaded from its `Project` asset. You should connect to the
project's `Project::loaded` signal if you want to know which
tracks were created for the timeline.
## `track`
The track that was added to `timeline`
<!-- trait TimelineExt::fn connect_track_removed -->
Will be emitted after the track is removed from the timeline.
## `track`
The track that was removed from `timeline`
<!-- trait TimelineExt::fn get_property_auto_transition -->
Whether to automatically create a transition whenever two
`Source`-s overlap in a track of the timeline. See
`Layer:auto-transition` if you want this to only happen in some
layers.
<!-- trait TimelineExt::fn set_property_auto_transition -->
Whether to automatically create a transition whenever two
`Source`-s overlap in a track of the timeline. See
`Layer:auto-transition` if you want this to only happen in some
layers.
<!-- trait TimelineExt::fn get_property_duration -->
The current duration (in nanoseconds) of the timeline. A timeline
'starts' at time 0, so this is the maximum end time of all of its
`TimelineElement`-s.
<!-- trait TimelineExt::fn get_property_snapping_distance -->
The distance (in nanoseconds) at which a `TimelineElement` being
moved within the timeline should snap one of its `Source`-s with
another `Source`-s edge. See `EditMode` for which edges can
snap during an edit. 0 means no snapping.
<!-- trait TimelineExt::fn set_property_snapping_distance -->
The distance (in nanoseconds) at which a `TimelineElement` being
moved within the timeline should snap one of its `Source`-s with
another `Source`-s edge. See `EditMode` for which edges can
snap during an edit. 0 means no snapping.
<!-- struct TimelineElement -->
A `TimelineElement` will have some temporal extent in its
corresponding `TimelineElement:timeline`, controlled by its
`TimelineElement:start` and `TimelineElement:duration`. This
determines when its content will be displayed, or its effect applied,
in the timeline. Several objects may overlap within a given
`Timeline`, in which case their `TimelineElement:priority` is used
to determine their ordering in the timeline. Priority is mostly handled
internally by `Layer`-s and `Clip`-s.

A timeline element can have a `TimelineElement:parent`,
such as a `Clip`, which is responsible for controlling its timing.

## Editing

Elements can be moved around in their `TimelineElement:timeline` by
setting their `TimelineElement:start` and
`TimelineElement:duration` using `TimelineElementExt::set_start`
and `TimelineElementExt::set_duration`. Additionally, which parts of
the underlying content are played in the timeline can be adjusted by
setting the `TimelineElement:in-point` using
`TimelineElementExt::set_inpoint`. The library also provides
`TimelineElementExt::edit`, with various `EditMode`-s, which can
adjust these properties in a convenient way, as well as introduce
similar changes in neighbouring or later elements in the timeline.

However, a timeline may refuse a change in these properties if they
would place the timeline in an unsupported configuration. See
`Timeline` for its overlap rules.

Additionally, an edit may be refused if it would place one of the
timing properties out of bounds (such as a negative time value for
`TimelineElement:start`, or having insufficient internal
content to last for the desired `TimelineElement:duration`).

## Time Coordinates

There are three main sets of time coordinates to consider when using
timeline elements:

+ Timeline coordinates: these are the time coordinates used in the
 output of the timeline in its `Track`-s. Each track share the same
 coordinates, so there is only one set of coordinates for the
 timeline. These extend indefinitely from 0. The times used for
 editing (including setting `TimelineElement:start` and
 `TimelineElement:duration`) use these coordinates, since these
 define when an element is present and for how long the element lasts
 for in the timeline.
+ Internal source coordinates: these are the time coordinates used
 internally at the element's output. This is only really defined for
 `TrackElement`-s, where it refers to time coordinates used at the
 final source pad of the wrapped `gst::Element`-s. However, these
 coordinates may also be used in a `Clip` in reference to its
 children. In particular, these are the coordinates used for
 `TimelineElement:in-point` and `TimelineElement:max-duration`.
+ Internal sink coordinates: these are the time coordinates used
 internally at the element's input. A `Source` has no input, so
 these would be undefined. Otherwise, for most `TrackElement`-s
 these will be the same set of coordinates as the internal source
 coordinates because the element does not change the timing
 internally. Only `BaseEffect` can support elements where these
 are different. See `BaseEffect` for more information.

You can determine the timeline time for a given internal source time
in a `Track` in a `Clip` using
`ClipExt::get_timeline_time_from_internal_time`, and vice versa using
`ClipExt::get_internal_time_from_timeline_time`, for the purposes of
editing and setting timings properties.

## Children Properties

If a timeline element owns another `gst::Object` and wishes to expose
some of its properties, it can do so by registering the property as one
of the timeline element's children properties using
`TimelineElementExt::add_child_property`. The registered property of
the child can then be read and set using the
`TimelineElementExt::get_child_property` and
`TimelineElementExt::set_child_property` methods, respectively. Some
sub-classed objects will be created with pre-registered children
properties; for example, to expose part of an underlying `gst::Element`
that is used internally. The registered properties can be listed with
`TimelineElementExt::list_children_properties`.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait TimelineElementExt -->
Trait containing all `TimelineElement` methods.

# Implementors

[`Container`](struct@crate::Container), [`TimelineElement`](struct@crate::TimelineElement), [`TrackElement`](struct@crate::TrackElement)
<!-- trait TimelineElementExt::fn add_child_property -->
Register a property of a child of the element to allow it to be
written with `TimelineElementExt::set_child_property` and read with
`TimelineElementExt::get_child_property`. A change in the property
will also appear in the `TimelineElement::deep-notify` signal.

`pspec` should be unique from other children properties that have been
registered on `self`.
## `pspec`
The specification for the property to add
## `child`
The `gst::Object` who the property belongs to

# Returns

`true` if the property was successfully registered.
<!-- trait TimelineElementExt::fn copy -->
Create a copy of `self`. All the properties of `self` are copied into
a new element, with the exception of `TimelineElement:parent`,
`TimelineElement:timeline` and `TimelineElement:name`. Other data,
such the list of a `Container`'s children, is **not** copied.

If `deep` is `true`, then the new element is prepared so that it can be
used in `TimelineElementExt::paste` or `TimelineExt::paste_element`.
In the case of copying a `Container`, this ensures that the children
of `self` will also be pasted. The new element should not be used for
anything else and can only be used **once** in a pasting operation. In
particular, the new element itself is not an actual 'deep' copy of
`self`, but should be thought of as an intermediate object used for a
single paste operation.
## `deep`
Whether the copy is needed for pasting

# Returns

The newly create element,
copied from `self`.
<!-- trait TimelineElementExt::fn edit -->
See `TimelineElementExt::edit_full`, which also gives an error.

Note that the `layers` argument is currently ignored, so you should
just pass `None`.

Feature: `v1_18`

## `layers`
A whitelist of layers
where the edit can be performed, `None` allows all layers in the
timeline.
## `new_layer_priority`
The priority/index of the layer `self` should be
moved to. -1 means no move
## `mode`
The edit mode
## `edge`
The edge of `self` where the edit should occur
## `position`
The edit position: a new location for the edge of `self`
(in nanoseconds) in the timeline coordinates

# Returns

`true` if the edit of `self` completed, `false` on failure.
<!-- trait TimelineElementExt::fn edit_full -->
Edits the element within its timeline by adjusting its
`TimelineElement:start`, `TimelineElement:duration` or
`TimelineElement:in-point`, and potentially doing the same for
other elements in the timeline. See `EditMode` for details about each
edit mode. An edit may fail if it would place one of these properties
out of bounds, or if it would place the timeline in an unsupported
configuration.

Note that if you act on a `TrackElement`, this will edit its parent
`Clip` instead. Moreover, for any `TimelineElement`, if you select
`Edge::None` for `EditMode::Normal` or `EditMode::Ripple`, this
will edit the toplevel instead, but still in such a way as to make the
`TimelineElement:start` of `self` reach the edit `position`.

Note that if the element's timeline has a
`Timeline:snapping-distance` set, then the edit position may be
snapped to the edge of some element under the edited element.

`new_layer_priority` can be used to switch `self`, and other elements
moved by the edit, to a new layer. New layers may be be created if the
the corresponding layer priority/index does not yet exist for the
timeline.

Feature: `v1_18`

## `new_layer_priority`
The priority/index of the layer `self` should be
moved to. -1 means no move
## `mode`
The edit mode
## `edge`
The edge of `self` where the edit should occur
## `position`
The edit position: a new location for the edge of `self`
(in nanoseconds) in the timeline coordinates

# Returns

`true` if the edit of `self` completed, `false` on failure.
<!-- trait TimelineElementExt::fn get_child_properties -->
Gets several of the children properties of the element. See
`TimelineElementExt::get_child_property`.
## `first_property_name`
The name of the first child property to get
<!-- trait TimelineElementExt::fn get_child_property -->
Gets the property of a child of the element.

`property_name` can either be in the format "prop-name" or
"TypeName::prop-name", where "prop-name" is the name of the property
to get (as used in `glib::object::ObjectExt::get`), and "TypeName" is the type name of
the child (as returned by G_OBJECT_TYPE_NAME()). The latter format is
useful when two children of different types share the same property
name.

The first child found with the given "prop-name" property that was
registered with `TimelineElementExt::add_child_property` (and of the
type "TypeName", if it was given) will have the corresponding
property copied into `value`.

Note that `TimelineElementExt::get_child_properties` may be more
convenient for C programming.
## `property_name`
The name of the child property to get
## `value`
The return location for the value

# Returns

`true` if the property was found and copied to `value`.
<!-- trait TimelineElementExt::fn get_child_property_by_pspec -->
Gets the property of a child of the element. Specifically, the property
corresponding to the `pspec` used in
`TimelineElementExt::add_child_property` is copied into `value`.
## `pspec`
The specification of a registered child property to get
## `value`
The return location for the value
<!-- trait TimelineElementExt::fn get_child_property_valist -->
Gets several of the children properties of the element. See
`TimelineElementExt::get_child_property`.
## `first_property_name`
The name of the first child property to get
## `var_args`
The return location for the first property, followed
optionally by more name/return location pairs, followed by `None`
<!-- trait TimelineElementExt::fn duration -->
Gets the `TimelineElement:duration` for the element.

# Returns

The duration of `self` (in nanoseconds).
<!-- trait TimelineElementExt::fn inpoint -->
Gets the `TimelineElement:in-point` for the element.

# Returns

The in-point of `self` (in nanoseconds).
<!-- trait TimelineElementExt::fn layer_priority -->
Gets the priority of the layer the element is in. A `Group` may span
several layers, so this would return the highest priority (numerically,
the smallest) amongst them.

Feature: `v1_16`


# Returns

The priority of the layer `self` is in, or
`GES_TIMELINE_ELEMENT_NO_LAYER_PRIORITY` if `self` does not exist in a
layer.
<!-- trait TimelineElementExt::fn max_duration -->
Gets the `TimelineElement:max-duration` for the element.

# Returns

The max-duration of `self` (in nanoseconds).
<!-- trait TimelineElementExt::fn name -->
Gets the `TimelineElement:name` for the element.

# Returns

The name of `self`.
<!-- trait TimelineElementExt::fn natural_framerate -->
Get the "natural" framerate of `self`. This is to say, for example
for a `VideoUriSource` the framerate of the source.

Note that a `AudioSource` may also have a natural framerate if it derives
from the same `SourceClip` asset as a `VideoSource`, and its value will
be that of the video source. For example, if the uri of a `UriClip` points
to a file that contains both a video and audio stream, then the corresponding
`AudioUriSource` will share the natural framerate of the corresponding
`VideoUriSource`.

Feature: `v1_18`

## `framerate_n`
The framerate numerator
## `framerate_d`
The framerate denominator

# Returns

Whether `self` has a natural framerate or not, `framerate_n`
and `framerate_d` will be set to, respectively, 0 and -1 if it is
not the case.
<!-- trait TimelineElementExt::fn parent -->
Gets the `TimelineElement:parent` for the element.

# Returns

The parent of `self`, or `None` if
`self` has no parent.
<!-- trait TimelineElementExt::fn priority -->
Gets the `TimelineElement:priority` for the element.

# Returns

The priority of `self`.
<!-- trait TimelineElementExt::fn start -->
Gets the `TimelineElement:start` for the element.

# Returns

The start of `self` (in nanoseconds).
<!-- trait TimelineElementExt::fn timeline -->
Gets the `TimelineElement:timeline` for the element.

# Returns

The timeline of `self`, or `None`
if `self` has no timeline.
<!-- trait TimelineElementExt::fn toplevel_parent -->
Gets the toplevel `TimelineElement:parent` of the element.

# Returns

The toplevel parent of `self`.
<!-- trait TimelineElementExt::fn track_types -->
Gets the track types that the element can interact with, i.e. the type
of `Track` it can exist in, or will create `TrackElement`-s for.

# Returns

The track types that `self` supports.
<!-- trait TimelineElementExt::fn list_children_properties -->
Get a list of children properties of the element, which is a list of
all the specifications passed to
`TimelineElementExt::add_child_property`.
## `n_properties`
The return location for the length of the
returned array

# Returns

An array of
`glib::object::ParamSpec` corresponding to the child properties of `self`, or `None` if
something went wrong.
<!-- trait TimelineElementExt::fn lookup_child -->
Looks up a child property of the element.

`prop_name` can either be in the format "prop-name" or
"TypeName::prop-name", where "prop-name" is the name of the property
to look up (as used in `glib::object::ObjectExt::get`), and "TypeName" is the type name
of the child (as returned by G_OBJECT_TYPE_NAME()). The latter format is
useful when two children of different types share the same property
name.

The first child found with the given "prop-name" property that was
registered with `TimelineElementExt::add_child_property` (and of the
type "TypeName", if it was given) will be passed to `child`, and the
registered specification of this property will be passed to `pspec`.
## `prop_name`
The name of a child property
## `child`
The return location for the
found child
## `pspec`
The return location for the
specification of the child property

# Returns

`true` if a child corresponding to the property was found, in
which case `child` and `pspec` are set.
<!-- trait TimelineElementExt::fn paste -->
Paste an element inside the same timeline and layer as `self`. `self`
**must** be the return of `TimelineElementExt::copy` with `deep=TRUE`,
and it should not be changed before pasting.
`self` is not placed in the timeline, instead a new element is created,
alike to the originally copied element. Note that the originally
copied element must stay within the same timeline and layer, at both
the point of copying and pasting.

Pasting may fail if it would place the timeline in an unsupported
configuration.

After calling this function `element` should not be used. In particular,
`element` can **not** be pasted again. Instead, you can copy the
returned element and paste that copy (although, this is only possible
if the paste was successful).

See also `TimelineExt::paste_element`.
## `paste_position`
The position in the timeline `element` should be pasted
to, i.e. the `TimelineElement:start` value for the pasted element.

# Returns

The newly created element, or
`None` if pasting fails.
<!-- trait TimelineElementExt::fn remove_child_property -->
Remove a child property from the element. `pspec` should be a
specification that was passed to
`TimelineElementExt::add_child_property`. The corresponding property
will no longer be registered as a child property for the element.
## `pspec`
The specification for the property to remove

# Returns

`true` if the property was successfully un-registered for `self`.
<!-- trait TimelineElementExt::fn ripple -->
Edits the start time of an element within its timeline in ripple mode.
See `TimelineElementExt::edit` with `EditMode::Ripple` and
`Edge::None`.
## `start`
The new start time of `self` in ripple mode

# Returns

`true` if the ripple edit of `self` completed, `false` on
failure.
<!-- trait TimelineElementExt::fn ripple_end -->
Edits the end time of an element within its timeline in ripple mode.
See `TimelineElementExt::edit` with `EditMode::Ripple` and
`Edge::End`.
## `end`
The new end time of `self` in ripple mode

# Returns

`true` if the ripple edit of `self` completed, `false` on
failure.
<!-- trait TimelineElementExt::fn roll_end -->
Edits the end time of an element within its timeline in roll mode.
See `TimelineElementExt::edit` with `EditMode::Roll` and
`Edge::End`.
## `end`
The new end time of `self` in roll mode

# Returns

`true` if the roll edit of `self` completed, `false` on failure.
<!-- trait TimelineElementExt::fn roll_start -->
Edits the start time of an element within its timeline in roll mode.
See `TimelineElementExt::edit` with `EditMode::Roll` and
`Edge::Start`.
## `start`
The new start time of `self` in roll mode

# Returns

`true` if the roll edit of `self` completed, `false` on failure.
<!-- trait TimelineElementExt::fn set_child_properties -->
Sets several of the children properties of the element. See
`TimelineElementExt::set_child_property`.
## `first_property_name`
The name of the first child property to set
<!-- trait TimelineElementExt::fn set_child_property -->
See `TimelineElementExt::set_child_property_full`, which also gives an
error.

Note that `TimelineElementExt::set_child_properties` may be more
convenient for C programming.
## `property_name`
The name of the child property to set
## `value`
The value to set the property to

# Returns

`true` if the property was found and set.
<!-- trait TimelineElementExt::fn set_child_property_by_pspec -->
Sets the property of a child of the element. Specifically, the property
corresponding to the `pspec` used in
`TimelineElementExt::add_child_property` is set to `value`.
## `pspec`
The specification of a registered child property to set
## `value`
The value to set the property to
<!-- trait TimelineElementExt::fn set_child_property_full -->
Sets the property of a child of the element.

`property_name` can either be in the format "prop-name" or
"TypeName::prop-name", where "prop-name" is the name of the property
to set (as used in `glib::object::ObjectExt::set`), and "TypeName" is the type name of
the child (as returned by G_OBJECT_TYPE_NAME()). The latter format is
useful when two children of different types share the same property
name.

The first child found with the given "prop-name" property that was
registered with `TimelineElementExt::add_child_property` (and of the
type "TypeName", if it was given) will have the corresponding
property set to `value`. Other children that may have also matched the
property name (and type name) are left unchanged!

Feature: `v1_18`

## `property_name`
The name of the child property to set
## `value`
The value to set the property to

# Returns

`true` if the property was found and set.
<!-- trait TimelineElementExt::fn set_child_property_valist -->
Sets several of the children properties of the element. See
`TimelineElementExt::set_child_property`.
## `first_property_name`
The name of the first child property to set
## `var_args`
The value for the first property, followed optionally by more
name/value pairs, followed by `None`
<!-- trait TimelineElementExt::fn set_duration -->
Sets `TimelineElement:duration` for the element.

Whilst the element is part of a `Timeline`, this is the same as
editing the element with `TimelineElementExt::edit` under
`EditMode::Trim` with `Edge::End`. In particular, the
`TimelineElement:duration` of the element may be snapped to a
different timeline time difference from the one given. In addition,
setting may fail if it would place the timeline in an unsupported
configuration, or the element does not have enough internal content to
last the desired duration.
## `duration`
The desired duration in its timeline

# Returns

`true` if `duration` could be set for `self`.
<!-- trait TimelineElementExt::fn set_inpoint -->
Sets `TimelineElement:in-point` for the element. If the new in-point
is above the current `TimelineElement:max-duration` of the element,
this method will fail.
## `inpoint`
The in-point, in internal time coordinates

# Returns

`true` if `inpoint` could be set for `self`.
<!-- trait TimelineElementExt::fn set_max_duration -->
Sets `TimelineElement:max-duration` for the element. If the new
maximum duration is below the current `TimelineElement:in-point` of
the element, this method will fail.
## `maxduration`
The maximum duration, in internal time coordinates

# Returns

`true` if `maxduration` could be set for `self`.
<!-- trait TimelineElementExt::fn set_name -->
Sets the `TimelineElement:name` for the element. If `None` is given
for `name`, then the library will instead generate a new name based on
the type name of the element, such as the name "uriclip3" for a
`UriClip`, and will set that name instead.

If `self` already has a `TimelineElement:timeline`, you should not
call this function with `name` set to `None`.

You should ensure that, within each `Timeline`, every element has a
unique name. If you call this function with `name` as `None`, then
the library should ensure that the set generated name is unique from
previously **generated** names. However, if you choose a `name` that
interferes with the naming conventions of the library, the library will
attempt to ensure that the generated names will not conflict with the
chosen name, which may lead to a different name being set instead, but
the uniqueness between generated and user-chosen names is not
guaranteed.
## `name`
The name `self` should take

# Returns

`true` if `name` or a generated name for `self` could be set.
<!-- trait TimelineElementExt::fn set_parent -->
Sets the `TimelineElement:parent` for the element.

This is used internally and you should normally not call this. A
`Container` will set the `TimelineElement:parent` of its children
in `GESContainerExt::add` and `GESContainerExt::remove`.

Note, if `parent` is not `None`, `self` must not already have a parent
set. Therefore, if you wish to switch parents, you will need to call
this function twice: first to set the parent to `None`, and then to the
new parent.

If `parent` is not `None`, you must ensure it already has a
(non-floating) reference to `self` before calling this.

# Returns

`true` if `parent` could be set for `self`.
<!-- trait TimelineElementExt::fn set_priority -->
Sets the priority of the element within the containing layer.

# Deprecated since 1.10

All priority management is done by GES itself now.
To set `Effect` priorities `ClipExt::set_top_effect_index` should
be used.
## `priority`
The priority

# Returns

`true` if `priority` could be set for `self`.
<!-- trait TimelineElementExt::fn set_start -->
Sets `TimelineElement:start` for the element. If the element has a
parent, this will also move its siblings with the same shift.

Whilst the element is part of a `Timeline`, this is the same as
editing the element with `TimelineElementExt::edit` under
`EditMode::Normal` with `Edge::None`. In particular, the
`TimelineElement:start` of the element may be snapped to a different
timeline time from the one given. In addition, setting may fail if it
would place the timeline in an unsupported configuration.
## `start`
The desired start position of the element in its timeline

# Returns

`true` if `start` could be set for `self`.
<!-- trait TimelineElementExt::fn set_timeline -->
Sets the `TimelineElement:timeline` of the element.

This is used internally and you should normally not call this. A
`Clip` will have its `TimelineElement:timeline` set through its
`Layer`. A `Track` will similarly take care of setting the
`TimelineElement:timeline` of its `TrackElement`-s. A `Group`
will adopt the same `TimelineElement:timeline` as its children.

If `timeline` is `None`, this will stop its current
`TimelineElement:timeline` from tracking it, otherwise `timeline` will
start tracking `self`. Note, in the latter case, `self` must not already
have a timeline set. Therefore, if you wish to switch timelines, you
will need to call this function twice: first to set the timeline to
`None`, and then to the new timeline.

# Returns

`true` if `timeline` could be set for `self`.
<!-- trait TimelineElementExt::fn trim -->
Edits the start time of an element within its timeline in trim mode.
See `TimelineElementExt::edit` with `EditMode::Trim` and
`Edge::Start`.
## `start`
The new start time of `self` in trim mode

# Returns

`true` if the trim edit of `self` completed, `false` on failure.
<!-- trait TimelineElementExt::fn connect_child_property_added -->
Emitted when the element has a new child property registered. See
`TimelineElementExt::add_child_property`.

Note that some GES elements will be automatically created with
pre-registered children properties. You can use
`TimelineElementExt::list_children_properties` to list these.

Feature: `v1_18`

## `prop_object`
The child whose property has been registered
## `prop`
The specification for the property that has been registered
<!-- trait TimelineElementExt::fn connect_child_property_removed -->
Emitted when the element has a child property unregistered. See
`TimelineElementExt::remove_child_property`.

Feature: `v1_18`

## `prop_object`
The child whose property has been unregistered
## `prop`
The specification for the property that has been unregistered
<!-- trait TimelineElementExt::fn connect_deep_notify -->
Emitted when a child of the element has one of its registered
properties set. See `TimelineElementExt::add_child_property`.
Note that unlike `glib::object::Object::notify`, a child property name can not be
used as a signal detail.
## `prop_object`
The child whose property has been set
## `prop`
The specification for the property that been set
<!-- trait TimelineElementExt::fn get_property_duration -->
The duration that the element is in effect for in the timeline (a
time difference in nanoseconds using the time coordinates of the
timeline). For example, for a source element, this would determine
for how long it should output its internal content for. For an
operation element, this would determine for how long its effect
should be applied to any source content.
<!-- trait TimelineElementExt::fn set_property_duration -->
The duration that the element is in effect for in the timeline (a
time difference in nanoseconds using the time coordinates of the
timeline). For example, for a source element, this would determine
for how long it should output its internal content for. For an
operation element, this would determine for how long its effect
should be applied to any source content.
<!-- trait TimelineElementExt::fn get_property_in_point -->
The initial offset to use internally when outputting content (in
nanoseconds, but in the time coordinates of the internal content).

For example, for a `VideoUriSource` that references some media
file, the "internal content" is the media file data, and the
in-point would correspond to some timestamp in the media file.
When playing the timeline, and when the element is first reached at
timeline-time `TimelineElement:start`, it will begin outputting the
data from the timestamp in-point **onwards**, until it reaches the
end of its `TimelineElement:duration` in the timeline.

For elements that have no internal content, this should be kept
as 0.
<!-- trait TimelineElementExt::fn set_property_in_point -->
The initial offset to use internally when outputting content (in
nanoseconds, but in the time coordinates of the internal content).

For example, for a `VideoUriSource` that references some media
file, the "internal content" is the media file data, and the
in-point would correspond to some timestamp in the media file.
When playing the timeline, and when the element is first reached at
timeline-time `TimelineElement:start`, it will begin outputting the
data from the timestamp in-point **onwards**, until it reaches the
end of its `TimelineElement:duration` in the timeline.

For elements that have no internal content, this should be kept
as 0.
<!-- trait TimelineElementExt::fn get_property_max_duration -->
The full duration of internal content that is available (a time
difference in nanoseconds using the time coordinates of the internal
content).

This will act as a cap on the `TimelineElement:in-point` of the
element (which is in the same time coordinates), and will sometimes
be used to limit the `TimelineElement:duration` of the element in
the timeline.

For example, for a `VideoUriSource` that references some media
file, this would be the length of the media file.

For elements that have no internal content, or whose content is
indefinite, this should be kept as `GST_CLOCK_TIME_NONE`.
<!-- trait TimelineElementExt::fn set_property_max_duration -->
The full duration of internal content that is available (a time
difference in nanoseconds using the time coordinates of the internal
content).

This will act as a cap on the `TimelineElement:in-point` of the
element (which is in the same time coordinates), and will sometimes
be used to limit the `TimelineElement:duration` of the element in
the timeline.

For example, for a `VideoUriSource` that references some media
file, this would be the length of the media file.

For elements that have no internal content, or whose content is
indefinite, this should be kept as `GST_CLOCK_TIME_NONE`.
<!-- trait TimelineElementExt::fn get_property_name -->
The name of the element. This should be unique within its timeline.
<!-- trait TimelineElementExt::fn set_property_name -->
The name of the element. This should be unique within its timeline.
<!-- trait TimelineElementExt::fn get_property_parent -->
The parent container of the element.
<!-- trait TimelineElementExt::fn set_property_parent -->
The parent container of the element.
<!-- trait TimelineElementExt::fn get_property_priority -->
The priority of the element.

# Deprecated since 1.10

Priority management is now done by GES itself.
<!-- trait TimelineElementExt::fn set_property_priority -->
The priority of the element.

# Deprecated since 1.10

Priority management is now done by GES itself.
<!-- trait TimelineElementExt::fn get_property_serialize -->
Whether the element should be serialized.
<!-- trait TimelineElementExt::fn set_property_serialize -->
Whether the element should be serialized.
<!-- trait TimelineElementExt::fn get_property_start -->
The starting position of the element in the timeline (in nanoseconds
and in the time coordinates of the timeline). For example, for a
source element, this would determine the time at which it should
start outputting its internal content. For an operation element, this
would determine the time at which it should start applying its effect
to any source content.
<!-- trait TimelineElementExt::fn set_property_start -->
The starting position of the element in the timeline (in nanoseconds
and in the time coordinates of the timeline). For example, for a
source element, this would determine the time at which it should
start outputting its internal content. For an operation element, this
would determine the time at which it should start applying its effect
to any source content.
<!-- trait TimelineElementExt::fn get_property_timeline -->
The timeline that the element lies within.
<!-- trait TimelineElementExt::fn set_property_timeline -->
The timeline that the element lies within.
<!-- struct Track -->
A `Track` acts an output source for a `Timeline`. Each one
essentially provides an additional `gst::Pad` for the timeline, with
`Track:restriction-caps` capabilities. Internally, a track
wraps an `nlecomposition` filtered by a `capsfilter`.

A track will contain a number of `TrackElement`-s, and its role is
to select and activate these elements according to their timings when
the timeline in played. For example, a track would activate a
`Source` when its `TimelineElement:start` is reached by outputting
its data for its `TimelineElement:duration`. Similarly, a
`Operation` would be activated by applying its effect to the source
data, starting from its `TimelineElement:start` time and lasting for
its `TimelineElement:duration`.

For most users, it will usually be sufficient to add newly created
tracks to a timeline, but never directly add an element to a track.
Whenever a `Clip` is added to a timeline, the clip adds its
elements to the timeline's tracks and assumes responsibility for
updating them.

# Implements

[`GESTrackExt`](trait@crate::GESTrackExt), [`trait@gst::ElementExt`], [`trait@gst::ObjectExt`], [`trait@glib::object::ObjectExt`]
<!-- trait GESTrackExt -->
Trait containing all `Track` methods.

# Implementors

[`Track`](struct@crate::Track)
<!-- impl Track::fn new -->
Creates a new track with the given track-type and caps.

If `type_` is `TrackType::Video`, and `caps` is a subset of
"video/x-raw(ANY)", then a `VideoTrack` is created. This will
automatically choose a gap creation method suitable for video data. You
will likely want to set `Track:restriction-caps` separately. You may
prefer to use the `VideoTrack::new` method instead.

If `type_` is `TrackType::Audio`, and `caps` is a subset of
"audio/x-raw(ANY)", then a `AudioTrack` is created. This will
automatically choose a gap creation method suitable for audio data, and
will set the `Track:restriction-caps` to the default for
`AudioTrack`. You may prefer to use the `AudioTrack::new` method
instead.

Otherwise, a plain `Track` is returned. You will likely want to set
the `Track:restriction-caps` and call
`GESTrackExt::set_create_element_for_gap_func` on the returned track.
## `type_`
The `Track:track-type` for the track
## `caps`
The `Track:caps` for the track

# Returns

A new track.
<!-- trait GESTrackExt::fn add_element -->
See `GESTrackExt::add_element`, which also gives an error.
## `object`
The element to add

# Returns

`true` if `object` was successfully added to `self`.
<!-- trait GESTrackExt::fn add_element_full -->
Adds the given track element to the track, which takes ownership of the
element.

Note that this can fail if it would break a configuration rule of the
track's `Timeline`.

Note that a `TrackElement` can only be added to one track.

Feature: `v1_18`

## `object`
The element to add

# Returns

`true` if `object` was successfully added to `self`.
<!-- trait GESTrackExt::fn commit -->
Commits all the pending changes for the elements contained in the
track.

When changes are made to the timing or priority of elements within a
track, they are not directly executed for the underlying
`nlecomposition` and its children. This method will finally execute
these changes so they are reflected in the data output of the track.

Any pending changes will be executed in the backend. The
`Timeline::commited` signal will be emitted once this has completed.

Note that `TimelineExt::commit` will call this method on all of its
tracks, so you are unlikely to need to use this directly.

# Returns

`true` if pending changes were committed, or `false` if nothing
needed to be committed.
<!-- trait GESTrackExt::fn caps -->
Get the `Track:caps` of the track.

# Returns

The caps of `self`.
<!-- trait GESTrackExt::fn elements -->
Gets the track elements contained in the track. The returned list is
sorted by the element's `TimelineElement:priority` and
`TimelineElement:start`.

# Returns

A list of
all the `TrackElement`-s in `self`.
<!-- trait GESTrackExt::fn is_mixing -->
Gets the `Track:mixing` of the track.

# Returns

Whether `self` is mixing.
<!-- trait GESTrackExt::fn restriction_caps -->
Gets the `Track:restriction-caps` of the track.

Feature: `v1_18`


# Returns

The restriction-caps of `self`.
<!-- trait GESTrackExt::fn timeline -->
Get the timeline this track belongs to.

# Returns

The timeline that `self` belongs to, or `None` if
it does not belong to a timeline.
<!-- trait GESTrackExt::fn remove_element -->
See `GESTrackExt::remove_element_full`, which also returns an error.
## `object`
The element to remove

# Returns

`true` if `object` was successfully removed from `self`.
<!-- trait GESTrackExt::fn remove_element_full -->
Removes the given track element from the track, which revokes
ownership of the element.

Feature: `v1_18`

## `object`
The element to remove

# Returns

`true` if `object` was successfully removed from `self`.
<!-- trait GESTrackExt::fn set_create_element_for_gap_func -->
Sets the function that will be used to create a `gst::Element` that can be
used as a source to fill the gaps of the track. A gap is a timeline
region where the track has no `TrackElement` sources. Therefore, you
are likely to want the `gst::Element` returned by the function to always
produce 'empty' content, defined relative to the stream type, such as
transparent frames for a video, or mute samples for audio.

`AudioTrack` and `VideoTrack` objects are created with such a
function already set appropriately.
## `func`
The function to be used to create a source
`gst::Element` that can fill gaps in `self`
<!-- trait GESTrackExt::fn set_mixing -->
Sets the `Track:mixing` for the track.
## `mixing`
Whether `self` should be mixing
<!-- trait GESTrackExt::fn set_restriction_caps -->
Sets the `Track:restriction-caps` for the track.

> **NOTE**: Restriction caps are **not** taken into account when
> using `Pipeline:mode`=`PipelineFlags::SmartRender`.
## `caps`
The new restriction-caps for `self`
<!-- trait GESTrackExt::fn set_timeline -->
Informs the track that it belongs to the given timeline. Calling this
does not actually add the track to the timeline. For that, you should
use `TimelineExt::add_track`, which will also take care of informing
the track that it belongs to the timeline. As such, there is no need
for you to call this method.
<!-- trait GESTrackExt::fn update_restriction_caps -->
Updates the `Track:restriction-caps` of the track using the fields
found in the given caps. Each of the `gst::Structure`-s in `caps` is
compared against the existing structure with the same index in the
current `Track:restriction-caps`. If there is no corresponding
existing structure at that index, then the new structure is simply
copied to that index. Otherwise, any fields in the new structure are
copied into the existing structure. This will replace existing values,
and may introduce new ones, but any fields 'missing' in the new
structure are left unchanged in the existing structure.

For example, if the existing `Track:restriction-caps` are
"video/x-raw, width=480, height=360", and the updating caps is
"video/x-raw, format=I420, width=500; video/x-bayer, width=400", then
the new `Track:restriction-caps` after calling this will be
"video/x-raw, width=500, height=360, format=I420; video/x-bayer,
width=400".
## `caps`
The caps to update the restriction-caps with
<!-- trait GESTrackExt::fn connect_commited -->
This signal will be emitted once the changes initiated by
`GESTrackExt::commit` have been executed in the backend. In particular,
this will be emitted whenever the underlying `nlecomposition` has been
committed (see `nlecomposition::commited`).
<!-- trait GESTrackExt::fn connect_track_element_added -->
Will be emitted after a track element is added to the track.
## `effect`
The element that was added
<!-- trait GESTrackExt::fn connect_track_element_removed -->
Will be emitted after a track element is removed from the track.
## `effect`
The element that was removed
<!-- trait GESTrackExt::fn get_property_caps -->
The capabilities used to choose the output of the `Track`'s
elements. Internally, this is used to select output streams when
several may be available, by determining whether its `gst::Pad` is
compatible (see `NleObject:caps` for `nlecomposition`). As such,
this is used as a weaker indication of the desired output type of the
track, **before** the `Track:restriction-caps` is applied.
Therefore, this should be set to a *generic* superset of the
`Track:restriction-caps`, such as "video/x-raw(ANY)". In addition,
it should match with the track's `Track:track-type`.

Note that when you set this property, the `gst::CapsFeatures` of all its
`gst::Structure`-s will be automatically set to `GST_CAPS_FEATURES_ANY`.

Once a track has been added to a `Timeline`, you should not change
this.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn set_property_caps -->
The capabilities used to choose the output of the `Track`'s
elements. Internally, this is used to select output streams when
several may be available, by determining whether its `gst::Pad` is
compatible (see `NleObject:caps` for `nlecomposition`). As such,
this is used as a weaker indication of the desired output type of the
track, **before** the `Track:restriction-caps` is applied.
Therefore, this should be set to a *generic* superset of the
`Track:restriction-caps`, such as "video/x-raw(ANY)". In addition,
it should match with the track's `Track:track-type`.

Note that when you set this property, the `gst::CapsFeatures` of all its
`gst::Structure`-s will be automatically set to `GST_CAPS_FEATURES_ANY`.

Once a track has been added to a `Timeline`, you should not change
this.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn get_property_duration -->
Current duration of the track

Default value: O
<!-- trait GESTrackExt::fn get_property_id -->
The `nlecomposition:id` of the underlying `nlecomposition`.

Feature: `v1_18`

<!-- trait GESTrackExt::fn set_property_id -->
The `nlecomposition:id` of the underlying `nlecomposition`.

Feature: `v1_18`

<!-- trait GESTrackExt::fn get_property_mixing -->
Whether the track should support the mixing of `Layer` data, such
as composing the video data of each layer (when part of the video
data is transparent, the next layer will become visible) or adding
together the audio data. As such, for audio and video tracks, you'll
likely want to keep this set to `true`.
<!-- trait GESTrackExt::fn set_property_mixing -->
Whether the track should support the mixing of `Layer` data, such
as composing the video data of each layer (when part of the video
data is transparent, the next layer will become visible) or adding
together the audio data. As such, for audio and video tracks, you'll
likely want to keep this set to `true`.
<!-- trait GESTrackExt::fn get_property_restriction_caps -->
The capabilities that specifies the final output format of the
`Track`. For example, for a video track, it would specify the
height, width, framerate and other properties of the stream.

You may change this property after the track has been added to a
`Timeline`, but it must remain compatible with the track's
`Track:caps`.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn set_property_restriction_caps -->
The capabilities that specifies the final output format of the
`Track`. For example, for a video track, it would specify the
height, width, framerate and other properties of the stream.

You may change this property after the track has been added to a
`Timeline`, but it must remain compatible with the track's
`Track:caps`.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn get_property_track_type -->
The track type of the track. This controls the type of
`TrackElement`-s that can be added to the track. This should
match with the track's `Track:caps`.

Once a track has been added to a `Timeline`, you should not change
this.
<!-- trait GESTrackExt::fn set_property_track_type -->
The track type of the track. This controls the type of
`TrackElement`-s that can be added to the track. This should
match with the track's `Track:caps`.

Once a track has been added to a `Timeline`, you should not change
this.
<!-- struct TrackElement -->
A `TrackElement` is a `TimelineElement` that specifically belongs
to a single `Track` of its `TimelineElement:timeline`. Its
`TimelineElement:start` and `TimelineElement:duration` specify its
temporal extent in the track. Specifically, a track element wraps some
nleobject, such as an `nlesource` or `nleoperation`, which can be
retrieved with `TrackElementExt::get_nleobject`, and its
`TimelineElement:start`, `TimelineElement:duration`,
`TimelineElement:in-point`, `TimelineElement:priority` and
`TrackElement:active` properties expose the corresponding nleobject
properties. When a track element is added to a track, its nleobject is
added to the corresponding `nlecomposition` that the track wraps.

Most users will not have to work directly with track elements since a
`Clip` will automatically create track elements for its timeline's
tracks and take responsibility for updating them. The only track
elements that are not automatically created by clips, but a user is
likely to want to create, are `Effect`-s.

## Control Bindings for Children Properties

You can set up control bindings for a track element child property
using `TrackElementExt::set_control_source`. A
`GstTimedValueControlSource` should specify the timed values using the
internal source coordinates (see `TimelineElement`). By default,
these will be updated to lie between the `TimelineElement:in-point`
and out-point of the element. This can be switched off by setting
`TrackElement:auto-clamp-control-sources` to `false`.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`TrackElementExt`](trait@crate::TrackElementExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait TrackElementExt -->
Trait containing all `TrackElement` methods.

# Implementors

[`TrackElement`](struct@crate::TrackElement)
<!-- trait TrackElementExt::fn add_children_props -->
Adds all the properties of a `gst::Element` that match the criteria as
children properties of the track element. If the name of `element`'s
`gst::ElementFactory` is not in `blacklist`, and the factory's
`GST_ELEMENT_METADATA_KLASS` contains at least one member of
`wanted_categories` (e.g. `GST_ELEMENT_FACTORY_KLASS_DECODER`), then
all the properties of `element` that are also in `whitelist` are added as
child properties of `self` using
`TimelineElementExt::add_child_property`.

This is intended to be used by subclasses when constructing.
## `element`
The child object to retrieve properties from
## `wanted_categories`

An array of element factory "klass" categories to whitelist, or `None`
to accept all categories
## `blacklist`
A
blacklist of element factory names, or `None` to not blacklist any
element factory
## `whitelist`
A
whitelist of element property names, or `None` to whitelist all
writeable properties
<!-- trait TrackElementExt::fn clamp_control_source -->
Clamp the `GstTimedValueControlSource` for the specified child property
to lie between the `TimelineElement:in-point` and out-point of the
element. The out-point is the `GES_TIMELINE_ELEMENT_END` of the element
translated from the timeline coordinates to the internal source
coordinates of the element.

If the property does not have a `GstTimedValueControlSource` set by
`TrackElementExt::set_control_source`, nothing happens. Otherwise, if
a timed value for the control source lies before the in-point of the
element, or after its out-point, then it will be removed. At the
in-point and out-point times, a new interpolated value will be placed.

Feature: `v1_18`

## `property_name`
The name of the child property to clamp the control
source of
<!-- trait TrackElementExt::fn edit -->
Edits the element within its track.

# Deprecated since 1.18

use `TimelineElementExt::edit` instead.
## `layers`
A whitelist of layers
where the edit can be performed, `None` allows all layers in the
timeline
## `mode`
The edit mode
## `edge`
The edge of `self` where the edit should occur
## `position`
The edit position: a new location for the edge of `self`
(in nanoseconds)

# Returns

`true` if the edit of `self` completed, `false` on failure.
<!-- trait TrackElementExt::fn all_control_bindings -->
Get all the control bindings that have been created for the children
properties of the track element using
`TrackElementExt::set_control_source`. The keys used in the returned
hash table are the child property names that were passed to
`TrackElementExt::set_control_source`, and their values are the
corresponding created `gst::ControlBinding`.

# Returns

A
hash table containing all child-property-name/control-binding pairs
for `self`.
<!-- trait TrackElementExt::fn is_auto_clamp_control_sources -->
Gets `TrackElement:auto-clamp-control-sources`.

Feature: `v1_18`


# Returns

Whether the control sources for the child properties of
`self` are automatically clamped.
<!-- trait TrackElementExt::fn get_child_properties -->
Gets properties of a child of `self`.

# Deprecated

Use `TimelineElementExt::get_child_properties`
## `first_property_name`
The name of the first property to get
<!-- trait TrackElementExt::fn get_child_property -->
In general, a copy is made of the property contents and
the caller is responsible for freeing the memory by calling
`glib::object::Value::unset`.

Gets a property of a GstElement contained in `self`.

Note that `TrackElement::get_child_property` is really
intended for language bindings, `TrackElement::get_child_properties`
is much more convenient for C programming.

# Deprecated

Use `TimelineElementExt::get_child_property`
## `property_name`
The name of the property
## `value`
return location for the property value, it will
be initialized if it is initialized with 0

# Returns

`true` if the property was found, `false` otherwise.
<!-- trait TrackElementExt::fn get_child_property_by_pspec -->
Gets a property of a child of `self`.

# Deprecated

Use `TimelineElementExt::get_child_property_by_pspec`
## `pspec`
The `glib::object::ParamSpec` that specifies the property you want to get
## `value`
return location for the value
<!-- trait TrackElementExt::fn get_child_property_valist -->
Gets a property of a child of `self`. If there are various child elements
that have the same property name, you can distinguish them using the following
syntax: 'ClasseName::property_name' as property name. If you don't, the
corresponding property of the first element found will be set.

# Deprecated

Use `TimelineElementExt::get_child_property_valist`
## `first_property_name`
The name of the first property to get
## `var_args`
Value for the first property, followed optionally by more
name/return location pairs, followed by NULL
<!-- trait TrackElementExt::fn get_control_binding -->
Gets the control binding that was created for the specified child
property of the track element using
`TrackElementExt::set_control_source`. The given `property_name` must
be the same name of the child property that was passed to
`TrackElementExt::set_control_source`.
## `property_name`
The name of the child property to return the control
binding of

# Returns

The control binding that was
created for the specified child property of `self`, or `None` if
`property_name` does not correspond to any control binding.
<!-- trait TrackElementExt::fn element -->
Get the `gst::Element` that the track element's underlying nleobject
controls.

# Returns

The `gst::Element` being controlled by the
nleobject that `self` wraps.
<!-- trait TrackElementExt::fn gnlobject -->
Get the GNonLin object this object is controlling.

# Deprecated

use `TrackElementExt::get_nleobject` instead.

# Returns

The GNonLin object this object is controlling.
<!-- trait TrackElementExt::fn nleobject -->
Get the nleobject that this element wraps.

# Returns

The nleobject that `self` wraps.
<!-- trait TrackElementExt::fn track -->
Get the `TrackElement:track` for the element.

# Returns

The track that `self` belongs to,
or `None` if it does not belong to a track.
<!-- trait TrackElementExt::fn track_type -->
Gets the `TrackElement:track-type` for the element.

# Returns

The track-type of `self`.
<!-- trait TrackElementExt::fn has_internal_source -->
Gets `TrackElement:has-internal-source` for the element.

Feature: `v1_18`


# Returns

`true` if `self` can have its 'internal time' properties set.
<!-- trait TrackElementExt::fn is_active -->
Gets `TrackElement:active` for the element.

# Returns

`true` if `self` is active in its track.
<!-- trait TrackElementExt::fn is_core -->
Get whether the given track element is a core track element. That is,
it was created by the `create_track_elements` `ClipClass` method for
some `Clip`.

Note that such a track element can only be added to a clip that shares
the same `Asset` as the clip that created it. For example, you are
allowed to move core children between clips that resulted from
`GESContainerExt::ungroup`, but you could not move the core child from a
`UriClip` to a `TitleClip` or another `UriClip` with a different
`UriClip:uri`.

Moreover, if a core track element is added to a clip, it will always be
added as a core child. Therefore, if this returns `true`, then `element`
will be a core child of its parent clip.

Feature: `v1_18`


# Returns

`true` if `element` is a core track element.
<!-- trait TrackElementExt::fn list_children_properties -->
Gets an array of `glib::object::ParamSpec`* for all configurable properties of the
children of `self`.

# Deprecated

Use `TimelineElementExt::list_children_properties`
## `n_properties`
return location for the length of the returned array

# Returns

An array of `glib::object::ParamSpec`* which should be freed after use or
`None` if something went wrong.
<!-- trait TrackElementExt::fn lookup_child -->
Looks up which `element` and `pspec` would be effected by the given `name`. If various
contained elements have this property name you will get the first one, unless you
specify the class name in `name`.

# Deprecated

Use `TimelineElementExt::lookup_child`
## `prop_name`
Name of the property to look up. You can specify the name of the
 class as such: "ClassName::property-name", to guarantee that you get the
 proper GParamSpec in case various GstElement-s contain the same property
 name. If you don't do so, you will get the first element found, having
 this property and the and the corresponding GParamSpec.
## `element`
pointer to a `gst::Element` that
 takes the real object to set property on
## `pspec`
pointer to take the specification
 describing the property

# Returns

TRUE if `element` and `pspec` could be found. FALSE otherwise. In that
case the values for `pspec` and `element` are not modified. Unref `element` after
usage.
<!-- trait TrackElementExt::fn remove_control_binding -->
Removes the `gst::ControlBinding` that was created for the specified child
property of the track element using
`TrackElementExt::set_control_source`. The given `property_name` must
be the same name of the child property that was passed to
`TrackElementExt::set_control_source`.
## `property_name`
The name of the child property to remove the control
binding from

# Returns

`true` if the control binding was removed from the specified
child property of `self`, or `false` if an error occurred.
<!-- trait TrackElementExt::fn set_active -->
Sets `TrackElement:active` for the element.
## `active`
Whether `self` should be active in its track

# Returns

`true` if the property was *toggled*.
<!-- trait TrackElementExt::fn set_auto_clamp_control_sources -->
Sets `TrackElement:auto-clamp-control-sources`. If set to `true`, this
will immediately clamp all the control sources.

Feature: `v1_18`

## `auto_clamp`
Whether to automatically clamp the control sources for the
child properties of `self`
<!-- trait TrackElementExt::fn set_child_properties -->
Sets a property of a child of `self`. If there are various child elements
that have the same property name, you can distinguish them using the following
syntax: 'ClasseName::property_name' as property name. If you don't, the
corresponding property of the first element found will be set.

# Deprecated

Use `TimelineElementExt::set_child_properties`
## `first_property_name`
The name of the first property to set
<!-- trait TrackElementExt::fn set_child_property -->
Sets a property of a GstElement contained in `self`.

Note that `TrackElement::set_child_property` is really
intended for language bindings, `TrackElement::set_child_properties`
is much more convenient for C programming.

# Deprecated

use `TimelineElementExt::set_child_property` instead
## `property_name`
The name of the property
## `value`
The value

# Returns

`true` if the property was set, `false` otherwise.
<!-- trait TrackElementExt::fn set_child_property_by_pspec -->
Sets a property of a child of `self`.

# Deprecated

Use `ges_timeline_element_set_child_property_by_spec`
## `pspec`
The `glib::object::ParamSpec` that specifies the property you want to set
## `value`
The value
<!-- trait TrackElementExt::fn set_child_property_valist -->
Sets a property of a child of `self`. If there are various child elements
that have the same property name, you can distinguish them using the following
syntax: 'ClasseName::property_name' as property name. If you don't, the
corresponding property of the first element found will be set.

# Deprecated

Use `TimelineElementExt::set_child_property_valist`
## `first_property_name`
The name of the first property to set
## `var_args`
Value for the first property, followed optionally by more
name/return location pairs, followed by NULL
<!-- trait TrackElementExt::fn set_control_source -->
Creates a `gst::ControlBinding` for the specified child property of the
track element using the given control source. The given `property_name`
should refer to an existing child property of the track element, as
used in `TimelineElementExt::lookup_child`.

If `binding_type` is "direct", then the control binding is created with
`gst_direct_control_binding_new` using the given control source. If
`binding_type` is "direct-absolute", it is created with
`gst_direct_control_binding_new_absolute` instead.
## `source`
The control source to bind the child property to
## `property_name`
The name of the child property to control
## `binding_type`
The type of binding to create ("direct" or
"direct-absolute")

# Returns

`true` if the specified child property could be bound to
`source`, or `false` if an error occurred.
<!-- trait TrackElementExt::fn set_has_internal_source -->
Sets `TrackElement:has-internal-source` for the element. If this is
set to `false`, this method will also set the
`TimelineElement:in-point` of the element to 0 and its
`TimelineElement:max-duration` to `GST_CLOCK_TIME_NONE`.

Feature: `v1_18`

## `has_internal_source`
Whether the `self` should be allowed to have its
'internal time' properties set.

# Returns

`false` if `has_internal_source` is forbidden for `self` and
`true` in any other case.
<!-- trait TrackElementExt::fn set_track_type -->
Sets the `TrackElement:track-type` for the element.
## `type_`
The new track-type for `self`
<!-- trait TrackElementExt::fn connect_control_binding_added -->
This is emitted when a control binding is added to a child property
of the track element.
## `control_binding`
The control binding that has been added
<!-- trait TrackElementExt::fn connect_control_binding_removed -->
This is emitted when a control binding is removed from a child
property of the track element.
## `control_binding`
The control binding that has been removed
<!-- trait TrackElementExt::fn get_property_active -->
Whether the effect of the element should be applied in its
`TrackElement:track`. If set to `false`, it will not be used in
the output of the track.
<!-- trait TrackElementExt::fn set_property_active -->
Whether the effect of the element should be applied in its
`TrackElement:track`. If set to `false`, it will not be used in
the output of the track.
<!-- trait TrackElementExt::fn get_property_auto_clamp_control_sources -->
Whether the control sources on the element (see
`TrackElementExt::set_control_source`) will be automatically
updated whenever the `TimelineElement:in-point` or out-point of the
element change in value.

See `TrackElementExt::clamp_control_source` for how this is done
per control source.

Default value: `true`

Feature: `v1_18`

<!-- trait TrackElementExt::fn set_property_auto_clamp_control_sources -->
Whether the control sources on the element (see
`TrackElementExt::set_control_source`) will be automatically
updated whenever the `TimelineElement:in-point` or out-point of the
element change in value.

See `TrackElementExt::clamp_control_source` for how this is done
per control source.

Default value: `true`

Feature: `v1_18`

<!-- trait TrackElementExt::fn get_property_has_internal_source -->
This property is used to determine whether the 'internal time'
properties of the element have any meaning. In particular, unless
this is set to `true`, the `TimelineElement:in-point` and
`TimelineElement:max-duration` can not be set to any value other
than the default 0 and `GST_CLOCK_TIME_NONE`, respectively.

If an element has some *internal* *timed* source `gst::Element` that it
reads stream data from as part of its function in a `Track`, then
you'll likely want to set this to `true` to allow the
`TimelineElement:in-point` and `TimelineElement:max-duration` to
be set.

The default value is determined by the `TrackElementClass`
`default_has_internal_source` class property. For most
`SourceClass`-es, this will be `true`, with the exception of those
that have a potentially *static* source, such as `ImageSourceClass`
and `TitleSourceClass`. Otherwise, this will usually be `false`.

For most `Operation`-s you will likely want to leave this set to
`false`. The exception may be for an operation that reads some stream
data from some private internal source as part of manipulating the
input data from the usual linked upstream `TrackElement`.

For example, you may want to set this to `true` for a
`TrackType::Video` operation that wraps a `textoverlay` that reads
from a subtitle file and places its text on top of the received video
data. The `TimelineElement:in-point` of the element would be used
to shift the initial seek time on the `textoverlay` away from 0, and
the `TimelineElement:max-duration` could be set to reflect the
time at which the subtitle file runs out of data.

Note that GES can not support track elements that have both internal
content and manipulate the timing of their data streams (time
effects).

Feature: `v1_18`

<!-- trait TrackElementExt::fn set_property_has_internal_source -->
This property is used to determine whether the 'internal time'
properties of the element have any meaning. In particular, unless
this is set to `true`, the `TimelineElement:in-point` and
`TimelineElement:max-duration` can not be set to any value other
than the default 0 and `GST_CLOCK_TIME_NONE`, respectively.

If an element has some *internal* *timed* source `gst::Element` that it
reads stream data from as part of its function in a `Track`, then
you'll likely want to set this to `true` to allow the
`TimelineElement:in-point` and `TimelineElement:max-duration` to
be set.

The default value is determined by the `TrackElementClass`
`default_has_internal_source` class property. For most
`SourceClass`-es, this will be `true`, with the exception of those
that have a potentially *static* source, such as `ImageSourceClass`
and `TitleSourceClass`. Otherwise, this will usually be `false`.

For most `Operation`-s you will likely want to leave this set to
`false`. The exception may be for an operation that reads some stream
data from some private internal source as part of manipulating the
input data from the usual linked upstream `TrackElement`.

For example, you may want to set this to `true` for a
`TrackType::Video` operation that wraps a `textoverlay` that reads
from a subtitle file and places its text on top of the received video
data. The `TimelineElement:in-point` of the element would be used
to shift the initial seek time on the `textoverlay` away from 0, and
the `TimelineElement:max-duration` could be set to reflect the
time at which the subtitle file runs out of data.

Note that GES can not support track elements that have both internal
content and manipulate the timing of their data streams (time
effects).

Feature: `v1_18`

<!-- trait TrackElementExt::fn get_property_track -->
The track that this element belongs to, or `None` if it does not
belong to a track.
<!-- trait TrackElementExt::fn get_property_track_type -->
The track type of the element, which determines the type of track the
element can be added to (see `Track:track-type`). This should
correspond to the type of data that the element can produce or
process.
<!-- trait TrackElementExt::fn set_property_track_type -->
The track type of the element, which determines the type of track the
element can be added to (see `Track:track-type`). This should
correspond to the type of data that the element can produce or
process.
<!-- struct TrackType -->
Types of content handled by a track. If the content is not one of
[`Audio`](Self::Audio), [`Video`](Self::Video) or [`Text`](Self::Text),
the user of the `Track` must set the type to [`Custom`](Self::Custom).

[`Unknown`](Self::Unknown) is for internal purposes and should not be used
by users
<!-- struct TrackType::const UNKNOWN -->
A track of unknown type (i.e. invalid)
<!-- struct TrackType::const AUDIO -->
An audio track
<!-- struct TrackType::const VIDEO -->
A video track
<!-- struct TrackType::const TEXT -->
A text (subtitle) track
<!-- struct TrackType::const CUSTOM -->
A custom-content track
<!-- struct TransitionClip -->
Creates an object that mixes together the two underlying objects, A and B.
The A object is assumed to have a higher prioirity (lower number) than the
B object. At the transition in point, only A will be visible, and by the
end only B will be visible.

The shape of the video transition depends on the value of the "vtype"
property. The default value is "crossfade". For audio, only "crossfade" is
supported.

The ID of the ExtractableType is the nickname of the vtype property value. Note
that this value can be changed after creation and the GESExtractable.asset value
will be updated when needed.

# Implements

[`TransitionClipExt`](trait@crate::TransitionClipExt), [`BaseTransitionClipExt`](trait@crate::BaseTransitionClipExt), [`OperationClipExt`](trait@crate::OperationClipExt), [`ClipExt`](trait@crate::ClipExt), [`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait TransitionClipExt -->
Trait containing all `TransitionClip` methods.

# Implementors

[`TransitionClip`](struct@crate::TransitionClip)
<!-- impl TransitionClip::fn new -->
Creates a new `TransitionClip`.
## `vtype`
the type of transition to create

# Returns

a newly created `TransitionClip`,
or `None` if something went wrong.
<!-- impl TransitionClip::fn for_nick -->
Creates a new `TransitionClip` for the provided `nick`.
## `nick`
a string representing the type of transition to create

# Returns

The newly created `TransitionClip`,
or `None` if something went wrong
<!-- trait TransitionClipExt::fn get_property_vtype -->
a `VideoStandardTransitionType` representing the wipe to use
<!-- trait TransitionClipExt::fn set_property_vtype -->
a `VideoStandardTransitionType` representing the wipe to use
<!-- struct UriClip -->
Represents all the output streams from a particular uri. It is assumed that
the URI points to a file of some type.

# Implements

[`UriClipExt`](trait@crate::UriClipExt), [`ClipExt`](trait@crate::ClipExt), [`GESContainerExt`](trait@crate::GESContainerExt), [`TimelineElementExt`](trait@crate::TimelineElementExt), [`trait@glib::object::ObjectExt`], [`ExtractableExt`](trait@crate::ExtractableExt), [`TimelineElementExtManual`](trait@crate::TimelineElementExtManual)
<!-- trait UriClipExt -->
Trait containing all `UriClip` methods.

# Implementors

[`UriClip`](struct@crate::UriClip)
<!-- impl UriClip::fn new -->
Creates a new `UriClip` for the provided `uri`.

> **WARNING**: This function might 'discover` @uri **synchrounously**, it is
> an IO and processing intensive task that you probably don't want to run in
> an application mainloop. Have a look at #ges_asset_request_async to see how
> to make that operation happen **asynchronously**.
## `uri`
the URI the source should control

# Returns

The newly created `UriClip`, or
`None` if there was an error.
<!-- trait UriClipExt::fn uri -->
Get the location of the resource.

# Returns

The location of the resource.
<!-- trait UriClipExt::fn is_image -->
Lets you know if `self` is an image or not.

# Returns

`true` if `self` is a still image `false` otherwise.
<!-- trait UriClipExt::fn is_muted -->
Lets you know if the audio track of `self` is muted or not.

# Returns

`true` if the audio track of `self` is muted, `false` otherwise.
<!-- trait UriClipExt::fn set_is_image -->
Sets whether the clip is a still image or not.
## `is_image`
`true` if `self` is a still image, `false` otherwise
<!-- trait UriClipExt::fn set_mute -->
Sets whether the audio track of this clip is muted or not.
## `mute`
`true` to mute `self` audio track, `false` to unmute it
<!-- trait UriClipExt::fn get_property_is_image -->
Whether this uri clip represents a still image or not. This must be set
before create_track_elements is called.
<!-- trait UriClipExt::fn set_property_is_image -->
Whether this uri clip represents a still image or not. This must be set
before create_track_elements is called.
<!-- trait UriClipExt::fn get_property_mute -->
Whether the sound will be played or not.
<!-- trait UriClipExt::fn set_property_mute -->
Whether the sound will be played or not.
<!-- trait UriClipExt::fn get_property_uri -->
The location of the file/resource to use.
<!-- trait UriClipExt::fn set_property_uri -->
The location of the file/resource to use.
<!-- struct UriClipAsset -->


# Implements

[`UriClipAssetExt`](trait@crate::UriClipAssetExt), [`AssetExt`](trait@crate::AssetExt), [`trait@glib::object::ObjectExt`]
<!-- trait UriClipAssetExt -->
Trait containing all `UriClipAsset` methods.

# Implementors

[`UriClipAsset`](struct@crate::UriClipAsset)
<!-- impl UriClipAsset::fn finish -->
Finalize the request of an async `UriClipAsset`

Feature: `v1_16`

## `res`
The `gio::AsyncResult` from which to get the newly created `UriClipAsset`

# Returns

The `UriClipAsset` previously requested
<!-- impl UriClipAsset::fn new -->
Creates a `UriClipAsset` for `uri`

Example of request of a GESUriClipAsset:

```text
// The request callback
static void
filesource_asset_loaded_cb (GESAsset * source, GAsyncResult * res, gpointer user_data)
{
  GError *error = NULL;
  GESUriClipAsset *filesource_asset;

  filesource_asset = ges_uri_clip_asset_finish (res, &error);
  if (filesource_asset) {
   g_print ("The file: %s is usable as a FileSource, it is%s an image and lasts %" GST_TIME_FORMAT,
       ges_asset_get_id (GES_ASSET (filesource_asset))
       ges_uri_clip_asset_is_image (filesource_asset) ? "" : " not",
       GST_TIME_ARGS (ges_uri_clip_asset_get_duration (filesource_asset));
  } else {
   g_print ("The file: %s is *not* usable as a FileSource because: %s",
       ges_asset_get_id (source), error->message);
  }

  gst_object_unref (mfs);
}

// The request:
ges_uri_clip_asset_new (uri, (GAsyncReadyCallback) filesource_asset_loaded_cb, user_data);
```
## `uri`
The URI of the file for which to create a `UriClipAsset`
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback` to call when the initialization is finished
## `user_data`
The user data to pass when `callback` is called
<!-- impl UriClipAsset::fn request_sync -->
Creates a `UriClipAsset` for `uri` syncronously. You should avoid
to use it in application, and rather create `UriClipAsset` asynchronously
## `uri`
The URI of the file for which to create a `UriClipAsset`.
You can also use multi file uris for `MultiFileSource`.

# Returns

A reference to the requested asset or `None` if
an error happened
<!-- trait UriClipAssetExt::fn duration -->
Gets duration of the file represented by `self`

# Returns

The duration of `self`
<!-- trait UriClipAssetExt::fn info -->
Gets `gst_pbutils::DiscovererInfo` about the file

# Returns

`gst_pbutils::DiscovererInfo` of specified asset
<!-- trait UriClipAssetExt::fn max_duration -->
Gets maximum duration of the file represented by `self`,
it is usually the same as GESUriClipAsset::duration,
but in the case of nested timelines, for example, they
are different as those can be extended 'infinitely'.

Feature: `v1_18`


# Returns

The maximum duration of `self`
<!-- trait UriClipAssetExt::fn stream_assets -->
Get the GESUriSourceAsset `self` containes

# Returns

a
`glib::List` of `UriSourceAsset`
<!-- trait UriClipAssetExt::fn is_image -->
Gets Whether the file represented by `self` is an image or not

Feature: `v1_18`


# Returns

Whether the file represented by `self` is an image or not
<!-- trait UriClipAssetExt::fn get_property_duration -->
The duration (in nanoseconds) of the media file
<!-- trait UriClipAssetExt::fn set_property_duration -->
The duration (in nanoseconds) of the media file
<!-- trait UriClipAssetExt::fn get_property_is_nested_timeline -->
The duration (in nanoseconds) of the media file

Feature: `v1_18`

<!-- struct UriSourceAsset -->
Asset to create a stream specific `Source` for a media file.

NOTE: You should never request such a `Asset` as they will be created automatically
by `UriClipAsset`-s.

# Implements

[`UriSourceAssetExt`](trait@crate::UriSourceAssetExt), [`AssetExt`](trait@crate::AssetExt), [`trait@glib::object::ObjectExt`]
<!-- trait UriSourceAssetExt -->
Trait containing all `UriSourceAsset` methods.

# Implementors

[`UriSourceAsset`](struct@crate::UriSourceAsset)
<!-- trait UriSourceAssetExt::fn filesource_asset -->
Get the `UriClipAsset` `self_` is contained in

# Returns

a `UriClipAsset`
<!-- trait UriSourceAssetExt::fn stream_info -->
Get the `gst_pbutils::DiscovererStreamInfo` user by `self`

# Returns

a `UriClipAsset`
<!-- trait UriSourceAssetExt::fn is_image -->
Check if `self` contains a single image

Feature: `v1_18`


# Returns

`true` if the video stream corresponds to an image (i.e. only
contains one frame)
<!-- enum VideoStandardTransitionType -->
<!-- enum VideoStandardTransitionType::variant None -->
Transition type has not been set,
<!-- enum VideoStandardTransitionType::variant BarWipeLr -->
A bar moves from left to right,
<!-- enum VideoStandardTransitionType::variant BarWipeTb -->
A bar moves from top to bottom,
<!-- enum VideoStandardTransitionType::variant BoxWipeTl -->
A box expands from the upper-left corner to the lower-right corner,
<!-- enum VideoStandardTransitionType::variant BoxWipeTr -->
A box expands from the upper-right corner to the lower-left corner,
<!-- enum VideoStandardTransitionType::variant BoxWipeBr -->
A box expands from the lower-right corner to the upper-left corner,
<!-- enum VideoStandardTransitionType::variant BoxWipeBl -->
A box expands from the lower-left corner to the upper-right corner,
<!-- enum VideoStandardTransitionType::variant FourBoxWipeCi -->
A box shape expands from each of the four corners toward the center,
<!-- enum VideoStandardTransitionType::variant FourBoxWipeCo -->
A box shape expands from the center of each quadrant toward the corners of each quadrant,
<!-- enum VideoStandardTransitionType::variant BarndoorV -->
A central, vertical line splits and expands toward the left and right edges,
<!-- enum VideoStandardTransitionType::variant BarndoorH -->
A central, horizontal line splits and expands toward the top and bottom edges,
<!-- enum VideoStandardTransitionType::variant BoxWipeTc -->
A box expands from the top edge's midpoint to the bottom corners,
<!-- enum VideoStandardTransitionType::variant BoxWipeRc -->
A box expands from the right edge's midpoint to the left corners,
<!-- enum VideoStandardTransitionType::variant BoxWipeBc -->
A box expands from the bottom edge's midpoint to the top corners,
<!-- enum VideoStandardTransitionType::variant BoxWipeLc -->
A box expands from the left edge's midpoint to the right corners,
<!-- enum VideoStandardTransitionType::variant DiagonalTl -->
A diagonal line moves from the upper-left corner to the lower-right corner,
<!-- enum VideoStandardTransitionType::variant DiagonalTr -->
A diagonal line moves from the upper right corner to the lower-left corner,
<!-- enum VideoStandardTransitionType::variant BowtieV -->
Two wedge shapes slide in from the top and bottom edges toward the center,
<!-- enum VideoStandardTransitionType::variant BowtieH -->
Two wedge shapes slide in from the left and right edges toward the center,
<!-- enum VideoStandardTransitionType::variant BarndoorDbl -->
A diagonal line from the lower-left to upper-right corners splits and expands toward the opposite corners,
<!-- enum VideoStandardTransitionType::variant BarndoorDtl -->
A diagonal line from upper-left to lower-right corners splits and expands toward the opposite corners,
<!-- enum VideoStandardTransitionType::variant MiscDiagonalDbd -->
Four wedge shapes split from the center and retract toward the four edges,
<!-- enum VideoStandardTransitionType::variant MiscDiagonalDd -->
A diamond connecting the four edge midpoints simultaneously contracts toward the center and expands toward the edges,
<!-- enum VideoStandardTransitionType::variant VeeD -->
A wedge shape moves from top to bottom,
<!-- enum VideoStandardTransitionType::variant VeeL -->
A wedge shape moves from right to left,
<!-- enum VideoStandardTransitionType::variant VeeU -->
A wedge shape moves from bottom to top,
<!-- enum VideoStandardTransitionType::variant VeeR -->
A wedge shape moves from left to right,
<!-- enum VideoStandardTransitionType::variant BarnveeD -->
A 'V' shape extending from the bottom edge's midpoint to the opposite corners contracts toward the center and expands toward the edges,
<!-- enum VideoStandardTransitionType::variant BarnveeL -->
A 'V' shape extending from the left edge's midpoint to the opposite corners contracts toward the center and expands toward the edges,
<!-- enum VideoStandardTransitionType::variant BarnveeU -->
A 'V' shape extending from the top edge's midpoint to the opposite corners contracts toward the center and expands toward the edges,
<!-- enum VideoStandardTransitionType::variant BarnveeR -->
A 'V' shape extending from the right edge's midpoint to the opposite corners contracts toward the center and expands toward the edges,
<!-- enum VideoStandardTransitionType::variant IrisRect -->
A rectangle expands from the center.,
<!-- enum VideoStandardTransitionType::variant ClockCw12 -->
A radial hand sweeps clockwise from the twelve o'clock position,
<!-- enum VideoStandardTransitionType::variant ClockCw3 -->
A radial hand sweeps clockwise from the three o'clock position,
<!-- enum VideoStandardTransitionType::variant ClockCw6 -->
A radial hand sweeps clockwise from the six o'clock position,
<!-- enum VideoStandardTransitionType::variant ClockCw9 -->
A radial hand sweeps clockwise from the nine o'clock position,
<!-- enum VideoStandardTransitionType::variant PinwheelTbv -->
Two radial hands sweep clockwise from the twelve and six o'clock positions,
<!-- enum VideoStandardTransitionType::variant PinwheelTbh -->
Two radial hands sweep clockwise from the nine and three o'clock positions,
<!-- enum VideoStandardTransitionType::variant PinwheelFb -->
Four radial hands sweep clockwise,
<!-- enum VideoStandardTransitionType::variant FanCt -->
A fan unfolds from the top edge, the fan axis at the center,
<!-- enum VideoStandardTransitionType::variant FanCr -->
A fan unfolds from the right edge, the fan axis at the center,
<!-- enum VideoStandardTransitionType::variant DoublefanFov -->
Two fans, their axes at the center, unfold from the top and bottom,
<!-- enum VideoStandardTransitionType::variant DoublefanFoh -->
Two fans, their axes at the center, unfold from the left and right,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwt -->
A radial hand sweeps clockwise from the top edge's midpoint,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwr -->
A radial hand sweeps clockwise from the right edge's midpoint,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwb -->
A radial hand sweeps clockwise from the bottom edge's midpoint,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwl -->
A radial hand sweeps clockwise from the left edge's midpoint,
<!-- enum VideoStandardTransitionType::variant DoublesweepPv -->
Two radial hands sweep clockwise and counter-clockwise from the top and bottom edges' midpoints,
<!-- enum VideoStandardTransitionType::variant DoublesweepPd -->
Two radial hands sweep clockwise and counter-clockwise from the left and right edges' midpoints,
<!-- enum VideoStandardTransitionType::variant DoublesweepOv -->
Two radial hands attached at the top and bottom edges' midpoints sweep from right to left,
<!-- enum VideoStandardTransitionType::variant DoublesweepOh -->
Two radial hands attached at the left and right edges' midpoints sweep from top to bottom,
<!-- enum VideoStandardTransitionType::variant FanT -->
A fan unfolds from the bottom, the fan axis at the top edge's midpoint,
<!-- enum VideoStandardTransitionType::variant FanR -->
A fan unfolds from the left, the fan axis at the right edge's midpoint,
<!-- enum VideoStandardTransitionType::variant FanB -->
A fan unfolds from the top, the fan axis at the bottom edge's midpoint,
<!-- enum VideoStandardTransitionType::variant FanL -->
A fan unfolds from the right, the fan axis at the left edge's midpoint,
<!-- enum VideoStandardTransitionType::variant DoublefanFiv -->
Two fans, their axes at the top and bottom, unfold from the center,
<!-- enum VideoStandardTransitionType::variant DoublefanFih -->
Two fans, their axes at the left and right, unfold from the center,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwtl -->
A radial hand sweeps clockwise from the upper-left corner,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwbl -->
A radial hand sweeps counter-clockwise from the lower-left corner.,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwbr -->
A radial hand sweeps clockwise from the lower-right corner,
<!-- enum VideoStandardTransitionType::variant SinglesweepCwtr -->
A radial hand sweeps counter-clockwise from the upper-right corner,
<!-- enum VideoStandardTransitionType::variant DoublesweepPdtl -->
Two radial hands attached at the upper-left and lower-right corners sweep down and up,
<!-- enum VideoStandardTransitionType::variant DoublesweepPdbl -->
Two radial hands attached at the lower-left and upper-right corners sweep down and up,
<!-- enum VideoStandardTransitionType::variant SaloondoorT -->
Two radial hands attached at the upper-left and upper-right corners sweep down,
<!-- enum VideoStandardTransitionType::variant SaloondoorL -->
Two radial hands attached at the upper-left and lower-left corners sweep to the right,
<!-- enum VideoStandardTransitionType::variant SaloondoorB -->
Two radial hands attached at the lower-left and lower-right corners sweep up,
<!-- enum VideoStandardTransitionType::variant SaloondoorR -->
Two radial hands attached at the upper-right and lower-right corners sweep to the left,
<!-- enum VideoStandardTransitionType::variant WindshieldR -->
Two radial hands attached at the midpoints of the top and bottom halves sweep from right to left,
<!-- enum VideoStandardTransitionType::variant WindshieldU -->
Two radial hands attached at the midpoints of the left and right halves sweep from top to bottom,
<!-- enum VideoStandardTransitionType::variant WindshieldV -->
Two sets of radial hands attached at the midpoints of the top and bottom halves sweep from top to bottom and bottom to top,
<!-- enum VideoStandardTransitionType::variant WindshieldH -->
Two sets of radial hands attached at the midpoints of the left and right halves sweep from left to right and right to left,
<!-- enum VideoStandardTransitionType::variant Crossfade -->
Crossfade
