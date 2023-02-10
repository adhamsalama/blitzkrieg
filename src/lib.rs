//! # Blitzkrieg
//!
//! An HTTP Web Server written from scratch in Rust.
//!
//! This is written for educational purposes and is not meant to be used in production.

/// A module for parsing HTTP.
pub mod http;
/// A module for implementing a Server struct.
pub mod server;
/// A module for implementing a threadpool for the server.
pub mod threadpool;
