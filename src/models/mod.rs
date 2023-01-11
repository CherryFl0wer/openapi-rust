use serde::Serialize;

/// Name of all the models usable
#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub enum Models {
    TextSimilarityAda001,
    TextSimilarityBabbage001,
    TextSimilarityCurie001,
    TextSimilarityDavinci001,
    TextSearchAdaDoc001,
    TextSearchAdaQuery001,
    TextSearchBabbageDoc001,
    TextSearchBabbageQuery001,
    TextSearchCurieDoc001,
    TextSearchCurieQuery001,
    TextSearchDavinciDoc001,
    TextSearchDavinciQuery001,
    CodeSearchAdaCode001,
    CodeSearchAdaText001,
    CodeSearchBabbageCode001,
    CodeSearchBabbageText001,
}

impl Models {
    /// Return the name in String of the models
    pub fn to_name(self) -> String {
        match self {
            Models::TextSimilarityAda001 => "text-similarity-ada-001".to_string(),
            Models::TextSimilarityBabbage001 => "text-similarity-babbage-001".to_string(),
            Models::TextSimilarityCurie001 => "text-similarity-curie-001".to_string(),
            Models::TextSimilarityDavinci001 => "text-similarity-davinci-001".to_string(),
            Models::TextSearchAdaDoc001 => "text-search-ada-doc-001".to_string(),
            Models::TextSearchAdaQuery001 => "text-search-ada-query-001".to_string(),
            Models::TextSearchBabbageDoc001 => "text-search-babbage-doc-001".to_string(),
            Models::TextSearchBabbageQuery001 => "text-search-babbage-query-001".to_string(),
            Models::TextSearchCurieDoc001 => "text-search-curie-doc-001".to_string(),
            Models::TextSearchCurieQuery001 => "text-search-curie-query-001".to_string(),
            Models::TextSearchDavinciDoc001 => "text-search-davinci-doc-001".to_string(),
            Models::TextSearchDavinciQuery001 => "text-search-davinci-query-001".to_string(),
            Models::CodeSearchAdaCode001 => "code-search-ada-code-001".to_string(),
            Models::CodeSearchAdaText001 => "code-search-ada-text-001".to_string(),
            Models::CodeSearchBabbageCode001 => "code-search-babbage-code-001".to_string(),
            Models::CodeSearchBabbageText001 => "code-search-babbage-text-001".to_string(),
        }
    }
}