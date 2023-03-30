// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteEntityRecognizer`](crate::client::fluent_builders::DeleteEntityRecognizer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`entity_recognizer_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteEntityRecognizer::entity_recognizer_arn) / [`set_entity_recognizer_arn(Option<String>)`](crate::client::fluent_builders::DeleteEntityRecognizer::set_entity_recognizer_arn): <p>The Amazon Resource Name (ARN) that identifies the entity recognizer.</p>
                            /// - On success, responds with [`DeleteEntityRecognizerOutput`](crate::output::DeleteEntityRecognizerOutput)
                            /// - On failure, responds with [`SdkError<DeleteEntityRecognizerError>`](crate::error::DeleteEntityRecognizerError)
    pub fn delete_entity_recognizer(&self) -> crate::client::fluent_builders::DeleteEntityRecognizer {
                                crate::client::fluent_builders::DeleteEntityRecognizer::new(self.handle.clone())
                            }
}

