// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListBucketInventoryConfigurations`](crate::client::fluent_builders::ListBucketInventoryConfigurations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::ListBucketInventoryConfigurations::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::ListBucketInventoryConfigurations::set_bucket): <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    ///   - [`continuation_token(impl Into<String>)`](crate::client::fluent_builders::ListBucketInventoryConfigurations::continuation_token) / [`set_continuation_token(Option<String>)`](crate::client::fluent_builders::ListBucketInventoryConfigurations::set_continuation_token): <p>The marker used to continue an inventory configuration listing that has been truncated. Use the NextContinuationToken from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::ListBucketInventoryConfigurations::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::ListBucketInventoryConfigurations::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`ListBucketInventoryConfigurationsOutput`](crate::output::ListBucketInventoryConfigurationsOutput) with field(s):
    ///   - [`continuation_token(Option<String>)`](crate::output::ListBucketInventoryConfigurationsOutput::continuation_token): <p>If sent in the request, the marker that is used as a starting point for this inventory configuration list response.</p>
    ///   - [`inventory_configuration_list(Option<Vec<InventoryConfiguration>>)`](crate::output::ListBucketInventoryConfigurationsOutput::inventory_configuration_list): <p>The list of inventory configurations for a bucket.</p>
    ///   - [`is_truncated(bool)`](crate::output::ListBucketInventoryConfigurationsOutput::is_truncated): <p>Tells whether the returned list of inventory configurations is complete. A value of true indicates that the list is not complete and the NextContinuationToken is provided for a subsequent request.</p>
    ///   - [`next_continuation_token(Option<String>)`](crate::output::ListBucketInventoryConfigurationsOutput::next_continuation_token): <p>The marker used to continue this inventory configuration listing. Use the <code>NextContinuationToken</code> from this response to continue the listing in a subsequent request. The continuation token is an opaque value that Amazon S3 understands.</p>
                            /// - On failure, responds with [`SdkError<ListBucketInventoryConfigurationsError>`](crate::error::ListBucketInventoryConfigurationsError)
    pub fn list_bucket_inventory_configurations(&self) -> crate::client::fluent_builders::ListBucketInventoryConfigurations {
                                crate::client::fluent_builders::ListBucketInventoryConfigurations::new(self.handle.clone())
                            }
}

