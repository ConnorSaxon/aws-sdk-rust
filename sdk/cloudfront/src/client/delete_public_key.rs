// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePublicKey`](crate::client::fluent_builders::DeletePublicKey) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::DeletePublicKey::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::DeletePublicKey::set_id): <p>The ID of the public key you want to remove from CloudFront.</p>
    ///   - [`if_match(impl Into<String>)`](crate::client::fluent_builders::DeletePublicKey::if_match) / [`set_if_match(Option<String>)`](crate::client::fluent_builders::DeletePublicKey::set_if_match): <p>The value of the <code>ETag</code> header that you received when retrieving the public key identity to delete. For example: <code>E2QWRUHAPOMQZL</code>.</p>
                            /// - On success, responds with [`DeletePublicKeyOutput`](crate::output::DeletePublicKeyOutput)
                            /// - On failure, responds with [`SdkError<DeletePublicKeyError>`](crate::error::DeletePublicKeyError)
    pub fn delete_public_key(&self) -> crate::client::fluent_builders::DeletePublicKey {
                                crate::client::fluent_builders::DeletePublicKey::new(self.handle.clone())
                            }
}

