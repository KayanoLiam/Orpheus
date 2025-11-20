// Orpheus Library - BaaS Platform Core
//
// 这个库提供 Backend-as-a-Service 的核心功能

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::empty_loop)]
#![deny(clippy::indexing_slicing)]
#![deny(unused)]

// 公开导出核心模块
pub mod schema;

// 导出常用类型
pub use schema::SchemaCache;
