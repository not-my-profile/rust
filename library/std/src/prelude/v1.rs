//! The first version of the prelude of The Rust Standard Library.
//!
//! See the [module-level documentation](super) for more.

#![stable(feature = "rust1", since = "1.0.0")]

// Re-exported core operators
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::marker::{Send, Sized, Sync, Unpin};
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::mem::drop;

// Re-exported types and traits
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::convert::{AsMut, AsRef, From, Into};
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::iter::{Extend, IntoIterator, Iterator};
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::option::Option::{self, None, Some};
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::result::Result::{self, Err, Ok};

// Re-exported built-in macros
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow(deprecated)]
#[doc(no_inline)]
pub use core::prelude::v1::{
    assert, cfg, column, compile_error, concat, concat_idents, env, file, format_args,
    format_args_nl, include, include_bytes, include_str, line, llvm_asm, log_syntax, module_path,
    option_env, stringify, trace_macros, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq,
    PartialOrd,
};

#[unstable(
    feature = "concat_bytes",
    issue = "87555",
    reason = "`concat_bytes` is not stable enough for use and is subject to change"
)]
#[cfg(not(bootstrap))]
#[doc(no_inline)]
pub use core::prelude::v1::concat_bytes;

#[unstable(
    feature = "asm",
    issue = "72016",
    reason = "inline assembly is not stable enough for use and is subject to change"
)]
#[doc(no_inline)]
pub use core::prelude::v1::asm;

#[unstable(
    feature = "global_asm",
    issue = "35119",
    reason = "`global_asm!` is not stable enough for use and is subject to change"
)]
#[doc(no_inline)]
pub use core::prelude::v1::global_asm;

// FIXME: Attribute and internal derive macros are not documented because for them rustdoc generates
// dead links which fail link checker testing.
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow(deprecated, deprecated_in_future)]
#[doc(hidden)]
pub use core::prelude::v1::{
    bench, global_allocator, test, test_case, RustcDecodable, RustcEncodable,
};

#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(hidden)]
pub use core::prelude::v1::derive;

#[unstable(
    feature = "cfg_accessible",
    issue = "64797",
    reason = "`cfg_accessible` is not fully implemented"
)]
#[doc(hidden)]
pub use core::prelude::v1::cfg_accessible;

#[unstable(
    feature = "cfg_eval",
    issue = "82679",
    reason = "`cfg_eval` is a recently implemented feature"
)]
#[doc(hidden)]
pub use core::prelude::v1::cfg_eval;

// The file so far is equivalent to src/libcore/prelude/v1.rs,
// and below to src/liballoc/prelude.rs.
// Those files are duplicated rather than using glob imports
// because we want docs to show these re-exports as pointing to within `std`.

#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::borrow::ToOwned;
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::boxed::Box;
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::string::{String, ToString};
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::vec::Vec;
