// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteFindingsFilter`](crate::client::fluent_builders::DeleteFindingsFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeleteFindingsFilter::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeleteFindingsFilter::set_id): <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
                            /// - On success, responds with [`DeleteFindingsFilterOutput`](crate::output::DeleteFindingsFilterOutput)
                            /// - On failure, responds with [`SdkError<DeleteFindingsFilterError>`](crate::error::DeleteFindingsFilterError)
    pub fn delete_findings_filter(&self) -> crate::client::fluent_builders::DeleteFindingsFilter {
                                crate::client::fluent_builders::DeleteFindingsFilter::new(self.handle.clone())
                            }
}

