pub mod error;
pub mod fields;
pub mod objects;
pub mod rules;

pub mod api;

pub const API_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("api");
