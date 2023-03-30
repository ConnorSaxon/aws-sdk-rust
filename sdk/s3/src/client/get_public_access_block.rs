// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPublicAccessBlock`](crate::client::fluent_builders::GetPublicAccessBlock) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::GetPublicAccessBlock::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::GetPublicAccessBlock::set_bucket): <p>The name of the Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to retrieve. </p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::GetPublicAccessBlock::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::GetPublicAccessBlock::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`GetPublicAccessBlockOutput`](crate::output::GetPublicAccessBlockOutput) with field(s):
    ///   - [`public_access_block_configuration(Option<PublicAccessBlockConfiguration>)`](crate::output::GetPublicAccessBlockOutput::public_access_block_configuration): <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
                            /// - On failure, responds with [`SdkError<GetPublicAccessBlockError>`](crate::error::GetPublicAccessBlockError)
    pub fn get_public_access_block(&self) -> crate::client::fluent_builders::GetPublicAccessBlock {
                                crate::client::fluent_builders::GetPublicAccessBlock::new(self.handle.clone())
                            }
}

