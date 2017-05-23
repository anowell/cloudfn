#![feature(associated_type_defaults)]

extern crate algorithmia;
extern crate uuid;
extern crate serde;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;

#[cfg(feature="image-processing")]
extern crate image as image_crate;

use algorithmia::Algorithmia;
use algorithmia::data::{DataDir, DataAcl};

#[cfg(feature="image-processing")]
pub mod image;

pub mod error;

lazy_static! {
    static ref CLIENT: Algorithmia = Algorithmia::default();
    static ref DATA_TMP: DataDir = {
        let dir = CLIENT.dir("data://.my/tmp");
        let _ = dir.create(DataAcl::default());
        dir
    };
}

/// Get the global Algorithmia client
pub fn client() -> &'static Algorithmia {
    &*CLIENT
}

// fn init_with_env(api_key: &str) {
//     *CLIENT = Algorithmia::client(api_key);
// }

