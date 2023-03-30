// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelKeyDeletion`](crate::client::fluent_builders::CancelKeyDeletion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::client::fluent_builders::CancelKeyDeletion::key_id) / [`set_key_id(Option<String>)`](crate::client::fluent_builders::CancelKeyDeletion::set_key_id): <p>Identifies the KMS key whose deletion is being canceled.</p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
                            /// - On success, responds with [`CancelKeyDeletionOutput`](crate::output::CancelKeyDeletionOutput) with field(s):
    ///   - [`key_id(Option<String>)`](crate::output::CancelKeyDeletionOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key whose deletion is canceled.</p>
                            /// - On failure, responds with [`SdkError<CancelKeyDeletionError>`](crate::error::CancelKeyDeletionError)
    pub fn cancel_key_deletion(&self) -> crate::client::fluent_builders::CancelKeyDeletion {
                                crate::client::fluent_builders::CancelKeyDeletion::new(self.handle.clone())
                            }
}

