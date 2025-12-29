//! Koto Timeline - Timeline and arrangement

use koto_core::SamplePosition;
use serde::{Deserialize, Serialize};

/// Unique identifier for tracks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub u64);

/// Unique identifier for regions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionId(pub u64);

/// Track type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Audio,
    Midi,
    Instrument,
    Bus,
    Master,
}

/// Audio/MIDI region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: RegionId,
    pub name: String,
    pub start: SamplePosition,
    pub length: SamplePosition,
    pub track_id: TrackId,
    pub color: u32,
}

impl Region {
    pub fn new(id: RegionId, track_id: TrackId, start: SamplePosition, length: SamplePosition) -> Self {
        Self {
            id,
            name: String::new(),
            start,
            length,
            track_id,
            color: 0x4A90D9,
        }
    }

    pub fn end(&self) -> SamplePosition {
        SamplePosition(self.start.0 + self.length.0)
    }
}

/// Track in the timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub track_type: TrackType,
    pub regions: Vec<Region>,
    pub mute: bool,
    pub solo: bool,
    pub armed: bool,
    pub height: u32,
    pub color: u32,
}

impl Track {
    pub fn new(id: TrackId, name: impl Into<String>, track_type: TrackType) -> Self {
        Self {
            id,
            name: name.into(),
            track_type,
            regions: Vec::new(),
            mute: false,
            solo: false,
            armed: false,
            height: 80,
            color: 0x4A90D9,
        }
    }

    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }
}

/// The main timeline structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    next_track_id: u64,
    next_region_id: u64,
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new track
    pub fn add_track(&mut self, name: impl Into<String>, track_type: TrackType) -> TrackId {
        let id = TrackId(self.next_track_id);
        self.next_track_id += 1;
        self.tracks.push(Track::new(id, name, track_type));
        id
    }

    /// Remove a track
    pub fn remove_track(&mut self, id: TrackId) {
        self.tracks.retain(|t| t.id != id);
    }

    /// Get a track by ID
    pub fn get_track(&self, id: TrackId) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    /// Get a mutable track by ID
    pub fn get_track_mut(&mut self, id: TrackId) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == id)
    }

    /// Create a new region ID
    pub fn new_region_id(&mut self) -> RegionId {
        let id = RegionId(self.next_region_id);
        self.next_region_id += 1;
        id
    }
}
