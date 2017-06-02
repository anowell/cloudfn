use error::Result;
use CLIENT;

pub struct Summary<E> {
    pub summary: String,
    pub extra: E,
}

pub trait SummarizerAlgo {
    type Extra;
    fn summarize(text: &str) -> Result<Summary<Self::Extra>>;
}

pub struct Classifier4J;
impl SummarizerAlgo for Classifier4J {
    type Extra = ();
    fn summarize(text: &str) -> Result<Summary<Self::Extra>> {
        let resp = CLIENT.algo("nlp/summarizer/0.1")
            .pipe(text)?
            .into_string();
        match resp {
            Some(summary) => Ok(Summary{ summary, extra: () }),
            None => bail!("Algorithm did not output a string")
        }
    }
}

#[derive(Deserialize)]
pub struct SummarAiOutput {
    summarized_data: String,
    auto_gen_ranked_keywords: Vec<String>,
}

pub struct SummarAiExtra {
    pub keywords: Vec<String>,
}

pub struct SummarAi;
impl SummarizerAlgo for SummarAi {
    type Extra = SummarAiExtra;
    fn summarize(text: &str) -> Result<Summary<Self::Extra>> {
        let resp = CLIENT.algo("SummarAI/Summarizer/0.1")
            .pipe(text)?
            .decode::<SummarAiOutput>()?;

        Ok(Summary {
            summary: resp.summarized_data,
            extra: SummarAiExtra { keywords: resp.auto_gen_ranked_keywords },
        })
    }

}
