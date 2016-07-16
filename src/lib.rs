/*
 * Copyright 2016 Michael Krolikowski
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! # Azkaban Client
//! ## Usage
//! Add the following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! azkaban-client = { git = "https://github.com/mkroli/azkaban-client" }
//! ```
//!
//! Add the following to your crate root:
//!
//! ```rust
//! extern crate azkaban_client;
//! ```
//!
//! Then you can use it:
//!
//! ```rust,no_run
//! use ::azkaban_client::Azkaban;
//!
//! let azkaban = Azkaban::authenticated("http://127.0.0.1:8081", "azkaban", "azkaban").unwrap();
//! let flows = azkaban.flows("TestProject").unwrap();
//! println!("{:?}", flows);
//! ```

extern crate hyper;
extern crate url;
extern crate rustc_serialize;

mod client;
pub use client::*;

pub mod response;
pub mod error;
