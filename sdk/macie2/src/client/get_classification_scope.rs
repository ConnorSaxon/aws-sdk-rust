// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetClassificationScope`](crate::client::fluent_builders::GetClassificationScope) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetClassificationScope::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetClassificationScope::set_id): <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
                            /// - On success, responds with [`GetClassificationScopeOutput`](crate::output::GetClassificationScopeOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::GetClassificationScopeOutput::id): <p>The unique identifier for the classification scope.</p>
    ///   - [`name(Option<String>)`](crate::output::GetClassificationScopeOutput::name): <p>The name of the classification scope.</p>
    ///   - [`s3(Option<S3ClassificationScope>)`](crate::output::GetClassificationScopeOutput::s3): <p>The S3 buckets that are excluded from automated sensitive data discovery.</p>
                            /// - On failure, responds with [`SdkError<GetClassificationScopeError>`](crate::error::GetClassificationScopeError)
    pub fn get_classification_scope(&self) -> crate::client::fluent_builders::GetClassificationScope {
                                crate::client::fluent_builders::GetClassificationScope::new(self.handle.clone())
                            }
}

