// -*- mode: rust; -*-
//
// This file is part of ed25519-dalek.
// Copyright (c) 2017 Isis Lovecruft
// See LICENSE for licensing information.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>

//! Errors which may occur when parsing keys and/or signatures to or from wire formats.

// rustc seems to think the typenames in match statements (e.g. in
// Display) should be snake cased, for some reason.
#![allow(non_snake_case)]

use core::fmt;
use core::fmt::Display;

/// Internal errors.  Most application-level developer will likely not
/// need to pay any attention to these.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub (crate) enum InternalError {
    PointDecompressionError,
    ScalarFormatError,
    /// An error in the length of bytes handed to a constructor.
    ///
    /// To use this, pass a string specifying the `name` of the type which is
    /// returning the error, and the `length` in bytes which its constructor
    /// expects.
    BytesLengthError{ name: &'static str, length: usize },
}

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InternalError::PointDecompressionError
                => write!(f, "Cannot decompress extended twisted edwards point"),
            InternalError::ScalarFormatError
                => write!(f, "Cannot use scalar with high-bit set"),
            InternalError::BytesLengthError{ name: n, length: l}
                => write!(f, "{} must be {} bytes in length", n, l),
        }
    }
}

impl ::failure::Fail for InternalError {}

/// Errors which may occur in the `from_bytes()` constructors of `PublicKey`,
/// `SecretKey`, `ExpandedSecretKey`, `Keypair`, and `Signature`.
///
/// There was an internal problem due to parsing the `Signature`.
///
/// This error may arise due to:
///
/// * A problem decompressing `r`, a curve point, in the `Signature`, or the
///   curve point for a `PublicKey`.
/// * A problem with the format of `s`, a scalar, in the `Signature`.  This
///   is only raised if the high-bit of the scalar was set.  (Scalars must
///   only be constructed from 255-bit integers.)
/// * Being given bytes with a length different to what was expected.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct DecodingError(pub (crate) InternalError);

impl Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            InternalError::PointDecompressionError                => write!(f, "{}", self.0),
            InternalError::ScalarFormatError                      => write!(f, "{}", self.0),
            InternalError::BytesLengthError{ name: _, length: _ } => write!(f, "{}", self.0),
        }
    }
}

impl ::failure::Fail for DecodingError {
    fn cause(&self) -> Option<&::failure::Fail> {
        match self.0 {
            InternalError::PointDecompressionError               => Some(&self.0),
            InternalError::ScalarFormatError                     => Some(&self.0),
            InternalError::BytesLengthError{ name: _, length: _} => Some(&self.0),
        }
    }
}
