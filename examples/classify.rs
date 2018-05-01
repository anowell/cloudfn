extern crate cloudfn;

use cloudfn::image::{Classifier};
use cloudfn::image::classifiers::{DeepFace, Subreddit, Places365, RealEstate, Illustration};
use std::path::Path;

fn main() {
    let file = Path::new("cliff.jpg");
    println!("DEFAULT: {:#?}", file.classify());
    println!("DEEPFACE: {:#?}", file.classify_with(&DeepFace));
    println!("SUBREDDIT: {:#?}", file.classify_with(&Subreddit));
    println!("PLACES: {:#?}", file.classify_with(&Places365));
    println!("REALESTATE: {:#?}", file.classify_with(&RealEstate{ num_results: Some(3) }));
    println!("ILLUSTRATION: {:#?}", file.classify_with(&Illustration::default()));
}
