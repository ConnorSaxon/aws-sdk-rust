// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutBucketMetricsConfiguration`](crate::client::fluent_builders::PutBucketMetricsConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::set_bucket): <p>The name of the bucket for which the metrics configuration is set.</p>
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::set_id): <p>The ID used to identify the metrics configuration.</p>
    ///   - [`metrics_configuration(MetricsConfiguration)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::metrics_configuration) / [`set_metrics_configuration(Option<MetricsConfiguration>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::set_metrics_configuration): <p>Specifies the metrics configuration.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::PutBucketMetricsConfiguration::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`PutBucketMetricsConfigurationOutput`](crate::output::PutBucketMetricsConfigurationOutput)
                            /// - On failure, responds with [`SdkError<PutBucketMetricsConfigurationError>`](crate::error::PutBucketMetricsConfigurationError)
    pub fn put_bucket_metrics_configuration(&self) -> crate::client::fluent_builders::PutBucketMetricsConfiguration {
                                crate::client::fluent_builders::PutBucketMetricsConfiguration::new(self.handle.clone())
                            }
}

