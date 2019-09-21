//! Raw FastLZ FFI bindings

#![no_std]

extern crate cty;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
