// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeVocabulary`](crate::client::fluent_builders::DescribeVocabulary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::DescribeVocabulary::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::DescribeVocabulary::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`vocabulary_id(impl Into<String>)`](crate::client::fluent_builders::DescribeVocabulary::vocabulary_id) / [`set_vocabulary_id(Option<String>)`](crate::client::fluent_builders::DescribeVocabulary::set_vocabulary_id): <p>The identifier of the custom vocabulary.</p>
                            /// - On success, responds with [`DescribeVocabularyOutput`](crate::output::DescribeVocabularyOutput) with field(s):
    ///   - [`vocabulary(Option<Vocabulary>)`](crate::output::DescribeVocabularyOutput::vocabulary): <p>A list of specific words that you want Contact Lens for Amazon Connect to recognize in your audio input. They are generally domain-specific words and phrases, words that Contact Lens is not recognizing, or proper nouns.</p>
                            /// - On failure, responds with [`SdkError<DescribeVocabularyError>`](crate::error::DescribeVocabularyError)
    pub fn describe_vocabulary(&self) -> crate::client::fluent_builders::DescribeVocabulary {
                                crate::client::fluent_builders::DescribeVocabulary::new(self.handle.clone())
                            }
}

