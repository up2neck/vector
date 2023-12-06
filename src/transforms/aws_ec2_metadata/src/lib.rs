// copied from src/internal_events/remap.rs
mod internal_events;

pub mod aws_ec2_metadata;

// part of src/ extracted to library
use config_lib as config;

use vector_lib::Result;

use vector_lib::Error;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate tracing;
