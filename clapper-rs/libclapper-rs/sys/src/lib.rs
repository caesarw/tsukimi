// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../../gir-files-gstreamer
// from ../../gir-files-gtk
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;
use gio_sys as gio;
use gstreamer_sys as gst;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, time_t, off_t, intptr_t, uintptr_t, FILE};
#[cfg(unix)]
#[allow(unused_imports)]
use libc::{dev_t, gid_t, pid_t, socklen_t, uid_t};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type ClapperDiscovererDiscoveryMode = c_int;
pub const CLAPPER_DISCOVERER_DISCOVERY_ALWAYS: ClapperDiscovererDiscoveryMode = 0;
pub const CLAPPER_DISCOVERER_DISCOVERY_NONCURRENT: ClapperDiscovererDiscoveryMode = 1;

pub type ClapperMarkerType = c_int;
pub const CLAPPER_MARKER_TYPE_UNKNOWN: ClapperMarkerType = 0;
pub const CLAPPER_MARKER_TYPE_TITLE: ClapperMarkerType = 1;
pub const CLAPPER_MARKER_TYPE_CHAPTER: ClapperMarkerType = 2;
pub const CLAPPER_MARKER_TYPE_TRACK: ClapperMarkerType = 3;
pub const CLAPPER_MARKER_TYPE_CUSTOM_1: ClapperMarkerType = 101;
pub const CLAPPER_MARKER_TYPE_CUSTOM_2: ClapperMarkerType = 102;
pub const CLAPPER_MARKER_TYPE_CUSTOM_3: ClapperMarkerType = 103;

pub type ClapperPlayerSeekMethod = c_int;
pub const CLAPPER_PLAYER_SEEK_METHOD_ACCURATE: ClapperPlayerSeekMethod = 0;
pub const CLAPPER_PLAYER_SEEK_METHOD_NORMAL: ClapperPlayerSeekMethod = 1;
pub const CLAPPER_PLAYER_SEEK_METHOD_FAST: ClapperPlayerSeekMethod = 2;

pub type ClapperPlayerState = c_int;
pub const CLAPPER_PLAYER_STATE_STOPPED: ClapperPlayerState = 0;
pub const CLAPPER_PLAYER_STATE_BUFFERING: ClapperPlayerState = 1;
pub const CLAPPER_PLAYER_STATE_PAUSED: ClapperPlayerState = 2;
pub const CLAPPER_PLAYER_STATE_PLAYING: ClapperPlayerState = 3;

pub type ClapperQueueProgressionMode = c_int;
pub const CLAPPER_QUEUE_PROGRESSION_NONE: ClapperQueueProgressionMode = 0;
pub const CLAPPER_QUEUE_PROGRESSION_CONSECUTIVE: ClapperQueueProgressionMode = 1;
pub const CLAPPER_QUEUE_PROGRESSION_REPEAT_ITEM: ClapperQueueProgressionMode = 2;
pub const CLAPPER_QUEUE_PROGRESSION_CAROUSEL: ClapperQueueProgressionMode = 3;
pub const CLAPPER_QUEUE_PROGRESSION_SHUFFLE: ClapperQueueProgressionMode = 4;

pub type ClapperStreamType = c_int;
pub const CLAPPER_STREAM_TYPE_UNKNOWN: ClapperStreamType = 0;
pub const CLAPPER_STREAM_TYPE_VIDEO: ClapperStreamType = 1;
pub const CLAPPER_STREAM_TYPE_AUDIO: ClapperStreamType = 2;
pub const CLAPPER_STREAM_TYPE_SUBTITLE: ClapperStreamType = 3;

// Constants
pub const CLAPPER_HAVE_DISCOVERER: gboolean = glib::GTRUE;
pub const CLAPPER_HAVE_MPRIS: gboolean = glib::GTRUE;
pub const CLAPPER_HAVE_SERVER: gboolean = glib::GTRUE;
pub const CLAPPER_MAJOR_VERSION: c_int = 0;
pub const CLAPPER_MARKER_NO_END: c_double = -1.000000;
pub const CLAPPER_MICRO_VERSION: c_int = 0;
pub const CLAPPER_MINOR_VERSION: c_int = 7;
pub const CLAPPER_QUEUE_INVALID_POSITION: c_uint = 4294967295;
pub const CLAPPER_STREAM_LIST_INVALID_POSITION: c_uint = 4294967295;
pub const CLAPPER_TIME_FORMAT: &[u8] = b"02u:%02u:%02u\0";
pub const CLAPPER_TIME_MS_FORMAT: &[u8] = b"02u:%02u:%02u.%03u\0";
pub const CLAPPER_VERSION_S: &[u8] = b"0.7.0\0";

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperAudioStreamClass {
    pub parent_class: ClapperStreamClass,
}

impl ::std::fmt::Debug for ClapperAudioStreamClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperAudioStreamClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperDiscovererClass {
    pub parent_class: ClapperFeatureClass,
}

impl ::std::fmt::Debug for ClapperDiscovererClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperDiscovererClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperFeatureClass {
    pub parent_class: gst::GstObjectClass,
    pub prepare: Option<unsafe extern "C" fn(*mut ClapperFeature) -> gboolean>,
    pub unprepare: Option<unsafe extern "C" fn(*mut ClapperFeature) -> gboolean>,
    pub property_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, *mut gobject::GParamSpec)>,
    pub state_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, ClapperPlayerState)>,
    pub position_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, c_double)>,
    pub speed_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, c_double)>,
    pub volume_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, c_double)>,
    pub mute_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, gboolean)>,
    pub played_item_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, *mut ClapperMediaItem)>,
    pub item_updated: Option<unsafe extern "C" fn(*mut ClapperFeature, *mut ClapperMediaItem)>,
    pub queue_item_added: Option<unsafe extern "C" fn(*mut ClapperFeature, *mut ClapperMediaItem, c_uint)>,
    pub queue_item_removed: Option<unsafe extern "C" fn(*mut ClapperFeature, *mut ClapperMediaItem, c_uint)>,
    pub queue_item_repositioned: Option<unsafe extern "C" fn(*mut ClapperFeature, c_uint, c_uint)>,
    pub queue_cleared: Option<unsafe extern "C" fn(*mut ClapperFeature)>,
    pub queue_progression_changed: Option<unsafe extern "C" fn(*mut ClapperFeature, ClapperQueueProgressionMode)>,
    pub padding: [gpointer; 8],
}

impl ::std::fmt::Debug for ClapperFeatureClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperFeatureClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("prepare", &self.prepare)
         .field("unprepare", &self.unprepare)
         .field("property_changed", &self.property_changed)
         .field("state_changed", &self.state_changed)
         .field("position_changed", &self.position_changed)
         .field("speed_changed", &self.speed_changed)
         .field("volume_changed", &self.volume_changed)
         .field("mute_changed", &self.mute_changed)
         .field("played_item_changed", &self.played_item_changed)
         .field("item_updated", &self.item_updated)
         .field("queue_item_added", &self.queue_item_added)
         .field("queue_item_removed", &self.queue_item_removed)
         .field("queue_item_repositioned", &self.queue_item_repositioned)
         .field("queue_cleared", &self.queue_cleared)
         .field("queue_progression_changed", &self.queue_progression_changed)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperMarkerClass {
    pub parent_class: gst::GstObjectClass,
}

impl ::std::fmt::Debug for ClapperMarkerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperMarkerClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperMediaItemClass {
    pub parent_class: gst::GstObjectClass,
}

impl ::std::fmt::Debug for ClapperMediaItemClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperMediaItemClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperMprisClass {
    pub parent_class: ClapperFeatureClass,
}

impl ::std::fmt::Debug for ClapperMprisClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperMprisClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperPlayerClass {
    pub parent_class: ClapperThreadedObjectClass,
}

impl ::std::fmt::Debug for ClapperPlayerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperPlayerClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperQueueClass {
    pub parent_class: gst::GstObjectClass,
}

impl ::std::fmt::Debug for ClapperQueueClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperQueueClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperServerClass {
    pub parent_class: ClapperFeatureClass,
}

impl ::std::fmt::Debug for ClapperServerClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperServerClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperStreamClass {
    pub parent_class: gst::GstObjectClass,
    pub internal_stream_updated: Option<unsafe extern "C" fn(*mut ClapperStream, *mut gst::GstCaps, *mut gst::GstTagList)>,
    pub padding: [gpointer; 4],
}

impl ::std::fmt::Debug for ClapperStreamClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperStreamClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("internal_stream_updated", &self.internal_stream_updated)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperStreamListClass {
    pub parent_class: gst::GstObjectClass,
}

impl ::std::fmt::Debug for ClapperStreamListClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperStreamListClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperSubtitleStreamClass {
    pub parent_class: ClapperStreamClass,
}

impl ::std::fmt::Debug for ClapperSubtitleStreamClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperSubtitleStreamClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperThreadedObjectClass {
    pub parent_class: gst::GstObjectClass,
    pub thread_start: Option<unsafe extern "C" fn(*mut ClapperThreadedObject)>,
    pub thread_stop: Option<unsafe extern "C" fn(*mut ClapperThreadedObject)>,
    pub padding: [gpointer; 4],
}

impl ::std::fmt::Debug for ClapperThreadedObjectClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperThreadedObjectClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .field("thread_start", &self.thread_start)
         .field("thread_stop", &self.thread_stop)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperTimelineClass {
    pub parent_class: gst::GstObjectClass,
}

impl ::std::fmt::Debug for ClapperTimelineClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperTimelineClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperVideoStreamClass {
    pub parent_class: ClapperStreamClass,
}

impl ::std::fmt::Debug for ClapperVideoStreamClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperVideoStreamClass @ {self:p}"))
         .field("parent_class", &self.parent_class)
         .finish()
    }
}

// Classes
#[repr(C)]
pub struct ClapperAudioStream {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperAudioStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperAudioStream @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperDiscoverer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperDiscoverer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperDiscoverer @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperFeature {
    pub parent_instance: gst::GstObject,
}

impl ::std::fmt::Debug for ClapperFeature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperFeature @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct ClapperMarker {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperMarker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperMarker @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperMediaItem {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperMediaItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperMediaItem @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperMpris {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperMpris {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperMpris @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperPlayer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperPlayer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperPlayer @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperQueue {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperQueue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperQueue @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperServer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperServer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperServer @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperStream {
    pub parent_instance: gst::GstObject,
}

impl ::std::fmt::Debug for ClapperStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperStream @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct ClapperStreamList {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperStreamList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperStreamList @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperSubtitleStream {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperSubtitleStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperSubtitleStream @ {self:p}"))
         .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClapperThreadedObject {
    pub parent_instance: gst::GstObject,
}

impl ::std::fmt::Debug for ClapperThreadedObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperThreadedObject @ {self:p}"))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
pub struct ClapperTimeline {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperTimeline {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperTimeline @ {self:p}"))
         .finish()
    }
}

#[repr(C)]
pub struct ClapperVideoStream {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for ClapperVideoStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("ClapperVideoStream @ {self:p}"))
         .finish()
    }
}

#[link(name = "clapper-0.0")]
extern "C" {

    //=========================================================================
    // ClapperDiscovererDiscoveryMode
    //=========================================================================
    pub fn clapper_discoverer_discovery_mode_get_type() -> GType;

    //=========================================================================
    // ClapperMarkerType
    //=========================================================================
    pub fn clapper_marker_type_get_type() -> GType;

    //=========================================================================
    // ClapperPlayerSeekMethod
    //=========================================================================
    pub fn clapper_player_seek_method_get_type() -> GType;

    //=========================================================================
    // ClapperPlayerState
    //=========================================================================
    pub fn clapper_player_state_get_type() -> GType;

    //=========================================================================
    // ClapperQueueProgressionMode
    //=========================================================================
    pub fn clapper_queue_progression_mode_get_type() -> GType;

    //=========================================================================
    // ClapperStreamType
    //=========================================================================
    pub fn clapper_stream_type_get_type() -> GType;

    //=========================================================================
    // ClapperAudioStream
    //=========================================================================
    pub fn clapper_audio_stream_get_type() -> GType;
    pub fn clapper_audio_stream_get_bitrate(stream: *mut ClapperAudioStream) -> c_uint;
    pub fn clapper_audio_stream_get_channels(stream: *mut ClapperAudioStream) -> c_int;
    pub fn clapper_audio_stream_get_codec(stream: *mut ClapperAudioStream) -> *mut c_char;
    pub fn clapper_audio_stream_get_lang_code(stream: *mut ClapperAudioStream) -> *mut c_char;
    pub fn clapper_audio_stream_get_lang_name(stream: *mut ClapperAudioStream) -> *mut c_char;
    pub fn clapper_audio_stream_get_sample_format(stream: *mut ClapperAudioStream) -> *mut c_char;
    pub fn clapper_audio_stream_get_sample_rate(stream: *mut ClapperAudioStream) -> c_int;

    //=========================================================================
    // ClapperDiscoverer
    //=========================================================================
    pub fn clapper_discoverer_get_type() -> GType;
    pub fn clapper_discoverer_new() -> *mut ClapperDiscoverer;
    pub fn clapper_discoverer_get_discovery_mode(discoverer: *mut ClapperDiscoverer) -> ClapperDiscovererDiscoveryMode;
    pub fn clapper_discoverer_set_discovery_mode(discoverer: *mut ClapperDiscoverer, mode: ClapperDiscovererDiscoveryMode);

    //=========================================================================
    // ClapperFeature
    //=========================================================================
    pub fn clapper_feature_get_type() -> GType;

    //=========================================================================
    // ClapperMarker
    //=========================================================================
    pub fn clapper_marker_get_type() -> GType;
    pub fn clapper_marker_new(marker_type: ClapperMarkerType, title: *const c_char, start: c_double, end: c_double) -> *mut ClapperMarker;
    pub fn clapper_marker_get_end(marker: *mut ClapperMarker) -> c_double;
    pub fn clapper_marker_get_marker_type(marker: *mut ClapperMarker) -> ClapperMarkerType;
    pub fn clapper_marker_get_start(marker: *mut ClapperMarker) -> c_double;
    pub fn clapper_marker_get_title(marker: *mut ClapperMarker) -> *const c_char;

    //=========================================================================
    // ClapperMediaItem
    //=========================================================================
    pub fn clapper_media_item_get_type() -> GType;
    pub fn clapper_media_item_new(uri: *const c_char) -> *mut ClapperMediaItem;
    pub fn clapper_media_item_new_from_file(file: *mut gio::GFile) -> *mut ClapperMediaItem;
    pub fn clapper_media_item_get_container_format(item: *mut ClapperMediaItem) -> *mut c_char;
    pub fn clapper_media_item_get_duration(item: *mut ClapperMediaItem) -> c_double;
    pub fn clapper_media_item_get_id(item: *mut ClapperMediaItem) -> c_uint;
    pub fn clapper_media_item_get_suburi(item: *mut ClapperMediaItem) -> *mut c_char;
    pub fn clapper_media_item_get_timeline(item: *mut ClapperMediaItem) -> *mut ClapperTimeline;
    pub fn clapper_media_item_get_title(item: *mut ClapperMediaItem) -> *mut c_char;
    pub fn clapper_media_item_get_uri(item: *mut ClapperMediaItem) -> *const c_char;
    pub fn clapper_media_item_set_suburi(item: *mut ClapperMediaItem, suburi: *const c_char);

    //=========================================================================
    // ClapperMpris
    //=========================================================================
    pub fn clapper_mpris_get_type() -> GType;
    pub fn clapper_mpris_new(own_name: *const c_char, identity: *const c_char, desktop_entry: *const c_char) -> *mut ClapperMpris;
    pub fn clapper_mpris_get_fallback_art_url(mpris: *mut ClapperMpris) -> *mut c_char;
    pub fn clapper_mpris_get_queue_controllable(mpris: *mut ClapperMpris) -> gboolean;
    pub fn clapper_mpris_set_fallback_art_url(mpris: *mut ClapperMpris, art_url: *const c_char);
    pub fn clapper_mpris_set_queue_controllable(mpris: *mut ClapperMpris, controllable: gboolean);

    //=========================================================================
    // ClapperPlayer
    //=========================================================================
    pub fn clapper_player_get_type() -> GType;
    pub fn clapper_player_new() -> *mut ClapperPlayer;
    pub fn clapper_player_add_feature(player: *mut ClapperPlayer, feature: *mut ClapperFeature);
    pub fn clapper_player_get_audio_enabled(player: *mut ClapperPlayer) -> gboolean;
    pub fn clapper_player_get_audio_filter(player: *mut ClapperPlayer) -> *mut gst::GstElement;
    pub fn clapper_player_get_audio_offset(player: *mut ClapperPlayer) -> c_double;
    pub fn clapper_player_get_audio_sink(player: *mut ClapperPlayer) -> *mut gst::GstElement;
    pub fn clapper_player_get_audio_streams(player: *mut ClapperPlayer) -> *mut ClapperStreamList;
    pub fn clapper_player_get_autoplay(player: *mut ClapperPlayer) -> gboolean;
    pub fn clapper_player_get_current_audio_decoder(player: *mut ClapperPlayer) -> *mut gst::GstElement;
    pub fn clapper_player_get_current_video_decoder(player: *mut ClapperPlayer) -> *mut gst::GstElement;
    pub fn clapper_player_get_mute(player: *mut ClapperPlayer) -> gboolean;
    pub fn clapper_player_get_position(player: *mut ClapperPlayer) -> c_double;
    pub fn clapper_player_get_queue(player: *mut ClapperPlayer) -> *mut ClapperQueue;
    pub fn clapper_player_get_speed(player: *mut ClapperPlayer) -> c_double;
    pub fn clapper_player_get_state(player: *mut ClapperPlayer) -> ClapperPlayerState;
    pub fn clapper_player_get_subtitle_font_desc(player: *mut ClapperPlayer) -> *mut c_char;
    pub fn clapper_player_get_subtitle_offset(player: *mut ClapperPlayer) -> c_double;
    pub fn clapper_player_get_subtitle_streams(player: *mut ClapperPlayer) -> *mut ClapperStreamList;
    pub fn clapper_player_get_subtitles_enabled(player: *mut ClapperPlayer) -> gboolean;
    pub fn clapper_player_get_video_enabled(player: *mut ClapperPlayer) -> gboolean;
    pub fn clapper_player_get_video_filter(player: *mut ClapperPlayer) -> *mut gst::GstElement;
    pub fn clapper_player_get_video_sink(player: *mut ClapperPlayer) -> *mut gst::GstElement;
    pub fn clapper_player_get_video_streams(player: *mut ClapperPlayer) -> *mut ClapperStreamList;
    pub fn clapper_player_get_volume(player: *mut ClapperPlayer) -> c_double;
    pub fn clapper_player_pause(player: *mut ClapperPlayer);
    pub fn clapper_player_play(player: *mut ClapperPlayer);
    pub fn clapper_player_seek(player: *mut ClapperPlayer, position: c_double);
    pub fn clapper_player_seek_custom(player: *mut ClapperPlayer, position: c_double, method: ClapperPlayerSeekMethod);
    pub fn clapper_player_set_audio_enabled(player: *mut ClapperPlayer, enabled: gboolean);
    pub fn clapper_player_set_audio_filter(player: *mut ClapperPlayer, element: *mut gst::GstElement);
    pub fn clapper_player_set_audio_offset(player: *mut ClapperPlayer, offset: c_double);
    pub fn clapper_player_set_audio_sink(player: *mut ClapperPlayer, element: *mut gst::GstElement);
    pub fn clapper_player_set_autoplay(player: *mut ClapperPlayer, enabled: gboolean);
    pub fn clapper_player_set_mute(player: *mut ClapperPlayer, mute: gboolean);
    pub fn clapper_player_set_speed(player: *mut ClapperPlayer, speed: c_double);
    pub fn clapper_player_set_subtitle_font_desc(player: *mut ClapperPlayer, font_desc: *const c_char);
    pub fn clapper_player_set_subtitle_offset(player: *mut ClapperPlayer, offset: c_double);
    pub fn clapper_player_set_subtitles_enabled(player: *mut ClapperPlayer, enabled: gboolean);
    pub fn clapper_player_set_video_enabled(player: *mut ClapperPlayer, enabled: gboolean);
    pub fn clapper_player_set_video_filter(player: *mut ClapperPlayer, element: *mut gst::GstElement);
    pub fn clapper_player_set_video_sink(player: *mut ClapperPlayer, element: *mut gst::GstElement);
    pub fn clapper_player_set_volume(player: *mut ClapperPlayer, volume: c_double);
    pub fn clapper_player_stop(player: *mut ClapperPlayer);

    //=========================================================================
    // ClapperQueue
    //=========================================================================
    pub fn clapper_queue_get_type() -> GType;
    pub fn clapper_queue_add_item(queue: *mut ClapperQueue, item: *mut ClapperMediaItem);
    pub fn clapper_queue_clear(queue: *mut ClapperQueue);
    pub fn clapper_queue_find_item(queue: *mut ClapperQueue, item: *mut ClapperMediaItem, index: *mut c_uint) -> gboolean;
    pub fn clapper_queue_get_current_index(queue: *mut ClapperQueue) -> c_uint;
    pub fn clapper_queue_get_current_item(queue: *mut ClapperQueue) -> *mut ClapperMediaItem;
    pub fn clapper_queue_get_gapless(queue: *mut ClapperQueue) -> gboolean;
    pub fn clapper_queue_get_instant(queue: *mut ClapperQueue) -> gboolean;
    pub fn clapper_queue_get_item(queue: *mut ClapperQueue, index: c_uint) -> *mut ClapperMediaItem;
    pub fn clapper_queue_get_n_items(queue: *mut ClapperQueue) -> c_uint;
    pub fn clapper_queue_get_progression_mode(queue: *mut ClapperQueue) -> ClapperQueueProgressionMode;
    pub fn clapper_queue_insert_item(queue: *mut ClapperQueue, item: *mut ClapperMediaItem, index: c_int);
    pub fn clapper_queue_item_is_current(queue: *mut ClapperQueue, item: *mut ClapperMediaItem) -> gboolean;
    pub fn clapper_queue_remove_index(queue: *mut ClapperQueue, index: c_uint);
    pub fn clapper_queue_remove_item(queue: *mut ClapperQueue, item: *mut ClapperMediaItem);
    pub fn clapper_queue_reposition_item(queue: *mut ClapperQueue, item: *mut ClapperMediaItem, index: c_int);
    pub fn clapper_queue_select_index(queue: *mut ClapperQueue, index: c_uint) -> gboolean;
    pub fn clapper_queue_select_item(queue: *mut ClapperQueue, item: *mut ClapperMediaItem) -> gboolean;
    pub fn clapper_queue_select_next_item(queue: *mut ClapperQueue) -> gboolean;
    pub fn clapper_queue_select_previous_item(queue: *mut ClapperQueue) -> gboolean;
    pub fn clapper_queue_set_gapless(queue: *mut ClapperQueue, gapless: gboolean);
    pub fn clapper_queue_set_instant(queue: *mut ClapperQueue, instant: gboolean);
    pub fn clapper_queue_set_progression_mode(queue: *mut ClapperQueue, mode: ClapperQueueProgressionMode);
    pub fn clapper_queue_steal_index(queue: *mut ClapperQueue, index: c_uint) -> *mut ClapperMediaItem;

    //=========================================================================
    // ClapperServer
    //=========================================================================
    pub fn clapper_server_get_type() -> GType;
    pub fn clapper_server_new() -> *mut ClapperServer;
    pub fn clapper_server_get_current_port(server: *mut ClapperServer) -> c_uint;
    pub fn clapper_server_get_enabled(server: *mut ClapperServer) -> gboolean;
    pub fn clapper_server_get_port(server: *mut ClapperServer) -> c_uint;
    pub fn clapper_server_get_queue_controllable(server: *mut ClapperServer) -> gboolean;
    pub fn clapper_server_get_running(server: *mut ClapperServer) -> gboolean;
    pub fn clapper_server_set_enabled(server: *mut ClapperServer, enabled: gboolean);
    pub fn clapper_server_set_port(server: *mut ClapperServer, port: c_uint);
    pub fn clapper_server_set_queue_controllable(server: *mut ClapperServer, controllable: gboolean);

    //=========================================================================
    // ClapperStream
    //=========================================================================
    pub fn clapper_stream_get_type() -> GType;
    pub fn clapper_stream_get_stream_type(stream: *mut ClapperStream) -> ClapperStreamType;
    pub fn clapper_stream_get_title(stream: *mut ClapperStream) -> *mut c_char;

    //=========================================================================
    // ClapperStreamList
    //=========================================================================
    pub fn clapper_stream_list_get_type() -> GType;
    pub fn clapper_stream_list_get_current_index(list: *mut ClapperStreamList) -> c_uint;
    pub fn clapper_stream_list_get_current_stream(list: *mut ClapperStreamList) -> *mut ClapperStream;
    pub fn clapper_stream_list_get_n_streams(list: *mut ClapperStreamList) -> c_uint;
    pub fn clapper_stream_list_get_stream(list: *mut ClapperStreamList, index: c_uint) -> *mut ClapperStream;
    pub fn clapper_stream_list_select_index(list: *mut ClapperStreamList, index: c_uint) -> gboolean;
    pub fn clapper_stream_list_select_stream(list: *mut ClapperStreamList, stream: *mut ClapperStream) -> gboolean;

    //=========================================================================
    // ClapperSubtitleStream
    //=========================================================================
    pub fn clapper_subtitle_stream_get_type() -> GType;
    pub fn clapper_subtitle_stream_get_lang_code(stream: *mut ClapperSubtitleStream) -> *mut c_char;
    pub fn clapper_subtitle_stream_get_lang_name(stream: *mut ClapperSubtitleStream) -> *mut c_char;

    //=========================================================================
    // ClapperThreadedObject
    //=========================================================================
    pub fn clapper_threaded_object_get_type() -> GType;
    pub fn clapper_threaded_object_get_context(threaded_object: *mut ClapperThreadedObject) -> *mut glib::GMainContext;

    //=========================================================================
    // ClapperTimeline
    //=========================================================================
    pub fn clapper_timeline_get_type() -> GType;
    pub fn clapper_timeline_get_marker(timeline: *mut ClapperTimeline, index: c_uint) -> *mut ClapperMarker;
    pub fn clapper_timeline_get_n_markers(timeline: *mut ClapperTimeline) -> c_uint;
    pub fn clapper_timeline_insert_marker(timeline: *mut ClapperTimeline, marker: *mut ClapperMarker) -> gboolean;
    pub fn clapper_timeline_remove_marker(timeline: *mut ClapperTimeline, marker: *mut ClapperMarker);

    //=========================================================================
    // ClapperVideoStream
    //=========================================================================
    pub fn clapper_video_stream_get_type() -> GType;
    pub fn clapper_video_stream_get_bitrate(stream: *mut ClapperVideoStream) -> c_uint;
    pub fn clapper_video_stream_get_codec(stream: *mut ClapperVideoStream) -> *mut c_char;
    pub fn clapper_video_stream_get_fps(stream: *mut ClapperVideoStream) -> c_double;
    pub fn clapper_video_stream_get_height(stream: *mut ClapperVideoStream) -> c_int;
    pub fn clapper_video_stream_get_pixel_format(stream: *mut ClapperVideoStream) -> *mut c_char;
    pub fn clapper_video_stream_get_width(stream: *mut ClapperVideoStream) -> c_int;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn clapper_init(argc: *mut c_int, argv: *mut *mut *mut c_char);
    pub fn clapper_init_check(argc: *mut c_int, argv: *mut *mut *mut c_char) -> gboolean;

}
