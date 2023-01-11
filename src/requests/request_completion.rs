use crate::models::Models;
use crate::errors::Result;
use serde::{Deserialize, Serialize};
use crate::client::OpenAPIClient;
use crate::settings::APISettings;

#[derive(Clone, Serialize)]
pub struct CompletionRequest {

    /// ID of the models to use.
    /// You can use the List models API to see all of your available models,
    /// or see our Model overview for descriptions of them.
    ///
    /// [Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-model)
    pub engine: Models,

    /// The prompt(s) to generate completions for, encoded as a string.
    ///
    /// Note that <|endoftext|> is the document separator that the models sees during training,
    /// so if a prompt is not specified the models will generate as if from the beginning of a new document.
    pub prompt: String,

    /// The suffix that comes after a completion of inserted text.
    pub suffix: Option<String>,

    /// What sampling temperature to use. Higher values means the models will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling)
    /// for ones with a well-defined answer.
    /// We generally recommend altering this or `top_p` but not both.
    pub temperature: f32,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the models considers the results of the tokens with `top_p` probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    /// We generally recommend altering this or `temperature` but not both.
    pub top_p: f32,

    /// The maximum number of tokens to generate in the completion.
    /// The token count of your prompt plus `max_tokens` cannot exceed the models's context length.
    /// Most models have a context length of 2048 tokens
    /// (except for the newest models, which support 4096).
    pub max_token: u16,

    /// How many completions to generate for each prompt.
    ///
    /// Note: Because this parameter generates many completions,
    /// it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and stop.
    pub n: u16,

    /// Whether to stream back partial progress. If set, tokens will be sent as data-only
    /// server-sent events as they become available,
    /// with the stream terminated by a data: [DONE] message.
    pub stream: bool,

    /// Include the log probabilities on the logprobs most likely tokens, as well the chosen tokens.
    /// For example, if logprobs is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the logprob of the sampled token,
    /// so there may be up to logprobs+1 elements in the response.
    ///
    /// The maximum value for logprobs is 5.
    /// If you need more than this, please contact us through our Help center and describe your use case.
    pub log_probs: Option<u16>,

    /// Echo back the prompt in addition to the completion
    pub echo: bool,

    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the stop sequence.
    pub stop: Option<String>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether
    /// they appear in the text so far, increasing the models's likelihood to talk about new topics.
    ///
    /// [Reference](https://beta.openai.com/docs/api-reference/completions/create#completions/create-presence_penalty)
    pub presence_penalty: f32,

    /// Generates best_of completions server-side and returns the "best"
    /// (the one with the highest log probability per token). Results cannot be streamed.
    /// When used with `n`, `best_of` controls the number of candidate completions and `n` specifies
    /// how many to return – `best_of` must be greater than `n`.
    /// Note: Because this parameter generates many completions, it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    pub best_of: u16,

    pub user: Option<String>,
}

impl CompletionRequest {
    fn set_prompt(&mut self, prompt: String) -> &mut Self {
        self.prompt = prompt;
        self
    }

    fn set_suffix(&mut self, suffix: Option<String>) -> &mut Self {
        self.suffix = suffix;
        self
    }

    fn set_temperature(&mut self, temperature: f32) -> &mut Self {
        self.temperature = temperature;
        self
    }

    fn set_top_p(&mut self, top_p: f32) -> &mut Self {
        self.top_p = top_p;
        self
    }

    fn set_max_token(&mut self, max_token: u16) -> &mut Self {
        self.max_token = max_token;
        self
    }

    fn set_n(&mut self, n: u16) -> &mut Self {
        self.n = n;
        self
    }

    fn set_stream(&mut self, stream: bool) -> &mut Self {
        self.stream = stream;
        self
    }

    fn set_log_probs(&mut self, log_probs: Option<u16>) -> &mut Self {
        self.log_probs = log_probs;
        self
    }

    fn set_echo(&mut self, echo: bool) -> &mut Self {
        self.echo = echo;
        self
    }

    fn set_stop(&mut self, stop: Option<String>) -> &mut Self {
        self.stop = stop;
        self
    }

    fn set_presence_penalty(&mut self, presence_penalty: f32) -> &mut Self {
        self.presence_penalty = presence_penalty;
        self
    }

    fn set_best_of(&mut self, best_of: u16) -> &mut Self {
        self.best_of = best_of;
        self
    }

    fn set_user(&mut self, user: Option<String>) -> &mut Self {
        self.user = user;
        self
    }
}

#[derive(Clone, Deserialize)]
pub struct CompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage
}

#[derive(Clone, Deserialize)]
struct Choice {
    text: String,
    index: u16,
    log_probs: Option<String>,
    finish_reason: String
}

#[derive(Clone, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32
}

impl Default for CompletionRequest {
    fn default() -> Self {
        Self {
            engine: Models::TextSimilarityDavinci001,
            prompt: "".to_string(),
            suffix: None,
            temperature: 0.0,
            top_p: 1.0,
            max_token: 2048,
            n: 1,
            stream: false,
            log_probs: None,
            echo: false,
            stop: None,
            presence_penalty: 0.0,
            best_of: 1,
            user: None
        }
    }
}

struct Completion {
    client: OpenAPIClient,
    base: CompletionRequest
}

impl Completion  {
    pub fn new(settings : APISettings) -> Self {
        Completion {
            client: OpenAPIClient::new(settings),
            base: CompletionRequest::default()
        }
    }

    pub fn save_settings(&mut self, request_base: CompletionRequest) {
        self.base = request_base;
    }

    pub async fn execute(&mut self, prompt: String) -> Result<CompletionResponse> {
        let request = self.base.set_prompt(prompt).clone();
        self.client.post("/completions", request).await
    }

    pub async fn execute_with(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.client.post("/completions", request).await
    }
}