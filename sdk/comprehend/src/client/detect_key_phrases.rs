// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DetectKeyPhrases`](crate::client::fluent_builders::DetectKeyPhrases) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`text(impl Into<String>)`](crate::client::fluent_builders::DetectKeyPhrases::text) / [`set_text(Option<String>)`](crate::client::fluent_builders::DetectKeyPhrases::set_text): <p>A UTF-8 text string. The string must contain less than 100 KB of UTF-8 encoded characters.</p>
    ///   - [`language_code(LanguageCode)`](crate::client::fluent_builders::DetectKeyPhrases::language_code) / [`set_language_code(Option<LanguageCode>)`](crate::client::fluent_builders::DetectKeyPhrases::set_language_code): <p>The language of the input documents. You can specify any of the primary languages supported by Amazon Comprehend. All documents must be in the same language.</p>
                            /// - On success, responds with [`DetectKeyPhrasesOutput`](crate::output::DetectKeyPhrasesOutput) with field(s):
    ///   - [`key_phrases(Option<Vec<KeyPhrase>>)`](crate::output::DetectKeyPhrasesOutput::key_phrases): <p>A collection of key phrases that Amazon Comprehend identified in the input text. For each key phrase, the response provides the text of the key phrase, where the key phrase begins and ends, and the level of confidence that Amazon Comprehend has in the accuracy of the detection. </p>
                            /// - On failure, responds with [`SdkError<DetectKeyPhrasesError>`](crate::error::DetectKeyPhrasesError)
    pub fn detect_key_phrases(&self) -> crate::client::fluent_builders::DetectKeyPhrases {
                                crate::client::fluent_builders::DetectKeyPhrases::new(self.handle.clone())
                            }
}

