// Declare our Base Struct from base.rs
// No need to be public since it's only used in this directory.
mod base;

// Expose Done & Pending Task Structs.
pub mod done;
pub mod pending;
