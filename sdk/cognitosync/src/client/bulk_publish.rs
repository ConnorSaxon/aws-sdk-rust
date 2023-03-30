// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BulkPublish`](crate::client::fluent_builders::BulkPublish) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_pool_id(impl Into<String>)`](crate::client::fluent_builders::BulkPublish::identity_pool_id) / [`set_identity_pool_id(Option<String>)`](crate::client::fluent_builders::BulkPublish::set_identity_pool_id): A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
                            /// - On success, responds with [`BulkPublishOutput`](crate::output::BulkPublishOutput) with field(s):
    ///   - [`identity_pool_id(Option<String>)`](crate::output::BulkPublishOutput::identity_pool_id): A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.
                            /// - On failure, responds with [`SdkError<BulkPublishError>`](crate::error::BulkPublishError)
    pub fn bulk_publish(&self) -> crate::client::fluent_builders::BulkPublish {
                                crate::client::fluent_builders::BulkPublish::new(self.handle.clone())
                            }
}

