//! Librairie Rust pour un cache **LRU (Least Recently Used)** générique.

pub mod cache;
pub mod traits;
pub mod structs;
pub mod persistence;

/// Type principal exporté pour le cache LRU
pub use structs::Cache;
