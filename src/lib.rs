//! Implements a more compact but limited representation of a list of strings.
//!
//! Strings are stored contiguously in a vector of bytes, with their lengths and starting indices
//! being stored separately.
//!
//! Limitations include being unable to mutate strings stored in the vector.
//!
//! # Examples
//! ```
//! # use compact_strings::CompactStrings;
//! let mut cmpstrs = CompactStrings::with_capacity(20, 3);
//!
//! cmpstrs.push("One");
//! cmpstrs.push("Two");
//! cmpstrs.push("Three");
//!
//! cmpstrs.remove(1);
//!
//! assert_eq!(cmpstrs.get(0), Some("One"));
//! assert_eq!(cmpstrs.get(1), Some("Three"));
//! assert_eq!(cmpstrs.get(2), None);
//! ```
#![no_std]
#![warn(clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate alloc;

mod compact_strings;
pub use compact_strings::CompactStrings;
mod compact_bytestrings;
pub use compact_bytestrings::CompactBytestrings;
mod metadata;
