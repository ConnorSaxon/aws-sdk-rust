// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RotateEncryptionKey`](crate::client::fluent_builders::RotateEncryptionKey) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::client::fluent_builders::RotateEncryptionKey::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::client::fluent_builders::RotateEncryptionKey::set_cluster_identifier): <p>The unique identifier of the cluster that you want to rotate the encryption keys for.</p>  <p>Constraints: Must be the name of valid cluster that has encryption enabled.</p>
                            /// - On success, responds with [`RotateEncryptionKeyOutput`](crate::output::RotateEncryptionKeyOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::output::RotateEncryptionKeyOutput::cluster): <p>Describes a cluster.</p>
                            /// - On failure, responds with [`SdkError<RotateEncryptionKeyError>`](crate::error::RotateEncryptionKeyError)
    pub fn rotate_encryption_key(&self) -> crate::client::fluent_builders::RotateEncryptionKey {
                                crate::client::fluent_builders::RotateEncryptionKey::new(self.handle.clone())
                            }
}

