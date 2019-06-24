<!-- file * -->
<!-- struct Asset -->
The Assets in the GStreamer Editing Services represent the resources
that can be used. You can create assets for any type that implements the `Extractable`
interface, for example `GESClips`, `Formatter`, and `TrackElement` do implement it.
This means that assets will represent for example a `GESUriClips`, `BaseEffect` etc,
and then you can extract objects of those types with the appropriate parameters from the asset
using the `AssetExt::extract` method:


```text
GESAsset *effect_asset;
GESEffect *effect;

// You create an asset for an effect
effect_asset = ges_asset_request (GES_TYPE_EFFECT, "agingtv", NULL);

// And now you can extract an instance of GESEffect from that asset
effect = GES_EFFECT (ges_asset_extract (effect_asset));

```

In that example, the advantages of having a `Asset` are that you can know what effects
you are working with and let your user know about the avalaible ones, you can add metadata
to the `Asset` through the `MetaContainer` interface and you have a model for your
custom effects. Note that `Asset` management is making easier thanks to the `Project` class.

Each asset is represented by a pair of `extractable_type` and `id` (string). Actually the `extractable_type`
is the type that implements the `Extractable` interface, that means that for example for a `UriClip`,
the type that implements the `Extractable` interface is `Clip`.
The identifier represents different things depending on the `extractable_type` and you should check
the documentation of each type to know what the ID of `Asset` actually represents for that type. By default,
we only have one `Asset` per type, and the `id` is the name of the type, but this behaviour is overriden
to be more useful. For example, for GESTransitionClips, the ID is the vtype of the transition
you will extract from it (ie crossfade, box-wipe-rc etc..) For `Effect` the ID is the
`bin`-description property of the extracted objects (ie the gst-launch style description of the bin that
will be used).

Each and every `Asset` is cached into GES, and you can query those with the `ges_list_assets` function.
Also the system will automatically register `GESAssets` for `GESFormatters` and `GESTransitionClips`
and standard effects (actually not implemented yet) and you can simply query those calling:

```text
   GList *formatter_assets, *tmp;

   //  List all  the transitions
   formatter_assets = ges_list_assets (GES_TYPE_FORMATTER);

   // Print some infos about the formatter GESAsset
   for (tmp = formatter_assets; tmp; tmp = tmp->next) {
     g_print ("Name of the formatter: %s, file extension it produces: %s",
       ges_meta_container_get_string (GES_META_CONTAINER (tmp->data), GES_META_FORMATTER_NAME),
       ges_meta_container_get_string (GES_META_CONTAINER (tmp->data), GES_META_FORMATTER_EXTENSION));
   }

   g_list_free (transition_assets);

```

You can request the creation of `GESAssets` using either `Asset::request` or
`Asset::request_async`. All the `GESAssets` are cached and thus any asset that has already
been created can be requested again without overhead.

# Implements

[`AssetExt`](trait.AssetExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AssetExt -->
Trait containing all `Asset` methods.

# Implementors

[`Asset`](struct.Asset.html), [`Project`](struct.Project.html)
<!-- impl Asset::fn needs_reload -->
Sets an asset from the internal cache as needing reload. An asset needs reload
in the case where, for example, we were missing a GstPlugin to use it and that
plugin has been installed, or, that particular asset content as changed
meanwhile (in the case of the usage of proxies).

Once an asset has been set as "needs reload", requesting that asset again
will lead to it being re discovered, and reloaded as if it was not in the
cache before.
## `extractable_type`
The `glib::Type` of the object that can be extracted from the
 asset to be reloaded.
## `id`
The identifier of the asset to mark as needing reload

# Returns

`true` if the asset was in the cache and could be set as needing reload,
`false` otherwise.
<!-- impl Asset::fn request -->
Create a `Asset` in the most simple cases, you should look at the `extractable_type`
documentation to see if that constructor can be called for this particular type

As it is recommanded not to instanciate assets for GESUriClip synchronously,
it will not work with this method, but you can instead use the specific
`UriClipAsset::request_sync` method if you really want to.
## `extractable_type`
The `glib::Type` of the object that can be extracted from the new asset.
## `id`
The Identifier or `None`

# Returns

A reference to the wanted `Asset` or `None`
<!-- impl Asset::fn request_async -->
The `callback` will be called from a running `glib::MainLoop` which is iterating a `glib::MainContext`.
Note that, users should ensure the `glib::MainContext`, since this method will notify
`callback` from the thread which was associated with a thread default
`glib::MainContext` at calling `ges_init`.
For example, if a user wants non-default `glib::MainContext` to be associated
with `callback`, `ges_init` must be called after g_main_context_push_thread_default ()
with custom `glib::MainContext`.

Request a new `Asset` asyncronously, `callback` will be called when the materail is
ready to be used or if an error occured.

Example of request of a GESAsset async:

```text
// The request callback
static void
asset_loaded_cb (GESAsset * source, GAsyncResult * res, gpointer user_data)
{
  GESAsset *asset;
  GError *error = NULL;

  asset = ges_asset_request_finish (res, &error);
  if (asset) {
   g_print ("The file: %s is usable as a FileSource",
       ges_asset_get_id (asset));
  } else {
   g_print ("The file: %s is *not* usable as a FileSource because: %s",
       ges_asset_get_id (source), error->message);
  }

  gst_object_unref (mfs);
}

// The request:
ges_asset_request_async (GES_TYPE_URI_CLIP, some_uri, NULL,
   (GAsyncReadyCallback) asset_loaded_cb, user_data);
```
## `extractable_type`
The `glib::Type` of the object that can be extracted from the
 new asset. The class must implement the `Extractable` interface.
## `id`
The Identifier of the asset we want to create. This identifier depends of the extractable,
type you want. By default it is the name of the class itself (or `None`), but for example for a
GESEffect, it will be the pipeline description, for a GESUriClip it
will be the name of the file, etc... You should refer to the documentation of the `Extractable`
type you want to create a `Asset` for.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback` to call when the initialization is finished,
Note that the `source` of the callback will be the `Asset`, but you need to
make sure that the asset is properly loaded using the `Asset::request_finish`
method. This asset can not be used as is.
## `user_data`
The user data to pass when `callback` is called
<!-- impl Asset::fn request_finish -->
Finalize the request of an async `Asset`
## `res`
The `gio::AsyncResult` from which to get the newly created `Asset`

# Returns

The `Asset` previously requested
<!-- trait AssetExt::fn extract -->
Extracts a new `gobject::Object` from `asset`. The type of the object is
defined by the extractable-type of `asset`, you can check what
type will be extracted from `asset` using
`AssetExt::get_extractable_type`

# Returns

A newly created `Extractable`
<!-- trait AssetExt::fn get_error -->

# Returns

The `glib::Error` of the asset or `None` if
the asset was loaded without issue
<!-- trait AssetExt::fn get_extractable_type -->
Gets the type of object that can be extracted from `self`

# Returns

the type of object that can be extracted from `self`
<!-- trait AssetExt::fn get_id -->
Gets the ID of a `Asset`

# Returns

The ID of `self`
<!-- trait AssetExt::fn get_proxy -->

# Returns

The proxy in use for `self`
<!-- trait AssetExt::fn get_proxy_target -->

# Returns

The `Asset` that is proxied by `self`
<!-- trait AssetExt::fn list_proxies -->

# Returns

The list of proxies `self` has. Note that the default asset to be
used is always the first in that list.
<!-- trait AssetExt::fn set_proxy -->
A proxying asset is an asset that can substitue the real `self`. For example if you
have a full HD `UriClipAsset` you might want to set a lower resolution (HD version
of the same file) as proxy. Note that when an asset is proxied, calling
`Asset::request` will actually return the proxy asset.
## `proxy`
The `Asset` that should be used as default proxy for `self` or
`None` if you want to use the currently set proxy. Note that an asset can proxy one and only
one other asset.

# Returns

`true` if `proxy` has been set on `self`, `false` otherwise.
<!-- trait AssetExt::fn unproxy -->
Removes `proxy` from the list of known proxies for `self`.
If `proxy` was the current proxy for `self`, stop using it.
## `proxy`
The `Asset` to stop considering as a proxy for `self`

# Returns

`true` if `proxy` was a known proxy for `self`, `false` otherwise.
<!-- struct BaseEffect -->


# Implements

[`TrackElementExt`](trait.TrackElementExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- struct Clip -->
A `Clip` is a 'natural' object which controls one or more
`TrackElement`(s) in one or more `Track`(s).

Keeps a reference to the `TrackElement`(s) it created and
sets/updates their properties.

# Implements

[`ClipExt`](trait.ClipExt.html), [`GESContainerExt`](trait.GESContainerExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait ClipExt -->
Trait containing all `Clip` methods.

# Implementors

[`Clip`](struct.Clip.html)
<!-- trait ClipExt::fn add_asset -->
Extracts a `TrackElement` from `asset` and adds it to the `self`.
Should only be called in order to add operations to a `Clip`,
ni other cases TrackElement are added automatically when adding the
`Clip`/`Asset` to a layer.

Takes a reference on `track_element`.
## `asset`
a `Asset` with `GES_TYPE_TRACK_ELEMENT` as extractable_type

# Returns

Created `TrackElement` or NULL
if an error happened
<!-- trait ClipExt::fn find_track_element -->
Finds the `TrackElement` controlled by `self` that is used in `track`. You
may optionally specify a GType to further narrow search criteria.

Note: If many objects match, then the one with the highest priority will be
returned.
## `track`
a `Track` or NULL
## `type_`
a `glib::Type` indicating the type of track element you are looking
for or `G_TYPE_NONE` if you do not care about the track type.

# Returns

The `TrackElement` used by `track`,
else `None`. Unref after usage
<!-- trait ClipExt::fn find_track_elements -->
Finds all the `TrackElement` controlled by `self` that is used in `track`. You
may optionally specify a GType to further narrow search criteria.
## `track`
a `Track` or NULL
## `track_type`
a `TrackType` indicating the type of tracks in which elements
should be searched.
## `type_`
a `glib::Type` indicating the type of track element you are looking
for or `G_TYPE_NONE` if you do not care about the track type.

# Returns

a `glib::List` of the
`TrackElement` contained in `self`.
The refcount of the objects will be increased. The user will have to
unref each `TrackElement` and free the `glib::List`.
<!-- trait ClipExt::fn get_layer -->
Get the `Layer` to which this clip belongs.

# Returns

The `Layer` where this `self` is being
used, or `None` if it is not used on any layer. The caller should unref it
usage.
<!-- trait ClipExt::fn get_supported_formats -->
Get the formats supported by `self`.

# Returns

The formats supported by `self`.
<!-- trait ClipExt::fn get_top_effect_index -->
Gets the index position of an effect.
## `effect`
The `BaseEffect` we want to get the top index from

# Returns

The top index of the effect, -1 if something went wrong.
<!-- trait ClipExt::fn get_top_effects -->
Get effects applied on `self`

# Returns

a `glib::List` of the
`BaseEffect` that are applied on `self` order by ascendant priorities.
The refcount of the objects will be increased. The user will have to
unref each `BaseEffect` and free the `glib::List`.
<!-- trait ClipExt::fn move_to_layer -->
Moves `self` to `layer`. If `self` is not in any layer, it adds it to
`layer`, else, it removes it from its current layer, and adds it to `layer`.
## `layer`
the new `Layer`

# Returns

`true` if `self` could be moved `false` otherwize
<!-- trait ClipExt::fn set_supported_formats -->
Sets the formats supported by the file.
## `supportedformats`
the `TrackType` defining formats supported by `self`
<!-- trait ClipExt::fn set_top_effect_index -->
This is a convenience method that lets you set the index of a top effect.
## `effect`
The `BaseEffect` to move
## `newindex`
the new index at which to move the `effect` inside this
`Clip`

# Returns

`true` if `effect` was successfuly moved, `false` otherwise.
<!-- trait ClipExt::fn split -->
The function modifies `self`, and creates another `Clip` so we have two
clips at the end, splitted at the time specified by `position`, as a position
in the timeline (not in the clip to be split). For example, if
ges_clip_split is called on a 4-second clip playing from 0:01.00 until
0:05.00, with a split position of 0:02.00, this will result in one clip of 1
second and one clip of 3 seconds, not in two clips of 2 seconds.

The newly created clip will be added to the same layer as `self` is in. This
implies that `self` must be in a `Layer` for the operation to be possible.

This method supports clips playing at a different tempo than one second per
second. For example, splitting a clip with a `Effect` 'pitch tempo=1.5'
four seconds after it starts, will set the inpoint of the new clip to six
seconds after that of the clip to split. For this, the rate-changing
property must be registered using `EffectClass::register_rate_property`;
for the 'pitch' plugin, this is already done.
## `position`
a `gst::ClockTime` representing the timeline position at which to split

# Returns

The newly created `Clip` resulting
from the splitting or `None` if the clip can't be split.
<!-- trait ClipExt::fn get_property_layer -->
The GESLayer where this clip is being used. If you want to connect to its
notify signal you should connect to it with g_signal_connect_after as the
signal emission can be stop in the first fase.
<!-- trait ClipExt::fn get_property_supported-formats -->
The formats supported by the clip.
<!-- trait ClipExt::fn set_property_supported-formats -->
The formats supported by the clip.
<!-- struct Container -->
The `Container` base class.

# Implements

[`GESContainerExt`](trait.GESContainerExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait GESContainerExt -->
Trait containing all `Container` methods.

# Implementors

[`Clip`](struct.Clip.html), [`Container`](struct.Container.html), [`Group`](struct.Group.html)
<!-- impl Container::fn group -->
Groups the `Container`-s provided in `containers`. It creates a subclass
of `Container`, depending on the containers provided in `containers`.
Basically, if all the containers in `containers` should be contained in a same
clip (all the `TrackElement` they contain have the exact same
start/inpoint/duration and are in the same layer), it will create a `Clip`
otherwise a `Group` will be created
## `containers`
The
`Container` to group, they must all be in a same `Timeline`

# Returns

The `Container` (subclass) resulting of the
grouping
<!-- trait GESContainerExt::fn add -->
Add the `TimelineElement` to the container.
## `child`
the `TimelineElement`

# Returns

`true` on success, `false` on failure.
<!-- trait GESContainerExt::fn edit -->
Edit `self` in the different exisiting `EditMode` modes. In the case of
slide, and roll, you need to specify a `Edge`
## `layers`
The layers you want the edit to
 happen in, `None` means that the edition is done in all the
 `GESLayers` contained in the current timeline.
## `new_layer_priority`
The priority of the layer `self` should land in.
 If the layer you're trying to move the container to doesn't exist, it will
 be created automatically. -1 means no move.
## `mode`
The `EditMode` in which the editition will happen.
## `edge`
The `Edge` the edit should happen on.
## `position`
The position at which to edit `self` (in nanosecond)

# Returns

`true` if the container as been edited properly, `false` if an error
occured
<!-- trait GESContainerExt::fn get_children -->
Get the list of `TimelineElement` contained in `self`
The user is responsible for unreffing the contained objects
and freeing the list.
## `recursive`
Whether to recursively get children in `self`

# Returns

The list of
timeline element contained in `self`.
<!-- trait GESContainerExt::fn remove -->
Release the `child` from the control of `self`.
## `child`
the `TimelineElement` to release

# Returns

`true` if the `child` was properly released, else `false`.
<!-- trait GESContainerExt::fn ungroup -->
Ungroups the `TimelineElement` contained in this GESContainer,
creating new `Container` containing those `TimelineElement`
apropriately.
## `recursive`
Wether to recursively ungroup `self`

# Returns

The list of
`Container` resulting from the ungrouping operation
The user is responsible for unreffing the contained objects
and freeing the list.
<!-- trait GESContainerExt::fn connect_child_added -->
Will be emitted after a child was added to `container`.
Usually you should connect with `g_signal_connect_after`
as in the first emission stage, the signal emission might
get stopped internally.
## `element`
the `TimelineElement` that was added.
<!-- trait GESContainerExt::fn connect_child_removed -->
Will be emitted after a child was removed from `container`.
## `element`
the `TimelineElement` that was removed.
<!-- trait GESContainerExt::fn get_property_height -->
The span of priorities which this container occupies.
<!-- enum Edge -->
The edges of an object contain in a `Timeline` or `Track`
<!-- enum Edge::variant EdgeStart -->
Represents the start of an object.
<!-- enum Edge::variant EdgeEnd -->
Represents the end of an object.
<!-- enum Edge::variant EdgeNone -->
Represent the fact we are not workin with any edge of an
 object.
<!-- enum EditMode -->
You can also find more explanation about the behaviour of those modes at:
<ulink url="http://pitivi.org/manual/trimming.html"> trim, ripple and roll`</ulink>`
and <ulink url="http://pitivi.org/manual/usingclips.html">clip management`</ulink>`.
<!-- enum EditMode::variant EditNormal -->
The object is edited the normal way (default).
<!-- enum EditMode::variant EditRipple -->
The objects are edited in ripple mode.
 The Ripple mode allows you to modify the beginning/end of a clip
 and move the neighbours accordingly. This will change the overall
 timeline duration. In the case of ripple end, the duration of the
 clip being rippled can't be superior to its max_duration - inpoint
 otherwise the action won't be executed.
<!-- enum EditMode::variant EditRoll -->
The object is edited in roll mode.
 The Roll mode allows you to modify the position of an editing point
 between two clips without modifying the inpoint of the first clip
 nor the out-point of the second clip. This will not change the
 overall timeline duration.
<!-- enum EditMode::variant EditTrim -->
The object is edited in trim mode.
 The Trim mode allows you to modify the in-point/duration of a clip
 without modifying its position in the timeline.
<!-- enum EditMode::variant EditSlide -->
The object is edited in slide mode.
 The Slide mode allows you to modify the position of a clip in a
 timeline without modifying its duration or its in-point, but will
 modify the duration of the previous clip and in-point of the
 following clip so does not modify the overall timeline duration.
 (not implemented yet)
<!-- struct Effect -->


# Implements

[`EffectExt`](trait.EffectExt.html), [`BaseEffectExt`](trait.BaseEffectExt.html), [`TrackElementExt`](trait.TrackElementExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait EffectExt -->
Trait containing all `Effect` methods.

# Implementors

[`Effect`](struct.Effect.html)
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
<!-- trait EffectExt::fn get_property_bin-description -->
The description of the effect bin with a gst-launch-style
pipeline description.

Example: "videobalance saturation=1.5 hue=+0.5"
<!-- trait EffectExt::fn set_property_bin-description -->
The description of the effect bin with a gst-launch-style
pipeline description.

Example: "videobalance saturation=1.5 hue=+0.5"
<!-- struct Extractable -->
FIXME: Long description needed

# Implements

[`ExtractableExt`](trait.ExtractableExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait ExtractableExt -->
Trait containing all `Extractable` methods.

# Implementors

[`BaseEffect`](struct.BaseEffect.html), [`Clip`](struct.Clip.html), [`Container`](struct.Container.html), [`Effect`](struct.Effect.html), [`Extractable`](struct.Extractable.html), [`Group`](struct.Group.html), [`Layer`](struct.Layer.html), [`TimelineElement`](struct.TimelineElement.html), [`Timeline`](struct.Timeline.html), [`TrackElement`](struct.TrackElement.html), [`UriClip`](struct.UriClip.html)
<!-- trait ExtractableExt::fn get_asset -->
Method for getting an asset from a `Extractable`

# Returns

The `Asset` or `None` if none has
been set
<!-- trait ExtractableExt::fn get_id -->

# Returns

The `id` of the associated `Asset`, free with `g_free`
<!-- trait ExtractableExt::fn set_asset -->
Method to set the asset which instantiated the specified object
## `asset`
The `Asset` to set

# Returns

`true` if `asset` could be set `false` otherwize
<!-- struct Group -->
A `Group` is an object which controls one or more
`GESClips` in one or more `Layer`(s).

To instanciate a group, you should use the ges_container_group method,
this will be responsible for deciding what subclass of `Container`
should be instaciated to group the various `TimelineElement` passed
in parametter.

# Implements

[`GroupExt`](trait.GroupExt.html), [`GESContainerExt`](trait.GESContainerExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait GroupExt -->
Trait containing all `Group` methods.

# Implementors

[`Group`](struct.Group.html)
<!-- impl Group::fn new -->
Created a new empty `Group`, if you want to group several container
together, it is recommanded to use the `Container::group` method so the
proper subclass is selected.

# Returns

The new empty group.
<!-- trait GroupExt::fn get_property_duration -->
The duration (in nanoseconds) which will be used in the container
<!-- trait GroupExt::fn set_property_duration -->
The duration (in nanoseconds) which will be used in the container
<!-- trait GroupExt::fn get_property_in-point -->
The in-point at which this `Group` will start outputting data
from its contents (in nanoseconds).

Ex : an in-point of 5 seconds means that the first outputted buffer will
be the one located 5 seconds in the controlled resource.
<!-- trait GroupExt::fn set_property_in-point -->
The in-point at which this `Group` will start outputting data
from its contents (in nanoseconds).

Ex : an in-point of 5 seconds means that the first outputted buffer will
be the one located 5 seconds in the controlled resource.
<!-- trait GroupExt::fn get_property_max-duration -->
The maximum duration (in nanoseconds) of the `Group`.
<!-- trait GroupExt::fn set_property_max-duration -->
The maximum duration (in nanoseconds) of the `Group`.
<!-- trait GroupExt::fn get_property_start -->
The position of the object in its container (in nanoseconds).
<!-- trait GroupExt::fn set_property_start -->
The position of the object in its container (in nanoseconds).
<!-- struct Layer -->
Responsible for the ordering of the various contained Clip(s). A
timeline layer has a "priority" property, which is used to manage the
priorities of individual Clips. Two layers should not have the
same priority within a given timeline.

# Implements

[`LayerExt`](trait.LayerExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait LayerExt -->
Trait containing all `Layer` methods.

# Implementors

[`Layer`](struct.Layer.html)
<!-- impl Layer::fn new -->
Creates a new `Layer`.

# Returns

A new `Layer`
<!-- trait LayerExt::fn add_asset -->
Creates Clip from asset, adds it to layer and
returns a reference to it.
## `asset`
The asset to add to
## `start`
The start value to set on the new `Clip`,
if `start` == GST_CLOCK_TIME_NONE, it will be set to
the current duration of `self`
## `inpoint`
The inpoint value to set on the new `Clip`
## `duration`
The duration value to set on the new `Clip`
## `track_types`
The `TrackType` to set on the the new `Clip`

# Returns

Created `Clip`
<!-- trait LayerExt::fn add_clip -->
Adds the given clip to the layer. Sets the clip's parent, and thus
takes ownership of the clip.

An clip can only be added to one layer.

Calling this method will construct and properly set all the media related
elements on `clip`. If you need to know when those objects (actually `TrackElement`)
are constructed, you should connect to the container::child-added signal which
is emited right after those elements are ready to be used.
## `clip`
the `Clip` to add.

# Returns

`true` if the clip was properly added to the layer, or `false`
if the `self` refuses to add the clip.
<!-- trait LayerExt::fn get_auto_transition -->
Gets whether transitions are automatically added when objects
overlap or not.

# Returns

`true` if transitions are automatically added, else `false`.
<!-- trait LayerExt::fn get_clips -->
Get the clips this layer contains.

# Returns

a `glib::List` of
clips. The user is responsible for
unreffing the contained objects and freeing the list.
<!-- trait LayerExt::fn get_clips_in_interval -->
Gets the clips which appear between `start` and `end` on `self`.
## `start`
start of the interval
## `end`
end of the interval

# Returns

a `glib::List` of clips intersecting [`start`, `end`) interval on `self`.
<!-- trait LayerExt::fn get_duration -->
Lets you retrieve the duration of the layer, which means
the end time of the last clip inside it

# Returns

The duration of a layer
<!-- trait LayerExt::fn get_priority -->
Get the priority of `self` within the timeline.

# Returns

The priority of the `self` within the timeline.
<!-- trait LayerExt::fn get_timeline -->
Get the `Timeline` in which `Layer` currently is.

# Returns

the `Timeline` in which `Layer`
currently is or `None` if not in any timeline yet.
<!-- trait LayerExt::fn is_empty -->
Convenience method to check if `self` is empty (doesn't contain any clip),
or not.

# Returns

`true` if `self` is empty, `false` if it already contains at least
one `Clip`
<!-- trait LayerExt::fn remove_clip -->
Removes the given `clip` from the `self` and unparents it.
Unparenting it means the reference owned by `self` on the `clip` will be
removed. If you wish to use the `clip` after this function, make sure you
call `gst::ObjectExt::ref` before removing it from the `self`.
## `clip`
the `Clip` to remove

# Returns

`true` if the clip could be removed, `false` if the layer does
not want to remove the clip.
<!-- trait LayerExt::fn set_auto_transition -->
Sets the layer to the given `auto_transition`. See the documentation of the
property auto_transition for more information.
## `auto_transition`
whether the auto_transition is active
<!-- trait LayerExt::fn set_priority -->
Sets the layer to the given `priority`. See the documentation of the
priority property for more information.

# Deprecated since 1.16

use `TimelineExt::move_layer` instead. This deprecation means
that you will not need to handle layer priorities at all yourself, GES
will make sure there is never 'gaps' between layer priorities.
## `priority`
the priority to set
<!-- trait LayerExt::fn connect_clip_added -->
Will be emitted after the clip was added to the layer.
## `clip`
the `Clip` that was added.
<!-- trait LayerExt::fn connect_clip_removed -->
Will be emitted after the clip was removed from the layer.
## `clip`
the `Clip` that was removed
<!-- trait LayerExt::fn get_property_auto-transition -->
Sets whether transitions are added automagically when clips overlap.
<!-- trait LayerExt::fn set_property_auto-transition -->
Sets whether transitions are added automagically when clips overlap.
<!-- trait LayerExt::fn get_property_priority -->
The priority of the layer in the `Timeline`. 0 is the highest
priority. Conceptually, a `Timeline` is a stack of GESLayers,
and the priority of the layer represents its position in the stack. Two
layers should not have the same priority within a given GESTimeline.

Note that the timeline needs to be commited (with `TimelineExt::commit`)
for the change to be taken into account.

# Deprecated since 1.16

use `TimelineExt::move_layer` instead. This deprecation means
that you will not need to handle layer priorities at all yourself, GES
will make sure there is never 'gaps' between layer priorities.
<!-- trait LayerExt::fn set_property_priority -->
The priority of the layer in the `Timeline`. 0 is the highest
priority. Conceptually, a `Timeline` is a stack of GESLayers,
and the priority of the layer represents its position in the stack. Two
layers should not have the same priority within a given GESTimeline.

Note that the timeline needs to be commited (with `TimelineExt::commit`)
for the change to be taken into account.

# Deprecated since 1.16

use `TimelineExt::move_layer` instead. This deprecation means
that you will not need to handle layer priorities at all yourself, GES
will make sure there is never 'gaps' between layer priorities.
<!-- struct Pipeline -->
`Pipeline` allows developers to view and render `Timeline`
in a simple fashion.
Its usage is inspired by the 'playbin' element from gst-plugins-base.

# Implements

[`GESPipelineExt`](trait.GESPipelineExt.html), [`gst::PipelineExt`](../gst/trait.PipelineExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GESPipelineExt -->
Trait containing all `Pipeline` methods.

# Implementors

[`Pipeline`](struct.Pipeline.html)
<!-- impl Pipeline::fn new -->
Creates a new conveninence `Pipeline`.

# Returns

the new `Pipeline`.
<!-- trait GESPipelineExt::fn get_mode -->

# Returns

the `PipelineFlags` currently in use.
<!-- trait GESPipelineExt::fn get_thumbnail -->
Returns a `gst::Sample` with the currently playing image in the format specified by
caps. The caller should free the sample with `gst_sample_unref` when finished. If ANY
caps are specified, the information will be returned in the whatever format
is currently used by the sink. This information can be retrieve from caps
associated with the buffer.
## `caps`
caps specifying current format. Use `GST_CAPS_ANY`
for native size.

# Returns

a `gst::Sample` or `None`
<!-- trait GESPipelineExt::fn get_thumbnail_rgb24 -->
A convenience method for `GESPipelineExt::get_thumbnail` which
returns a buffer in 24-bit RGB, optionally scaled to the specified width
and height. If -1 is specified for either dimension, it will be left at
native size. You can retreive this information from the caps associated
with the buffer.

The caller is responsible for unreffing the returned sample with
`gst_sample_unref`.
## `width`
the requested width or -1 for native size
## `height`
the requested height or -1 for native size

# Returns

a `gst::Sample` or `None`
<!-- trait GESPipelineExt::fn preview_get_audio_sink -->
Obtains a pointer to playsink's audio sink element that is used for
displaying audio when the `Pipeline` is in `PipelineFlags::FullPreview`

The caller is responsible for unreffing the returned element with
`gst::ObjectExt::unref`.

# Returns

a pointer to the playsink audio sink `gst::Element`
<!-- trait GESPipelineExt::fn preview_get_video_sink -->
Obtains a pointer to playsink's video sink element that is used for
displaying video when the `Pipeline` is in `PipelineFlags::FullPreview`

The caller is responsible for unreffing the returned element with
`gst::ObjectExt::unref`.

# Returns

a pointer to the playsink video sink `gst::Element`
<!-- trait GESPipelineExt::fn preview_set_audio_sink -->
Sets playsink's audio sink element that is used for displaying audio when
the `Pipeline` is in `PipelineFlags::FullPreview`
## `sink`
a audio sink `gst::Element`
<!-- trait GESPipelineExt::fn preview_set_video_sink -->
Sets playsink's video sink element that is used for displaying video when
the `Pipeline` is in `PipelineFlags::FullPreview`
## `sink`
a video sink `gst::Element`
<!-- trait GESPipelineExt::fn save_thumbnail -->
Saves the current frame to the specified `location`.
## `width`
the requested width or -1 for native size
## `height`
the requested height or -1 for native size
## `format`
a string specifying the desired mime type (for example,
image/jpeg)
## `location`
the path to save the thumbnail

# Returns

`true` if the thumbnail was properly save, else `false`.
<!-- trait GESPipelineExt::fn set_mode -->
switches the `self` to the specified `mode`. The default mode when
creating a `Pipeline` is `PipelineFlags::FullPreview`.

Note: The `self` will be set to `gst::State::Null` during this call due to
the internal changes that happen. The caller will therefore have to
set the `self` to the requested state after calling this method.
## `mode`
the `PipelineFlags` to use

# Returns

`true` if the mode was properly set, else `false`.
<!-- trait GESPipelineExt::fn set_render_settings -->
Specify where the pipeline shall be rendered and with what settings.

A copy of `profile` and `output_uri` will be done internally, the caller can
safely free those values afterwards.

This method must be called before setting the pipeline mode to
`PipelineFlags::Render`
## `output_uri`
the URI to which the timeline will be rendered
## `profile`
the `gst_pbutils::EncodingProfile` to use to render the timeline.

# Returns

`true` if the settings were aknowledged properly, else `false`
<!-- trait GESPipelineExt::fn set_timeline -->
Sets the timeline to use in this pipeline.

The reference to the `timeline` will be stolen by the `self`.
## `timeline`
the `Timeline` to set on the `self`.

# Returns

`true` if the `timeline` could be successfully set on the `self`,
else `false`.
<!-- trait GESPipelineExt::fn get_property_audio-sink -->
Audio sink for the preview.
<!-- trait GESPipelineExt::fn set_property_audio-sink -->
Audio sink for the preview.
<!-- trait GESPipelineExt::fn get_property_mode -->
Pipeline mode. See `GESPipelineExt::set_mode` for more
info.
<!-- trait GESPipelineExt::fn set_property_mode -->
Pipeline mode. See `GESPipelineExt::set_mode` for more
info.
<!-- trait GESPipelineExt::fn get_property_timeline -->
Timeline to use in this pipeline. See also
`GESPipelineExt::set_timeline` for more info.
<!-- trait GESPipelineExt::fn set_property_timeline -->
Timeline to use in this pipeline. See also
`GESPipelineExt::set_timeline` for more info.
<!-- trait GESPipelineExt::fn get_property_video-sink -->
Video sink for the preview.
<!-- trait GESPipelineExt::fn set_property_video-sink -->
Video sink for the preview.
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

# Implements

[`ProjectExt`](trait.ProjectExt.html), [`AssetExt`](trait.AssetExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait ProjectExt -->
Trait containing all `Project` methods.

# Implementors

[`Project`](struct.Project.html)
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
<!-- trait ProjectExt::fn get_loading_assets -->
Get the assets that are being loaded

# Returns

A set of loading asset
that will be added to `self`. Note that those Asset are *not* loaded yet,
and thus can not be used
<!-- trait ProjectExt::fn get_uri -->
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
The formatter asset to use or `None`. If `None`,
will try to save in the same format as the one from which the timeline as been loaded
or default to the formatter with highest rank
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
The `Timeline` that complete loading
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

Contains a list of `Layer` which users should use to arrange the
various clips through time.

The output type is determined by the `Track` that are set on
the `Timeline`.

To save/load a timeline, you can use the `TimelineExt::load_from_uri` and
`TimelineExt::save_to_uri` methods to use the default format. If you wish

Note that any change you make in the timeline will not actually be taken
into account until you call the `TimelineExt::commit` method.

# Implements

[`TimelineExt`](trait.TimelineExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait TimelineExt -->
Trait containing all `Timeline` methods.

# Implementors

[`Timeline`](struct.Timeline.html)
<!-- impl Timeline::fn new -->
Creates a new empty `Timeline`.

# Returns

The new timeline.
<!-- impl Timeline::fn new_audio_video -->
Creates a new `Timeline` containing a raw audio and a
raw video track.

# Returns

The newly created `Timeline`.
<!-- impl Timeline::fn new_from_uri -->
Creates a timeline from the given URI.
## `uri`
the URI to load from

# Returns

A new timeline if the uri was loaded
successfully, or `None` if the uri could not be loaded.
<!-- trait TimelineExt::fn add_layer -->
Add the layer to the timeline. The reference to the `layer` will be stolen
by the `self`.
## `layer`
the `Layer` to add

# Returns

`true` if the layer was properly added, else `false`.
<!-- trait TimelineExt::fn add_track -->
Add a track to the timeline. The reference to the track will be stolen by the
pipeline.
## `track`
the `Track` to add

# Returns

`true` if the track was properly added, else `false`.
<!-- trait TimelineExt::fn append_layer -->
Append a newly created `Layer` to `self`
Note that you do not own any reference to the returned layer.

# Returns

The newly created `Layer`, or the last (empty)
`Layer` of `self`.
<!-- trait TimelineExt::fn commit -->
Commit all the pending changes of the clips contained in the
`self`.

When changes happen in a timeline, they are not
directly executed in the non-linear engine. Call this method once you are
done with a set of changes and want it to be executed.

The `Timeline::commited` signal will be emitted when the (possibly updated)
`gst::Pipeline` is ready to output data again, except if the state of the
timeline was `gst::State::Ready` or `gst::State::Null`.

Note that all the pending changes will automatically be executed when the
timeline goes from `gst::State::Ready` to `gst::State::Paused`, which usually is
triggered by corresponding state changes in a containing `Pipeline`.

You should not try to change the state of the timeline, seek it or add
tracks to it during a commit operation, that is between a call to this
function and after receiving the `Timeline::commited` signal.

See `TimelineExt::commit_sync` if you don't want to bother with waiting
for the signal.

# Returns

`true` if pending changes were commited or `false` if nothing needed
to be commited
<!-- trait TimelineExt::fn commit_sync -->
Commit all the pending changes of the `GESClips` contained in the
`self`.

Will return once the update is complete, that is when the
(possibly updated) `gst::Pipeline` is ready to output data again, or if the
state of the timeline was `gst::State::Ready` or `gst::State::Null`.

This function will wait for any pending state change of the timeline by
calling `gst::ElementExt::get_state` with a `GST_CLOCK_TIME_NONE` timeout, you
should not try to change the state from another thread before this function
has returned.

See `TimelineExt::commit` for more information.

# Returns

`true` if pending changes were commited or `false` if nothing needed
to be commited
<!-- trait TimelineExt::fn get_auto_transition -->
Gets whether transitions are automatically added when objects
overlap or not.

# Returns

`true` if transitions are automatically added, else `false`.
<!-- trait TimelineExt::fn get_duration -->
Get the current duration of `self`

# Returns

The current duration of `self`
<!-- trait TimelineExt::fn get_element -->
Gets a `TimelineElement` contained in the timeline

# Returns

The `TimelineElement` or `None` if
not found.
<!-- trait TimelineExt::fn get_groups -->
Get the list of `Group` present in the Timeline.

# Returns

the list of
`Group` that contain clips present in the timeline's layers.
Must not be changed.
<!-- trait TimelineExt::fn get_layer -->
Retrieve the layer with `priority` as a priority
## `priority`
The priority of the layer to find

# Returns

A `Layer` or `None` if no layer with
`priority` was found

Since 1.6
<!-- trait TimelineExt::fn get_layers -->
Get the list of `Layer` present in the Timeline.

# Returns

the list of
`Layer` present in the Timeline sorted by priority.
The caller should unref each Layer once he is done with them.
<!-- trait TimelineExt::fn get_pad_for_track -->
Search the `gst::Pad` corresponding to the given `self`'s `track`.
## `track`
The `Track`

# Returns

The corresponding `gst::Pad` if it is
found, or `None` if there is an error.
<!-- trait TimelineExt::fn get_snapping_distance -->
Gets the configured snapping distance of the timeline. See
the documentation of the property snapping_distance for more
information.

# Returns

The `snapping_distance` property of the timeline
<!-- trait TimelineExt::fn get_track_for_pad -->
Search the `Track` corresponding to the given `self`'s `pad`.
## `pad`
The `gst::Pad`

# Returns

The corresponding `Track` if it is
found, or `None` if there is an error.
<!-- trait TimelineExt::fn get_tracks -->
Returns the list of `Track` used by the Timeline.

# Returns

A list of `Track`.
The caller should unref each track once he is done with them.
<!-- trait TimelineExt::fn is_empty -->
Check whether a `Timeline` is empty or not

# Returns

`true` if the timeline is empty `false` otherwize
<!-- trait TimelineExt::fn load_from_uri -->
Loads the contents of URI into the given timeline.
## `uri`
The URI to load from

# Returns

`true` if the timeline was loaded successfully, or `false` if the uri
could not be loaded.
<!-- trait TimelineExt::fn move_layer -->
Moves `layer` at `new_layer_priority` meaning that `layer`
we land at that position in the stack of layers inside
the timeline. If `new_layer_priority` is superior than the number
of layers present in the time, it will move to the end of the
stack of layers.
## `layer`
The layer to move at `new_layer_priority`
## `new_layer_priority`
The index at which `layer` should land
<!-- trait TimelineExt::fn paste_element -->
Paste `element` inside the timeline. `element` must have been
created using ges_timeline_element_copy with deep=TRUE set,
i.e. it must be a deep copy, otherwise it will fail.
## `element`
The `TimelineElement` to paste
## `position`
The position in the timeline the element should
be pasted to, meaning it will become the start of `element`
## `layer_priority`
The `Layer` to which the element should be pasted to.
-1 means paste to the same layer from which the `element` has been copied from.

# Returns

Shallow copy of the `element` pasted
<!-- trait TimelineExt::fn remove_layer -->
Removes the layer from the timeline. The reference that the `self` holds on
the layer will be dropped. If you wish to use the `layer` after calling this
method, you need to take a reference before calling.
## `layer`
the `Layer` to remove

# Returns

`true` if the layer was properly removed, else `false`.
<!-- trait TimelineExt::fn remove_track -->
Remove the `track` from the `self`. The reference stolen when adding the
`track` will be removed. If you wish to use the `track` after calling this
function you must ensure that you have a reference to it.
## `track`
the `Track` to remove

# Returns

`true` if the `track` was properly removed, else `false`.
<!-- trait TimelineExt::fn save_to_uri -->
Saves the timeline to the given location
## `uri`
The location to save to
## `formatter_asset`
The formatter asset to use or `None`. If `None`,
will try to save in the same format as the one from which the timeline as been loaded
or default to the formatter with highest rank
## `overwrite`
`true` to overwrite file if it exists

# Returns

`true` if the timeline was successfully saved to the given location,
else `false`.
<!-- trait TimelineExt::fn set_auto_transition -->
Sets the layer to the given `auto_transition`. See the documentation of the
property auto_transition for more information.
## `auto_transition`
whether the auto_transition is active
<!-- trait TimelineExt::fn set_snapping_distance -->
Sets the `snapping_distance` of the timeline. See the documentation of the
property snapping_distance for more information.
## `snapping_distance`
whether the snapping_distance is active
<!-- trait TimelineExt::fn connect_commited -->
This signal will be emitted once the changes initiated by `TimelineExt::commit`
have been executed in the backend. Use `TimelineExt::commit_sync` if you
don't need to do anything in the meantime.
<!-- trait TimelineExt::fn connect_group_added -->
Will be emitted after a new group is added to to the timeline.
## `group`
the `Group`
<!-- trait TimelineExt::fn connect_group_removed -->
Will be emitted after a group has been removed from the timeline.
## `group`
the `Group`
## `children`
a list of `Container`
<!-- trait TimelineExt::fn connect_layer_added -->
Will be emitted after a new layer is added to the timeline.
## `layer`
the `Layer` that was added to the timeline
<!-- trait TimelineExt::fn connect_layer_removed -->
Will be emitted after the layer was removed from the timeline.
## `layer`
the `Layer` that was removed from the timeline
<!-- trait TimelineExt::fn connect_select_tracks_for_object -->
## `clip`
The `Clip` on which `track_element` will land
## `track_element`
The `TrackElement` for which to choose the tracks it should land into

# Returns

a `glib::PtrArray` of `Track`-s where that object should be added
<!-- trait TimelineExt::fn connect_snapping_ended -->
Will be emitted when the 2 `TrackElement` ended to snap
## `obj1`
the first `TrackElement` that was snapping.
## `obj2`
the second `TrackElement` that was snapping.
## `position`
the position where the two objects finally snapping.
<!-- trait TimelineExt::fn connect_snapping_started -->
Will be emitted when the 2 `TrackElement` first snapped
## `obj1`
the first `TrackElement` that was snapping.
## `obj2`
the second `TrackElement` that was snapping.
## `position`
the position where the two objects finally snapping.
<!-- trait TimelineExt::fn connect_track_added -->
Will be emitted after the track was added to the timeline.
## `track`
the `Track` that was added to the timeline
<!-- trait TimelineExt::fn connect_track_removed -->
Will be emitted after the track was removed from the timeline.
## `track`
the `Track` that was removed from the timeline
<!-- trait TimelineExt::fn get_property_auto-transition -->
Sets whether transitions are added automagically when clips overlap.
<!-- trait TimelineExt::fn set_property_auto-transition -->
Sets whether transitions are added automagically when clips overlap.
<!-- trait TimelineExt::fn get_property_duration -->
Current duration (in nanoseconds) of the `Timeline`
<!-- trait TimelineExt::fn get_property_snapping-distance -->
Distance (in nanoseconds) from which a moving object will snap
with it neighboors. 0 means no snapping.
<!-- trait TimelineExt::fn set_property_snapping-distance -->
Distance (in nanoseconds) from which a moving object will snap
with it neighboors. 0 means no snapping.
<!-- struct TimelineElement -->
The GESTimelineElement base class implements the notion of timing as well
as priority. A GESTimelineElement can have a parent object which will be
responsible for controlling its timing properties.

# Implements

[`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait TimelineElementExt -->
Trait containing all `TimelineElement` methods.

# Implementors

[`Container`](struct.Container.html), [`TimelineElement`](struct.TimelineElement.html), [`TrackElement`](struct.TrackElement.html)
<!-- trait TimelineElementExt::fn copy -->
Copies `self`
## `deep`
whether we want to create the elements `self` contains or not

# Returns

The newly create `TimelineElement`, copied from `self`
<!-- trait TimelineElementExt::fn get_child_properties -->
Gets properties of a child of `self`.
## `first_property_name`
The name of the first property to get
<!-- trait TimelineElementExt::fn get_child_property -->
In general, a copy is made of the property contents and
the caller is responsible for freeing the memory by calling
`gobject::Value::unset`.

Gets a property of a GstElement contained in `object`.

Note that `TimelineElementExt::get_child_property` is really
intended for language bindings, `TimelineElementExt::get_child_properties`
is much more convenient for C programming.
## `property_name`
The name of the property
## `value`
return location for the property value, it will
be initialized if it is initialized with 0

# Returns

`true` if the property was found, `false` otherwize
<!-- trait TimelineElementExt::fn get_child_property_by_pspec -->
Gets a property of a child of `self`.
## `pspec`
The `gobject::ParamSpec` that specifies the property you want to get
## `value`
return location for the value
<!-- trait TimelineElementExt::fn get_child_property_valist -->
Gets a property of a child of `self`. If there are various child elements
that have the same property name, you can distinguish them using the following
syntax: 'ClasseName::property_name' as property name. If you don't, the
corresponding property of the first element found will be set.
## `first_property_name`
The name of the first property to get
## `var_args`
value for the first property, followed optionally by more
name/return location pairs, followed by NULL
<!-- trait TimelineElementExt::fn get_duration -->

# Returns

The `duration` of `self`
<!-- trait TimelineElementExt::fn get_inpoint -->

# Returns

The `inpoint` of `self`
<!-- trait TimelineElementExt::fn get_layer_priority -->

# Returns

The priority of the first layer the element is in (note that only
groups can span over several layers). `GES_TIMELINE_ELEMENT_NO_LAYER_PRIORITY`
means that the element is not in a layer.
<!-- trait TimelineElementExt::fn get_max_duration -->

# Returns

The `maxduration` of `self`
<!-- trait TimelineElementExt::fn get_name -->
Returns a copy of the name of `self`.
Caller should `g_free` the return value after usage.

# Returns

The name of `self`
<!-- trait TimelineElementExt::fn get_parent -->
Returns the parent of `self`. This function increases the refcount
of the parent object so you should `gst::ObjectExt::unref` it after usage.

# Returns

parent of `self`, this can be `None` if
`self` has no parent. unref after usage.
<!-- trait TimelineElementExt::fn get_priority -->

# Returns

The `priority` of `self`
<!-- trait TimelineElementExt::fn get_start -->

# Returns

The `start` of `self`
<!-- trait TimelineElementExt::fn get_timeline -->
Returns the timeline of `self`. This function increases the refcount
of the timeline so you should `gst::ObjectExt::unref` it after usage.

# Returns

timeline of `self`, this can be `None` if
`self` has no timeline. unref after usage.
<!-- trait TimelineElementExt::fn get_toplevel_parent -->
Gets the toplevel `TimelineElement` controlling `self`

# Returns

The toplevel controlling parent of `self`
<!-- trait TimelineElementExt::fn get_track_types -->
Gets all the TrackTypes `self` will interact with
<!-- trait TimelineElementExt::fn list_children_properties -->
Gets an array of `gobject::ParamSpec`* for all configurable properties of the
children of `self`.
## `n_properties`
return location for the length of the returned array

# Returns

an array of `gobject::ParamSpec`* which should be freed after use or
`None` if something went wrong
<!-- trait TimelineElementExt::fn lookup_child -->
Looks up which `element` and `pspec` would be effected by the given `name`. If various
contained elements have this property name you will get the first one, unless you
specify the class name in `name`.
## `prop_name`
name of the property to look up. You can specify the name of the
 class as such: "ClassName::property-name", to guarantee that you get the
 proper GParamSpec in case various GstElement-s contain the same property
 name. If you don't do so, you will get the first element found, having
 this property and the and the corresponding GParamSpec.
## `child`
pointer to a `gst::Element` that
 takes the real object to set property on
## `pspec`
pointer to take the `gobject::ParamSpec`
 describing the property

# Returns

TRUE if `element` and `pspec` could be found. FALSE otherwise. In that
case the values for `pspec` and `element` are not modified. Unref `element` after
usage.
<!-- trait TimelineElementExt::fn paste -->
Paste `self` inside the timeline. `self` must have been created
using ges_timeline_element_copy with recurse=TRUE set,
otherwise it will fail.
## `paste_position`
The position in the timeline the element should
be copied to, meaning it will become the start of `self`

# Returns

Paste `self` copying the element
<!-- trait TimelineElementExt::fn ripple -->
Edits `self` in ripple mode. It allows you to modify the
start of `self` and move the following neighbours accordingly.
This will change the overall timeline duration.
## `start`
The new start of `self` in ripple mode.

# Returns

`true` if the self as been rippled properly, `false` if an error
occured
<!-- trait TimelineElementExt::fn ripple_end -->
Edits `self` in ripple mode. It allows you to modify the
duration of a `self` and move the following neighbours accordingly.
This will change the overall timeline duration.
## `end`
The new end (start + duration) of `self` in ripple mode. It will
 basically only change the duration of `self`.

# Returns

`true` if the self as been rippled properly, `false` if an error
occured
<!-- trait TimelineElementExt::fn roll_end -->
Edits `self` in roll mode. It allows you to modify the
duration of a `self` and trim (basicly change the start + inpoint
in this case) the following neighbours accordingly.
This will not change the overall timeline duration.
## `end`
The new end (start + duration) of `self` in roll mode

# Returns

`true` if the self as been rolled properly, `false` if an error
occured
<!-- trait TimelineElementExt::fn roll_start -->
Edits `self` in roll mode. It allows you to modify the
start and inpoint of a `self` and "resize" (basicly change the duration
in this case) of the previous neighbours accordingly.
This will not change the overall timeline duration.
## `start`
The new start of `self` in roll mode, it will also adapat
the in-point of `self` according

# Returns

`true` if the self as been roll properly, `false` if an error
occured
<!-- trait TimelineElementExt::fn set_child_properties -->
Sets a property of a child of `self`. If there are various child elements
that have the same property name, you can distinguish them using the following
syntax: 'ClasseName::property_name' as property name. If you don't, the
corresponding property of the first element found will be set.
## `first_property_name`
The name of the first property to set
<!-- trait TimelineElementExt::fn set_child_property -->
Sets a property of a child of `self`

Note that `TimelineElementExt::set_child_property` is really
intended for language bindings, `TimelineElementExt::set_child_properties`
is much more convenient for C programming.
## `property_name`
The name of the property
## `value`
the value

# Returns

`true` if the property was set, `false` otherwize
<!-- trait TimelineElementExt::fn set_child_property_by_pspec -->
Sets a property of a child of `self`.
## `pspec`
The `gobject::ParamSpec` that specifies the property you want to set
## `value`
the value
<!-- trait TimelineElementExt::fn set_child_property_valist -->
Sets a property of a child of `self`. If there are various child elements
that have the same property name, you can distinguish them using the following
syntax: 'ClasseName::property_name' as property name. If you don't, the
corresponding property of the first element found will be set.
## `first_property_name`
The name of the first property to set
## `var_args`
value for the first property, followed optionally by more
name/return location pairs, followed by NULL
<!-- trait TimelineElementExt::fn set_duration -->
Set the duration of the object

Note that if the timeline snap-distance property of the timeline containing
`self` is set, `self` will properly snap to its neighboors.
## `duration`
the duration in `gst::ClockTime`

# Returns

`true` if `duration` could be set.
<!-- trait TimelineElementExt::fn set_inpoint -->
Set the in-point, that is the moment at which the `self` will start
outputting data from its contents.
## `inpoint`
the in-point in `gst::ClockTime`

# Returns

`true` if `inpoint` could be set.
<!-- trait TimelineElementExt::fn set_max_duration -->
Set the maximun duration of the object
## `maxduration`
the maximum duration in `gst::ClockTime`

# Returns

`true` if `maxduration` could be set.
<!-- trait TimelineElementExt::fn set_name -->
Sets the name of object, or gives `self` a guaranteed unique name (if name is NULL).
This function makes a copy of the provided name, so the caller retains ownership
of the name it sent.
## `name`
The name `self` should take (if avalaible<)
<!-- trait TimelineElementExt::fn set_parent -->
Sets the parent of `self` to `parent`. The parents needs to already
own a hard reference on `self`.
## `parent`
new parent of self

# Returns

`true` if `parent` could be set or `false` when `self`
already had a parent or `self` and `parent` are the same.
<!-- trait TimelineElementExt::fn set_priority -->
Sets the priority of the object within the containing layer

# Deprecated

All priority management is done by GES itself now.
To set `Effect` priorities `ClipExt::set_top_effect_index` should
be used.
## `priority`
the priority

# Returns

`true` if `priority` could be set.
<!-- trait TimelineElementExt::fn set_start -->
Set the position of the object in its containing layer.

Note that if the snapping-distance property of the timeline containing
`self` is set, `self` will properly snap to the edges around `start`.
## `start`
the position in `gst::ClockTime`

# Returns

`true` if `start` could be set.
<!-- trait TimelineElementExt::fn set_timeline -->
Sets the timeline of `self` to `timeline`.
## `timeline`
The `Timeline` `self` is in

# Returns

`true` if `timeline` could be set or `false` when `timeline`
already had a timeline.
<!-- trait TimelineElementExt::fn trim -->
Edits `self` in trim mode. It allows you to modify the
inpoint and start of `self`.
This will not change the overall timeline duration.

Note that to trim the end of an self you can just set its duration. The same way
as this method, it will take into account the snapping-distance property of the
timeline in which `self` is.
## `start`
The new start of `self` in trim mode, will adapt the inpoint
of `self` accordingly

# Returns

`true` if the self as been trimmed properly, `false` if an error
occured
<!-- trait TimelineElementExt::fn connect_deep_notify -->
The deep notify signal is used to be notified of property changes of all
the childs of `timeline_element`
## `prop_object`
the object that originated the signal
## `prop`
the property that changed
<!-- trait TimelineElementExt::fn get_property_duration -->
The duration (in nanoseconds) which will be used in the container
<!-- trait TimelineElementExt::fn set_property_duration -->
The duration (in nanoseconds) which will be used in the container
<!-- trait TimelineElementExt::fn get_property_in-point -->
The in-point at which this `TimelineElement` will start outputting data
from its contents (in nanoseconds).

Ex : an in-point of 5 seconds means that the first outputted buffer will
be the one located 5 seconds in the controlled resource.
<!-- trait TimelineElementExt::fn set_property_in-point -->
The in-point at which this `TimelineElement` will start outputting data
from its contents (in nanoseconds).

Ex : an in-point of 5 seconds means that the first outputted buffer will
be the one located 5 seconds in the controlled resource.
<!-- trait TimelineElementExt::fn get_property_max-duration -->
The maximum duration (in nanoseconds) of the `TimelineElement`.
<!-- trait TimelineElementExt::fn set_property_max-duration -->
The maximum duration (in nanoseconds) of the `TimelineElement`.
<!-- trait TimelineElementExt::fn get_property_name -->
The name of the object
<!-- trait TimelineElementExt::fn set_property_name -->
The name of the object
<!-- trait TimelineElementExt::fn get_property_parent -->
The parent container of the object
<!-- trait TimelineElementExt::fn set_property_parent -->
The parent container of the object
<!-- trait TimelineElementExt::fn get_property_priority -->
The priority of the object.

Setting GESTimelineElement priorities is deprecated
as all priority management is done by GES itself now.
<!-- trait TimelineElementExt::fn set_property_priority -->
The priority of the object.

Setting GESTimelineElement priorities is deprecated
as all priority management is done by GES itself now.
<!-- trait TimelineElementExt::fn get_property_serialize -->
Whether the element should be serialized.
<!-- trait TimelineElementExt::fn set_property_serialize -->
Whether the element should be serialized.
<!-- trait TimelineElementExt::fn get_property_start -->
The position of the object in its container (in nanoseconds).
<!-- trait TimelineElementExt::fn set_property_start -->
The position of the object in its container (in nanoseconds).
<!-- trait TimelineElementExt::fn get_property_timeline -->
The timeline in which `element` is
<!-- trait TimelineElementExt::fn set_property_timeline -->
The timeline in which `element` is
<!-- struct Track -->
Corresponds to one output format (i.e. audio OR video).

Contains the compatible TrackElement(s).

# Implements

[`GESTrackExt`](trait.GESTrackExt.html), [`gst::ElementExt`](../gst/trait.ElementExt.html), [`gst::ObjectExt`](../gst/trait.ObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GESTrackExt -->
Trait containing all `Track` methods.

# Implementors

[`Track`](struct.Track.html)
<!-- impl Track::fn new -->
Creates a new `Track` with the given `type_` and `caps`.

The newly created track will steal a reference to the caps. If you wish to
use those caps elsewhere, you will have to take an extra reference.
## `type_`
The type of track
## `caps`
The caps to restrict the output of the track to.

# Returns

A new `Track`.
<!-- trait GESTrackExt::fn add_element -->
Adds the given object to the track. Sets the object's controlling track,
and thus takes ownership of the `object`.

An object can only be added to one track.
## `object`
the `TrackElement` to add

# Returns

`true` if the object was properly added. `false` if the track does not
want to accept the object.
<!-- trait GESTrackExt::fn commit -->
Commits all the pending changes of the TrackElement contained in the
track.

When timing changes happen in a timeline, the changes are not
directly done inside NLE. This method needs to be called so any changes
on a clip contained in the timeline actually happen at the media
processing level.

# Returns

`true` if something as been commited `false` if nothing needed
to be commited
<!-- trait GESTrackExt::fn get_caps -->
Get the `gst::Caps` this track is configured to output.

# Returns

The `gst::Caps` this track is configured to output.
<!-- trait GESTrackExt::fn get_elements -->
Gets the `TrackElement` contained in `self`

# Returns

the list of
`TrackElement` present in the Track sorted by priority and start.
<!-- trait GESTrackExt::fn get_mixing -->
Gets if the underlying `NleComposition` contains an expandable mixer.

# Returns

`True` if there is a mixer, `False` otherwise.
<!-- trait GESTrackExt::fn get_timeline -->
Get the `Timeline` this track belongs to. Can be `None`.

# Returns

The `Timeline` this track belongs to. Can be `None`.
<!-- trait GESTrackExt::fn remove_element -->
Removes the object from the track and unparents it.
Unparenting it means the reference owned by `self` on the `object` will be
removed. If you wish to use the `object` after this function, make sure you
call `gst::ObjectExt::ref` before removing it from the `self`.
## `object`
the `TrackElement` to remove

# Returns

`true` if the object was removed, else `false` if the track
could not remove the object (like if it didn't belong to the track).
<!-- trait GESTrackExt::fn set_create_element_for_gap_func -->
Sets the function that should be used to create the GstElement used to fill gaps.
To avoid to provide such a function we advice you to use the
`AudioTrack::new` and `VideoTrack::new` constructor when possible.
## `func`
The `GESCreateElementForGapFunc` that will be used
to create `gst::Element` to fill gaps
<!-- trait GESTrackExt::fn set_mixing -->
Sets if the `Track` should be mixing.
## `mixing`
TRUE if the track should be mixing, FALSE otherwise.
<!-- trait GESTrackExt::fn set_restriction_caps -->
Sets the given `caps` as the caps the track has to output.
## `caps`
the `gst::Caps` to set
<!-- trait GESTrackExt::fn set_timeline -->
Sets `timeline` as the timeline controlling `self`.
## `timeline`
a `Timeline`
<!-- trait GESTrackExt::fn update_restriction_caps -->
Updates the restriction caps by modifying all the fields present in `caps`
in the original restriction caps. If for example the current restriction caps
are video/x-raw, format=I420, width=360 and `caps` is video/x-raw, format=RGB,
the restriction caps will be updated to video/x-raw, format=RGB, width=360.

Modification happens for each structure in the new caps, and
one can add new fields or structures through that function.
## `caps`
the `gst::Caps` to update with
<!-- trait GESTrackExt::fn connect_track_element_added -->
Will be emitted after a track element was added to the track.
## `effect`
the `TrackElement` that was added.
<!-- trait GESTrackExt::fn connect_track_element_removed -->
Will be emitted after a track element was removed from the track.
## `effect`
the `TrackElement` that was removed.
<!-- trait GESTrackExt::fn get_property_caps -->
Caps used to filter/choose the output stream. This is generally set to
a generic set of caps like 'video/x-raw' for raw video.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn set_property_caps -->
Caps used to filter/choose the output stream. This is generally set to
a generic set of caps like 'video/x-raw' for raw video.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn get_property_duration -->
Current duration of the track

Default value: O
<!-- trait GESTrackExt::fn get_property_mixing -->
Whether layer mixing is activated or not on the track.
<!-- trait GESTrackExt::fn set_property_mixing -->
Whether layer mixing is activated or not on the track.
<!-- trait GESTrackExt::fn get_property_restriction-caps -->
Caps used to filter/choose the output stream.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn set_property_restriction-caps -->
Caps used to filter/choose the output stream.

Default value: `GST_CAPS_ANY`.
<!-- trait GESTrackExt::fn get_property_track-type -->
Type of stream the track outputs. This is used when creating the `Track`
to specify in generic terms what type of content will be outputted.

It also serves as a 'fast' way to check what type of data will be outputted
from the `Track` without having to actually check the `Track`'s caps
property.
<!-- trait GESTrackExt::fn set_property_track-type -->
Type of stream the track outputs. This is used when creating the `Track`
to specify in generic terms what type of content will be outputted.

It also serves as a 'fast' way to check what type of data will be outputted
from the `Track` without having to actually check the `Track`'s caps
property.
<!-- struct TrackElement -->
`TrackElement` is the Base Class for any object that can be contained in a
`Track`.

It contains the basic information as to the location of the object within
its container, like the start position, the inpoint, the duration and the
priority.

# Implements

[`TrackElementExt`](trait.TrackElementExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait TrackElementExt -->
Trait containing all `TrackElement` methods.

# Implementors

[`TrackElement`](struct.TrackElement.html)
<!-- trait TrackElementExt::fn add_children_props -->
Looks for the properties defines with the various parametters and add
them to the hashtable of children properties.

To be used by subclasses only
## `element`
The GstElement to retrieve properties from
## `wanted_categories`

An array of categories of GstElement to
take into account (as defined in the factory meta "klass" field)
## `blacklist`
A
blacklist of elements factory names to not take into account
## `whitelist`
A list
of propery names to add as children properties
<!-- trait TrackElementExt::fn edit -->
Edit `self` in the different exisiting `EditMode` modes. In the case of
slide, and roll, you need to specify a `Edge`
## `layers`
The layers you want the edit to
 happen in, `None` means that the edition is done in all the
 `GESLayers` contained in the current timeline.
 FIXME: This is not implemented yet.
## `mode`
The `EditMode` in which the edition will happen.
## `edge`
The `Edge` the edit should happen on.
## `position`
The position at which to edit `self` (in nanosecond)

# Returns

`true` if the object as been edited properly, `false` if an error
occured
<!-- trait TrackElementExt::fn get_all_control_bindings -->

# Returns

A
`glib::HashTable` containing all property_name: GstControlBinding
<!-- trait TrackElementExt::fn get_child_properties -->
Gets properties of a child of `self`.

# Deprecated

Use `TimelineElementExt::get_child_properties`
## `first_property_name`
The name of the first property to get
<!-- trait TrackElementExt::fn get_child_property -->
In general, a copy is made of the property contents and
the caller is responsible for freeing the memory by calling
`gobject::Value::unset`.

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

`true` if the property was found, `false` otherwize
<!-- trait TrackElementExt::fn get_child_property_by_pspec -->
Gets a property of a child of `self`.

# Deprecated

Use `TimelineElementExt::get_child_property_by_pspec`
## `pspec`
The `gobject::ParamSpec` that specifies the property you want to get
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
value for the first property, followed optionally by more
name/return location pairs, followed by NULL
<!-- trait TrackElementExt::fn get_control_binding -->
Looks up the various controlled properties for that `TrackElement`,
and returns the `gst::ControlBinding` which controls `property_name`.
## `property_name`
The property_name to which the binding is associated.

# Returns

the `gst::ControlBinding` associated with
`property_name`, or `None` if that property is not controlled.
<!-- trait TrackElementExt::fn get_element -->
Get the `gst::Element` this track element is controlling within GNonLin.

# Returns

the `gst::Element` this track element is controlling
within GNonLin.
<!-- trait TrackElementExt::fn get_gnlobject -->
Get the NleObject object this object is controlling.

# Deprecated

use `TrackElementExt::get_nleobject` instead.

# Returns

the NleObject object this object is controlling.
<!-- trait TrackElementExt::fn get_nleobject -->
Get the GNonLin object this object is controlling.

# Returns

the GNonLin object this object is controlling.
<!-- trait TrackElementExt::fn get_track -->
Get the `Track` to which this object belongs.

# Returns

The `Track` to which this object
belongs. Can be `None` if it is not in any track
<!-- trait TrackElementExt::fn is_active -->
Lets you know if `self` will be used for playback and rendering,
or not.

# Returns

`true` if `self` is active, `false` otherwize
<!-- trait TrackElementExt::fn list_children_properties -->
Gets an array of `gobject::ParamSpec`* for all configurable properties of the
children of `self`.

# Deprecated

Use `TimelineElementExt::list_children_properties`
## `n_properties`
return location for the length of the returned array

# Returns

an array of `gobject::ParamSpec`* which should be freed after use or
`None` if something went wrong
<!-- trait TrackElementExt::fn lookup_child -->
Looks up which `element` and `pspec` would be effected by the given `name`. If various
contained elements have this property name you will get the first one, unless you
specify the class name in `name`.

# Deprecated

Use `TimelineElementExt::lookup_child`
## `prop_name`
name of the property to look up. You can specify the name of the
 class as such: "ClassName::property-name", to guarantee that you get the
 proper GParamSpec in case various GstElement-s contain the same property
 name. If you don't do so, you will get the first element found, having
 this property and the and the corresponding GParamSpec.
## `element`
pointer to a `gst::Element` that
 takes the real object to set property on
## `pspec`
pointer to take the `gobject::ParamSpec`
 describing the property

# Returns

TRUE if `element` and `pspec` could be found. FALSE otherwise. In that
case the values for `pspec` and `element` are not modified. Unref `element` after
usage.
<!-- trait TrackElementExt::fn remove_control_binding -->
Removes a `gst::ControlBinding` from `self`.
## `property_name`
The name of the property to control.

# Returns

`true` if the binding could be removed, `false` if an error
occured
<!-- trait TrackElementExt::fn set_active -->
Sets the usage of the `self`. If `active` is `true`, the object will be used for
playback and rendering, else it will be ignored.
## `active`
visibility

# Returns

`true` if the property was toggled, else `false`
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
the value

# Returns

`true` if the property was set, `false` otherwize
<!-- trait TrackElementExt::fn set_child_property_by_pspec -->
Sets a property of a child of `self`.

# Deprecated

Use `ges_timeline_element_set_child_property_by_spec`
## `pspec`
The `gobject::ParamSpec` that specifies the property you want to set
## `value`
the value
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
value for the first property, followed optionally by more
name/return location pairs, followed by NULL
<!-- trait TrackElementExt::fn set_control_source -->
Creates a `gst::ControlBinding` and adds it to the `gst::Element` concerned by the
property. Use the same syntax as `TrackElementExt::lookup_child` for
the property name.
## `source`
the `gst::ControlSource` to set on the binding.
## `property_name`
The name of the property to control.
## `binding_type`
The type of binding to create. Currently the following values are valid:
 - "direct": See `gst_direct_control_binding_new`
 - "direct-absolute": See `gst_direct_control_binding_new_absolute`

# Returns

`true` if the binding could be created and added, `false` if an error
occured
<!-- trait TrackElementExt::fn connect_control_binding_added -->
The control-binding-added signal is emitted each time a control binding
is added for a child property of `track_element`
## `control_binding`
the `gst::ControlBinding` that has been added
<!-- trait TrackElementExt::fn connect_control_binding_removed -->
The control-binding-removed signal is emitted each time a control binding
is removed for a child property of `track_element`
## `control_binding`
the `gst::ControlBinding` that has been removed
<!-- trait TrackElementExt::fn get_property_active -->
Whether the object should be taken into account in the `Track` output.
If `false`, then its contents will not be used in the resulting track.
<!-- trait TrackElementExt::fn set_property_active -->
Whether the object should be taken into account in the `Track` output.
If `false`, then its contents will not be used in the resulting track.
<!-- struct UriClip -->
Represents all the output streams from a particular uri. It is assumed that
the URI points to a file of some type.

# Implements

[`UriClipExt`](trait.UriClipExt.html), [`ClipExt`](trait.ClipExt.html), [`GESContainerExt`](trait.GESContainerExt.html), [`TimelineElementExt`](trait.TimelineElementExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ExtractableExt`](trait.ExtractableExt.html)
<!-- trait UriClipExt -->
Trait containing all `UriClip` methods.

# Implementors

[`UriClip`](struct.UriClip.html)
<!-- impl UriClip::fn new -->
Creates a new `UriClip` for the provided `uri`.
## `uri`
the URI the source should control

# Returns

The newly created `UriClip`, or
`None` if there was an error.
<!-- trait UriClipExt::fn get_uri -->
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
<!-- trait UriClipExt::fn get_property_is-image -->
Whether this uri clip represents a still image or not. This must be set
before create_track_elements is called.
<!-- trait UriClipExt::fn set_property_is-image -->
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
The `UriClipAsset` is a special `Asset` that lets you handle
the media file to use inside the GStreamer Editing Services. It has APIs that
let you get information about the medias. Also, the tags found in the media file are
set as Metadata of the Asset.

# Implements

[`UriClipAssetExt`](trait.UriClipAssetExt.html), [`AssetExt`](trait.AssetExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait UriClipAssetExt -->
Trait containing all `UriClipAsset` methods.

# Implementors

[`UriClipAsset`](struct.UriClipAsset.html)
<!-- impl UriClipAsset::fn finish -->
Finalize the request of an async `UriClipAsset`
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
<!-- trait UriClipAssetExt::fn get_duration -->
Gets duration of the file represented by `self`

# Returns

The duration of `self`
<!-- trait UriClipAssetExt::fn get_info -->
Gets `gst_pbutils::DiscovererInfo` about the file

# Returns

`gst_pbutils::DiscovererInfo` of specified asset
<!-- trait UriClipAssetExt::fn get_stream_assets -->
Get the GESUriSourceAsset `self` containes

# Returns

a
`glib::List` of `UriSourceAsset`
<!-- trait UriClipAssetExt::fn is_image -->
Gets Whether the file represented by `self` is an image or not

# Returns

Whether the file represented by `self` is an image or not
<!-- trait UriClipAssetExt::fn get_property_duration -->
The duration (in nanoseconds) of the media file
<!-- trait UriClipAssetExt::fn set_property_duration -->
The duration (in nanoseconds) of the media file
<!-- struct UriSourceAsset -->
NOTE: You should never request such a `Asset` as they will be created automatically
by `UriClipAsset`-s.

# Implements

[`UriSourceAssetExt`](trait.UriSourceAssetExt.html), [`AssetExt`](trait.AssetExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait UriSourceAssetExt -->
Trait containing all `UriSourceAsset` methods.

# Implementors

[`UriSourceAsset`](struct.UriSourceAsset.html)
<!-- trait UriSourceAssetExt::fn get_filesource_asset -->
Get the `UriClipAsset` `self_` is contained in

# Returns

a `UriClipAsset`
<!-- trait UriSourceAssetExt::fn get_stream_info -->
Get the `gst_pbutils::DiscovererStreamInfo` user by `self`

# Returns

a `UriClipAsset`
