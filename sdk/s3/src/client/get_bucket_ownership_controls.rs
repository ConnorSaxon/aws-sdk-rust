// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBucketOwnershipControls`](crate::client::fluent_builders::GetBucketOwnershipControls) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::GetBucketOwnershipControls::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::GetBucketOwnershipControls::set_bucket): <p>The name of the Amazon S3 bucket whose <code>OwnershipControls</code> you want to retrieve. </p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::GetBucketOwnershipControls::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::GetBucketOwnershipControls::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`GetBucketOwnershipControlsOutput`](crate::output::GetBucketOwnershipControlsOutput) with field(s):
    ///   - [`ownership_controls(Option<OwnershipControls>)`](crate::output::GetBucketOwnershipControlsOutput::ownership_controls): <p>The <code>OwnershipControls</code> (BucketOwnerEnforced, BucketOwnerPreferred, or ObjectWriter) currently in effect for this Amazon S3 bucket.</p>
                            /// - On failure, responds with [`SdkError<GetBucketOwnershipControlsError>`](crate::error::GetBucketOwnershipControlsError)
    pub fn get_bucket_ownership_controls(&self) -> crate::client::fluent_builders::GetBucketOwnershipControls {
                                crate::client::fluent_builders::GetBucketOwnershipControls::new(self.handle.clone())
                            }
}

