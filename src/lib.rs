// src/lib.rs
#![allow(clippy::all)] // silence warnings from generated code

pub mod nebius {
    // This will include the generated code from OUT_DIR/nebius.rs
    include!(concat!(env!("OUT_DIR"), "/nebius.rs"));
}
