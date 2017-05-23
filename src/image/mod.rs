mod colorizer;
mod classifier;

use std::path::Path;
use std::io::{self, Read};
use std::fs::File;
use std::ops::Deref;

use error::{Result};
use {CLIENT, DATA_TMP};
use algorithmia::data::{DataFile, HasDataPath};
use image_crate::{guess_format, ImageFormat};
use uuid::Uuid;

pub use self::colorizer::*;
pub use self::classifier::*;

pub struct ImageFile(DataFile);

impl ImageFile {
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        let path = path.as_ref();
        let mut data = self.0.get()?;
        let mut file = File::create(path)?;
        let bytes = io::copy(&mut data, &mut file)?;
        Ok(bytes)
    }
}

impl Deref for ImageFile {
    type Target = DataFile;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ImageInterop {
    fn into_data_file(self) -> Result<DataFile>;
}

impl ImageInterop for File {
    fn into_data_file(self) -> Result<DataFile> {
        // TODO: guess_format and use correct extension
        let mut buf = Vec::new();
        let mut f = self;
        f.read_to_end(&mut buf)?;

        let format = guess_format(&buf)?;
        let ext = get_ext(format);
        let data_file: DataFile = DATA_TMP.child(&format!("{}.{}", Uuid::new_v4().simple(), ext));
        data_file.put(f)?;
        Ok(data_file)
    }
}

impl <'a> ImageInterop for &'a Path {
    fn into_data_file(self) -> Result<DataFile> {
        let filename = match self.file_name() {
            Some(name) => name.to_string_lossy(),
            None => { return File::open(self)?.into_data_file(); }
        };

        let f = File::open(self)?;
        let data_file: DataFile = DATA_TMP.child(&format!("{}-{}", Uuid::new_v4().simple(), filename));
        data_file.put(f)?;
        Ok(data_file)
    }
}

impl <'a> ImageInterop for &'a DataFile {
    fn into_data_file(self) -> Result<DataFile> {
        Ok(CLIENT.file(&self.to_data_uri()))
    }
}

impl ImageInterop for DataFile {
    fn into_data_file(self) -> Result<DataFile> {
        Ok(self)
    }
}

// TODO: Consider supporting:
// - Url
// - image_crate::DynamicImage
// - &str (and detect if Url or Path or DataFile)

fn get_ext(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::PNG => "png",
        ImageFormat::JPEG => "jpeg",
        ImageFormat::GIF => "gif",
        ImageFormat::WEBP => "webp",
        ImageFormat::PPM => "ppm",
        ImageFormat::TIFF => "tiff",
        ImageFormat::TGA => "tga",
        ImageFormat::BMP => "bmp",
        ImageFormat::ICO => "ico",
        ImageFormat::HDR => "hdr",
    }
}