// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This file is imported into every module by default.

/* Reexported core operators */

pub use either::{Either, Left, Right};
pub use kinds::{Const, Copy, Owned, Durable};
pub use ops::{Add, Sub, Mul, Div, Modulo, Neg, Not};
pub use ops::{BitAnd, BitOr, BitXor};
pub use ops::{Drop};
pub use ops::{Shl, Shr, Index};
pub use option::{Option, Some, None};
pub use result::{Result, Ok, Err};

/* Reexported types and traits */

pub use clone::Clone;
pub use cmp::{Eq, Ord};
pub use container::{Container, Mutable, Map, Set};
pub use hash::Hash;
pub use iter::{BaseIter, ExtendedIter, EqIter, CopyableIter};
pub use iter::{CopyableOrderedIter, CopyableNonstrictIter, Times};
pub use num::Num;
pub use path::GenericPath;
pub use path::Path;
pub use path::PosixPath;
pub use path::WindowsPath;
pub use pipes::{GenericChan, GenericPort};
pub use ptr::Ptr;
pub use str::{StrSlice, Trimmable};
pub use to_bytes::IterBytes;
pub use to_str::ToStr;
pub use tuple::{CopyableTuple, ImmutableTuple, ExtendedTupleOps};
pub use vec::{CopyableVector, ImmutableVector};
pub use vec::{ImmutableEqVector, ImmutableCopyableVector};
pub use vec::{OwnedVector, OwnedCopyableVector};

/* Reexported modules */

pub use at_vec;
pub use bool;
pub use cast;
pub use char;
pub use cmp;
pub use dvec;
pub use either;
pub use extfmt;
pub use f32;
pub use f64;
pub use float;
pub use i16;
pub use i32;
pub use i64;
pub use i8;
pub use int;
pub use io;
pub use iter;
pub use libc;
pub use num;
pub use ops;
pub use option;
pub use os;
pub use path;
pub use pipes;
pub use private;
pub use ptr;
pub use rand;
pub use result;
pub use str;
pub use sys;
pub use task;
pub use to_str;
pub use u16;
pub use u32;
pub use u64;
pub use u8;
pub use uint;
pub use vec;

/*
 * Export the log levels as global constants. Higher levels mean
 * more-verbosity. Error is the bottom level, default logging level is
 * warn-and-below.
 */

/// The error log level
pub const error : u32 = 1_u32;
/// The warning log level
pub const warn : u32 = 2_u32;
/// The info log level
pub const info : u32 = 3_u32;
/// The debug log level
pub const debug : u32 = 4_u32;
