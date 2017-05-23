use CLIENT;
use error::Result;
use image::{ImageFile, ImageInterop};

use algorithmia::data::HasDataPath;

static ALGO_COLORIZER: &str = "deeplearning/ColorfulImageColorization/1.1";

/// Colorize a black & white image
pub trait Colorize {
    /// ## Examples
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// use cloudfn::image::Colorize;
    /// let file = File::open("foo.jpg").unwrap();
    /// let processed = file.colorize().unwrap();
    /// ```
    fn colorize(self) -> Result<ImageFile>;
}

/// Colorize black & white images in a single batch
///
/// TODO: this trait should do some analysis of photo count/sizes
///     to determine an appropriate timeout
///
pub trait BatchColorize {
    /// ## Examples
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// use cloudfn::image::BatchColorize;
    /// let names = ["foo.jpg", "bar.jpg"];
    /// let files = names.iter().map(|p| File::open(p).unwrap());
    /// let processed = files.colorize().unwrap();
    /// ```
    fn colorize(self) -> Result<Vec<ImageFile>>;
}

#[derive(Serialize)]
struct ColorizerInput {
    image: Vec<String>,
}

#[derive(Deserialize)]
struct ColorizerOutput {
    output: Vec<String>
}

impl <I: ImageInterop> Colorize for I {
    fn colorize(self) -> Result<ImageFile> {
        let input = ColorizerInput {
            image: vec![self.into_data_file()?.to_data_uri()]
        };

        let resp = colorize(&input)?;
        Ok(ImageFile(CLIENT.file(&resp.output[0])))
    }
}

impl <I, D> BatchColorize for I where I: Iterator<Item=D>, D: ImageInterop {
    fn colorize(self) -> Result<Vec<ImageFile>> {
        // TODO: parallelize this step (which could involve uploads)
        let data_uris = self
            .map(|d| d.into_data_file().map(|d| d.to_data_uri()))
            .collect::<Result<_>>()?;

        let input = ColorizerInput { image: data_uris };
        let resp = colorize(&input)?;

        let files = resp.output
            .iter()
            .map(|uri| ImageFile(CLIENT.file(uri)))
            .collect();
        Ok(files)
    }
}

fn colorize(input: &ColorizerInput) -> Result<ColorizerOutput> {
        let resp = CLIENT.algo(ALGO_COLORIZER)
            .pipe(&input)?
            .decode()?;
        Ok(resp)
}
