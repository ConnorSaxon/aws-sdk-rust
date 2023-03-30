// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBucketAccelerateConfiguration`](crate::client::fluent_builders::GetBucketAccelerateConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::GetBucketAccelerateConfiguration::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::GetBucketAccelerateConfiguration::set_bucket): <p>The name of the bucket for which the accelerate configuration is retrieved.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::GetBucketAccelerateConfiguration::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::GetBucketAccelerateConfiguration::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`GetBucketAccelerateConfigurationOutput`](crate::output::GetBucketAccelerateConfigurationOutput) with field(s):
    ///   - [`status(Option<BucketAccelerateStatus>)`](crate::output::GetBucketAccelerateConfigurationOutput::status): <p>The accelerate configuration of the bucket.</p>
                            /// - On failure, responds with [`SdkError<GetBucketAccelerateConfigurationError>`](crate::error::GetBucketAccelerateConfigurationError)
    pub fn get_bucket_accelerate_configuration(&self) -> crate::client::fluent_builders::GetBucketAccelerateConfiguration {
                                crate::client::fluent_builders::GetBucketAccelerateConfiguration::new(self.handle.clone())
                            }
}

