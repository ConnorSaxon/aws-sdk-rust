// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateInvalidation`](crate::client::fluent_builders::CreateInvalidation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`distribution_id(impl Into<String>)`](crate::client::fluent_builders::CreateInvalidation::distribution_id) / [`set_distribution_id(Option<String>)`](crate::client::fluent_builders::CreateInvalidation::set_distribution_id): <p>The distribution's id.</p>
    ///   - [`invalidation_batch(InvalidationBatch)`](crate::client::fluent_builders::CreateInvalidation::invalidation_batch) / [`set_invalidation_batch(Option<InvalidationBatch>)`](crate::client::fluent_builders::CreateInvalidation::set_invalidation_batch): <p>The batch information for the invalidation.</p>
                            /// - On success, responds with [`CreateInvalidationOutput`](crate::output::CreateInvalidationOutput) with field(s):
    ///   - [`location(Option<String>)`](crate::output::CreateInvalidationOutput::location): <p>The fully qualified URI of the distribution and invalidation batch request, including the <code>Invalidation ID</code>.</p>
    ///   - [`invalidation(Option<Invalidation>)`](crate::output::CreateInvalidationOutput::invalidation): <p>The invalidation's information.</p>
                            /// - On failure, responds with [`SdkError<CreateInvalidationError>`](crate::error::CreateInvalidationError)
    pub fn create_invalidation(&self) -> crate::client::fluent_builders::CreateInvalidation {
                                crate::client::fluent_builders::CreateInvalidation::new(self.handle.clone())
                            }
}

