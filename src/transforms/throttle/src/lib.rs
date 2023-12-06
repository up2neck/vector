pub mod throttle;

mod internal_events;

use vector_lib::Result;

// part of src/ extracted to library
use config_lib as config;

#[macro_use]
extern crate tracing;
