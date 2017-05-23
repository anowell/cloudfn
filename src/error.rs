use image_crate;
use std::io;

error_chain! {
    links {
        Algorithmia(::algorithmia::error::Error, ::algorithmia::error::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
        Image(image_crate::ImageError) #[cfg(feature="image-processing")];
    }
}