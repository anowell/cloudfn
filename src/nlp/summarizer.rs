use error::Result;
use CLIENT;

/// Summarize text
///
/// This currently uses [classifier4j](classifier4j/index.html)
/// but the specific implementation may change in semver-breaking releases
pub fn summarize(text: &str) -> Result<String> {
    self::classifier4j::summarize(text)
}

pub mod classifier4j {
    use super::*;

    /// Summarize text using [nlp/summarizer](https://algorithmia.com/algorithms/nlp/summarizer)
    pub fn summarize(text: &str) -> Result<String> {
        let resp = CLIENT.algo("nlp/summarizer/0.1")
            .pipe(text)?
            .into_string();
        match resp {
            Some(summary) => Ok(summary),
            None => bail!("Algorithm did not output a string")
        }
    }
}


pub mod summarai {
    use super::*;

    #[derive(Deserialize)]
    struct Output {
        summarized_data: String,
        auto_gen_ranked_keywords: Vec<String>,
    }

    pub struct Summary {
        pub summary: String,
        pub keywords: Vec<String>,
    }

    /// Summarize text using [SummarAI/Summarizer](https://algorithmia.com/algorithms/SummarAI/Summarizer)
    pub fn summarize(text: &str) -> Result<Summary> {
        let resp = CLIENT.algo("SummarAI/Summarizer/0.1")
            .pipe(text)?
            .decode::<Output>()?;

        Ok(Summary {
            summary: resp.summarized_data,
            keywords: resp.auto_gen_ranked_keywords,
        })
    }
}
