//! Simple document-based NoSQL similar to MongoDB.
//! <br><br>
//! Final project for ECE464 Databases.
//! By Jonathan Lam, Derek Lee, Victor Zhang.

pub mod crud;
pub mod db;
pub mod document;
pub mod query;
pub mod value;

// Internal API's; should be hidden for production
// Unhidden now so that they show up in the documentation.
pub mod index;
pub mod mmapv1;
