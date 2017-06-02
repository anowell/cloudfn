extern crate cloudfn;

use cloudfn::image::{Classifier};
use cloudfn::image::classifiers::{DeepFace, Subreddit, Places365, RealEstate, Illustration};
use std::path::Path;

fn main() {
    let file = Path::new("bw.jpg");
    let resp = file.classify().unwrap();
    println!("DEFAULT: {:#?}", resp);

    let resp = file.classify_with(&DeepFace);
    println!("DEEPFACE: {:#?}", resp);

    let resp = file.classify_with(&Subreddit);
    println!("SUBREDDIT: {:#?}", resp);

    let resp = file.classify_with(&Places365);
    println!("PLACES: {:#?}", resp);

    let resp = file.classify_with(&RealEstate{ num_results: Some(3) });
    println!("REALESTATE: {:#?}", resp);

    let resp = file.classify_with(&Illustration::default());
    println!("ILLUSTRATION: {:#?}", resp);
}
