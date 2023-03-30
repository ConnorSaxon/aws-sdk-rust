// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetPublicKey`](crate::client::fluent_builders::GetPublicKey) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::client::fluent_builders::GetPublicKey::id) / [`set_id(Option<String>)`](crate::client::fluent_builders::GetPublicKey::set_id): <p>The identifier of the public key you are getting.</p>
                            /// - On success, responds with [`GetPublicKeyOutput`](crate::output::GetPublicKeyOutput) with field(s):
    ///   - [`public_key(Option<PublicKey>)`](crate::output::GetPublicKeyOutput::public_key): <p>The public key.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::GetPublicKeyOutput::e_tag): <p>The identifier for this version of the public key.</p>
                            /// - On failure, responds with [`SdkError<GetPublicKeyError>`](crate::error::GetPublicKeyError)
    pub fn get_public_key(&self) -> crate::client::fluent_builders::GetPublicKey {
                                crate::client::fluent_builders::GetPublicKey::new(self.handle.clone())
                            }
}

