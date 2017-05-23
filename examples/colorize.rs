extern crate cloudfn;

use cloudfn::image::Colorize;
use std::path::Path;

fn main() {
    let file = Path::new("bw.jpg");
    let resp = file.colorize().unwrap();
    resp.save("color.png").unwrap();
}
