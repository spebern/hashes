//! An implementation of the [Shabal][1] cryptographic hash algorithm.
//!
//! There are 5 standard algorithms specified in the Shabal standard:
//!
//! * `Shabal192`, which is the `Shabal` algorithm with the result truncated to 192 bits
//! * `Shabal224`, which is the `Shabal` algorithm with the result truncated to 224 bits
//! * `Shabal256`, which is the `Shabal` algorithm with the result truncated to 256 bits.
//! * `Shabal384`, which is the `Shabal` algorithm with the result truncated to 384 bits.
//! * `Shabal512`, which is the `Shabal` algorithm with the result not truncated.
//!
//! There is a single Shabal algorithm. All variants have different intialisation and apart
//! Shabal512 truncate the result.
//!
//! # Usage
//!
//! ```rust
//! # #[macro_use] extern crate hex_literal;
//! # extern crate shabal;
//! # fn main() {
//! use shabal::{Shabal256, Digest};
//!
//! // create a Shabal256 hasher instance
//! let mut hasher = Shabal256::new();
//!
//! // process input message
//! hasher.input(b"helloworld");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 32]
//! let result = hasher.result();
//! assert_eq!(result[..], hex!("d945dee21ffca23ac232763aa9cac6c15805f144db9d6c97395437e01c8595a8"));
//! # }
//! ```
//!
//! Also see [RustCrypto/hashes][2] readme.
//!
//! [1]: https://www.cs.rit.edu/~ark/20090927/Round2Candidates/Shabal.pdf
//! [2]: https://github.com/RustCrypto/hashes
#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]

extern crate block_buffer;
#[macro_use]
extern crate opaque_debug;
#[macro_use]
pub extern crate digest;
#[cfg(feature = "std")]
extern crate std;

mod consts;
mod shabal;

pub use digest::Digest;
pub use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
