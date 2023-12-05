pub mod remap;

// part of src/ extracted to library
use config_lib as config;

// copied from src/internal_events/remap.rs
mod internal_events;

use vector_lib::Result;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate tracing;
