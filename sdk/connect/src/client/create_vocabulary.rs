// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateVocabulary`](crate::client::fluent_builders::CreateVocabulary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateVocabulary::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateVocabulary::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>. If a create request is received more than once with same client token, subsequent requests return the previous response without creating a vocabulary again.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::CreateVocabulary::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::CreateVocabulary::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`vocabulary_name(impl Into<String>)`](crate::client::fluent_builders::CreateVocabulary::vocabulary_name) / [`set_vocabulary_name(Option<String>)`](crate::client::fluent_builders::CreateVocabulary::set_vocabulary_name): <p>A unique name of the custom vocabulary.</p>
    ///   - [`language_code(VocabularyLanguageCode)`](crate::client::fluent_builders::CreateVocabulary::language_code) / [`set_language_code(Option<VocabularyLanguageCode>)`](crate::client::fluent_builders::CreateVocabulary::set_language_code): <p>The language code of the vocabulary entries. For a list of languages and their corresponding language codes, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/transcribe-whatis.html">What is Amazon Transcribe?</a> </p>
    ///   - [`content(impl Into<String>)`](crate::client::fluent_builders::CreateVocabulary::content) / [`set_content(Option<String>)`](crate::client::fluent_builders::CreateVocabulary::set_content): <p>The content of the custom vocabulary in plain-text format with a table of values. Each row in the table represents a word or a phrase, described with <code>Phrase</code>, <code>IPA</code>, <code>SoundsLike</code>, and <code>DisplayAs</code> fields. Separate the fields with TAB characters. The size limit is 50KB. For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/custom-vocabulary.html#create-vocabulary-table">Create a custom vocabulary using a table</a>.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateVocabulary::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateVocabulary::set_tags): <p>The tags used to organize, track, or control access for this resource. For example, { "tags": {"key1":"value1", "key2":"value2"} }.</p>
                            /// - On success, responds with [`CreateVocabularyOutput`](crate::output::CreateVocabularyOutput) with field(s):
    ///   - [`vocabulary_arn(Option<String>)`](crate::output::CreateVocabularyOutput::vocabulary_arn): <p>The Amazon Resource Name (ARN) of the custom vocabulary.</p>
    ///   - [`vocabulary_id(Option<String>)`](crate::output::CreateVocabularyOutput::vocabulary_id): <p>The identifier of the custom vocabulary.</p>
    ///   - [`state(Option<VocabularyState>)`](crate::output::CreateVocabularyOutput::state): <p>The current state of the custom vocabulary.</p>
                            /// - On failure, responds with [`SdkError<CreateVocabularyError>`](crate::error::CreateVocabularyError)
    pub fn create_vocabulary(&self) -> crate::client::fluent_builders::CreateVocabulary {
                                crate::client::fluent_builders::CreateVocabulary::new(self.handle.clone())
                            }
}

