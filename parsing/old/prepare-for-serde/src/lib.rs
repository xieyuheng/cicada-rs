#![feature (uniform_paths)]
#![allow (unused_parens)]
#![allow (dead_code)]
#![allow (unused_macros)]

mod error;
mod de;
mod ser;

pub use error::{
    Error,
    Result,
};

pub use de::{
    from_str,
    Deserializer,
};

pub use ser::{
    to_string,
    Serializer,
};
