use image::ImageInterop;
use error::Result;
use CLIENT;
use serde::Serialize;
use serde::de::DeserializeOwned;
use algorithmia::data::HasDataPath;
use std::borrow::Borrow;
use std::collections::HashMap;

/// Classifies an image
pub trait Classifier {
    /// ## Examples
    ///
    /// ```no_run
    /// # use std::fs::File;
    /// use cloudfn::image::Classifier;
    /// let file = File::open("foo.jpg").unwrap();
    /// let tags = file.classify().unwrap();
    /// ```
    fn classify(self) -> Result<Vec<Tag>>;
    fn classify_with<C>(self, &C) -> Result<Classification<C::Extra>> where C: ClassifierAlgo;
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub confidence: f32,
    pub class: String
}

#[derive(Debug)]
pub struct Classification<E> {
    pub tags: Vec<Tag>,
    pub extra: E,
}

pub trait ClassifierAlgo {
    type Extra = ();
    type Input: Serialize;
    type Output: DeserializeOwned;

    fn algo() -> &'static str;
    fn make_input(&self, data_uri: &str) -> Self::Input;
    fn from_output(&self, output: Self::Output) -> Classification<Self::Extra>;
}

#[derive(Serialize)]
#[doc(hidden)]
pub struct ImageWrapper {
    image: String,
}

impl <I: ImageInterop> Classifier for I {
    fn classify(self) -> Result<Vec<Tag>> {
        let input = self.into_data_file()?.to_data_uri();
        let resp = tag(&classifiers::InceptionNet::default(), &input)?;
        Ok(resp.tags)
    }

    fn classify_with<C>(self, classifier_algo: &C) -> Result<Classification<C::Extra>> where C: ClassifierAlgo {
        let input = self.into_data_file()?.to_data_uri();
        let resp = tag(classifier_algo.borrow(), &input)?;
        Ok(resp)
    }
}


fn tag<C: ClassifierAlgo>(classifier_algo: &C, data_uri: &str) -> Result<Classification<C::Extra>> {
        let resp = CLIENT.algo(C::algo())
            .pipe(&classifier_algo.make_input(data_uri))?
            .decode::<C::Output>()?;
        Ok(classifier_algo.from_output(resp))
}

pub mod classifiers {
    use super::*;

    /////////////////////////////////////////////////////////////////////////////////
    // InceptionNet
    /////////////////////////////////////////////////////////////////////////////////

    #[derive(Deserialize)]
    #[doc(hidden)]
    pub struct InceptionNetOutput {
        tags: Vec<Tag>
    }

    #[derive(Default)]
    pub struct InceptionNet;
    impl ClassifierAlgo for InceptionNet {
        type Extra = ();
        type Input = String;
        type Output = InceptionNetOutput;
        fn algo() -> &'static str { "deeplearning/InceptionNet/1.0" }
        fn make_input(&self, data_uri: &str) -> Self::Input { data_uri.to_owned() }
        fn from_output(&self, output: Self::Output) -> Classification<Self::Extra> {
            Classification { tags: output.tags, extra: () }
        }
    }


    /////////////////////////////////////////////////////////////////////////////////
    // DeepFace
    /////////////////////////////////////////////////////////////////////////////////

    #[derive(Deserialize)]
    #[doc(hidden)]
    pub struct DeepFaceOutput {
        results: Vec<(f32, String)>,
    }

    #[derive(Default)]
    pub struct DeepFace;
    impl ClassifierAlgo for DeepFace {
        type Extra = ();
        type Input = ImageWrapper;
        type Output = DeepFaceOutput;
        fn algo() -> &'static str { "deeplearning/DeepFaceRecognition/0.1" }
        fn make_input(&self, data_uri: &str) -> Self::Input {
            ImageWrapper{ image: data_uri.to_owned() }
        }
        fn from_output(&self, output: Self::Output) -> Classification<Self::Extra> {
            Classification {
                tags: output.results.into_iter().map(|p| Tag { confidence: p.0, class: p.1 } ).collect(),
                extra: (),
            }
        }
    }

    /////////////////////////////////////////////////////////////////////////////////
    // Subreddit
    /////////////////////////////////////////////////////////////////////////////////

    #[derive(Deserialize)]
    #[doc(hidden)]
    pub struct SubredditOutput {
        first: SubredditTag,
        second: SubredditTag,
        third: SubredditTag,
    }

    #[derive(Deserialize)]
    #[serde(rename_all="camelCase")]
    #[doc(hidden)]
    pub struct SubredditTag {
        class_name: String,
        confidence: f32,
    }

    #[derive(Default)]
    pub struct Subreddit;
    impl ClassifierAlgo for Subreddit {
        type Extra = ();
        type Input = String;
        type Output = SubredditOutput;
        fn algo() -> &'static str { "deeplearning/SubredditClassifier/0.1" }
        fn make_input(&self, data_uri: &str) -> Self::Input {
            data_uri.to_owned()
        }
        fn from_output(&self, output: Self::Output) -> Classification<Self::Extra> {
            Classification {
                tags: vec![
                    Tag { class: output.first.class_name, confidence: output.first.confidence },
                    Tag { class: output.second.class_name, confidence: output.second.confidence },
                    Tag { class: output.third.class_name, confidence: output.third.confidence },
                ],
                extra: (),
            }
        }
    }


    /////////////////////////////////////////////////////////////////////////////////
    // Places365
    /////////////////////////////////////////////////////////////////////////////////

    #[derive(Deserialize)]
    #[doc(hidden)]
    pub struct PlacesOutput {
        predictions: Vec<PlacesTags>,
    }

    #[derive(Deserialize)]
    #[doc(hidden)]
    pub struct PlacesTags {
        class: String,
        prob: f32,
    }

    #[derive(Default)]
    pub struct Places365;
    impl ClassifierAlgo for Places365 {
        type Extra = ();
        type Input = ImageWrapper;
        type Output = PlacesOutput;
        fn algo() -> &'static str { "deeplearning/Places365Classifier/0.1" }
        fn make_input(&self, data_uri: &str) -> Self::Input {
            ImageWrapper{ image: data_uri.to_owned() }
        }
        fn from_output(&self, output: Self::Output) -> Classification<Self::Extra> {
            Classification {
                tags: output.predictions.into_iter().map(|t| Tag { confidence: t.prob, class: t.class } ).collect(),
                extra: (),
            }
        }
    }


    /////////////////////////////////////////////////////////////////////////////////
    // Real Estate
    /////////////////////////////////////////////////////////////////////////////////

    #[derive(Debug, Deserialize)]
    #[doc(hidden)]
    pub struct RealEstateOutput {
        predictions: Vec<RealEstateTags>,
    }

    #[derive(Debug, Deserialize)]
    #[doc(hidden)]
    pub struct RealEstateTags {
        class: String,
        prob: f32,
    }

    #[derive(Serialize)]
    #[doc(hidden)]
    #[serde(rename_all = "camelCase")]
    pub struct RealEstateInput {
        image: String,
        num_results: Option<u32>,
    }

    #[derive(Default)]
    pub struct RealEstate {
        pub num_results: Option<u32>,
    }

    impl ClassifierAlgo for RealEstate {
        type Extra = ();
        type Input = RealEstateInput;
        type Output = RealEstateOutput;
        fn algo() -> &'static str { "deeplearning/RealEstateClassifier/0.2" }
        fn make_input(&self, data_uri: &str) -> Self::Input {
            RealEstateInput{
                image: data_uri.to_owned(),
                num_results: self.num_results,
            }
        }
        fn from_output(&self, output: Self::Output) -> Classification<Self::Extra> {
            Classification {
                tags: output.predictions.into_iter().map(|t| Tag { confidence: t.prob, class: t.class } ).collect(),
                extra: (),
            }
        }
    }

    /////////////////////////////////////////////////////////////////////////////////
    // Illustration
    /////////////////////////////////////////////////////////////////////////////////


    #[derive(Debug, Deserialize)]
    #[doc(hidden)]
    pub struct IllustrationOutput {
        character: Vec<HashMap<String, f32>>,
        copyright: Vec<HashMap<String, f32>>,
        rating: Vec<HashMap<String, f32>>,
        general: Vec<HashMap<String, f32>>,
    }

    #[derive(Serialize)]
    #[doc(hidden)]
    #[serde(rename_all = "camelCase")]
    pub struct IllustrationInput {
        image: String,

        #[serde(skip_serializing_if="Option::is_none")]
        tags: Option<Vec<String>>,
    }

    #[derive(Debug)]
    pub struct IllustrationExtra {
        pub character: Vec<Tag>,
        pub copyright: Vec<Tag>,
        pub rating: Vec<Tag>,
    }

    #[derive(Default, Debug)]
    pub struct Illustration {
        pub whitelist: Vec<String>,
    }

    fn to_tags(tags: &[HashMap<String, f32>]) -> Vec<Tag> {
        tags.into_iter().map(|t| {
            let (k, v) = t.iter().next().unwrap();
            Tag { class: k.clone(), confidence: *v }
        }).collect()
    }

    impl ClassifierAlgo for Illustration {
        type Extra = IllustrationExtra;
        type Input = IllustrationInput;
        type Output = IllustrationOutput;
        fn algo() -> &'static str { "deeplearning/IllustrationTagger/0.2" }
        fn make_input(&self, data_uri: &str) -> Self::Input {
            let tags = if self.whitelist.is_empty() {
                None
            } else {
                Some(self.whitelist.clone())
            };
            IllustrationInput{
                image: data_uri.to_owned(),
                tags: tags,
            }
        }
        fn from_output(&self, output: Self::Output) -> Classification<Self::Extra> {
            Classification {
                tags: to_tags(&output.general),
                extra: IllustrationExtra {
                    character: to_tags(&output.character),
                    copyright: to_tags(&output.copyright),
                    rating: to_tags(&output.rating),
                }
            }
        }
    }
}


