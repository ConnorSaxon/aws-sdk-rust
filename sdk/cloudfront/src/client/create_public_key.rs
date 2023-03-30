// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreatePublicKey`](crate::client::fluent_builders::CreatePublicKey) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`public_key_config(PublicKeyConfig)`](crate::client::fluent_builders::CreatePublicKey::public_key_config) / [`set_public_key_config(Option<PublicKeyConfig>)`](crate::client::fluent_builders::CreatePublicKey::set_public_key_config): <p>A CloudFront public key configuration.</p>
                            /// - On success, responds with [`CreatePublicKeyOutput`](crate::output::CreatePublicKeyOutput) with field(s):
    ///   - [`public_key(Option<PublicKey>)`](crate::output::CreatePublicKeyOutput::public_key): <p>The public key.</p>
    ///   - [`location(Option<String>)`](crate::output::CreatePublicKeyOutput::location): <p>The URL of the public key.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::CreatePublicKeyOutput::e_tag): <p>The identifier for this version of the public key.</p>
                            /// - On failure, responds with [`SdkError<CreatePublicKeyError>`](crate::error::CreatePublicKeyError)
    pub fn create_public_key(&self) -> crate::client::fluent_builders::CreatePublicKey {
                                crate::client::fluent_builders::CreatePublicKey::new(self.handle.clone())
                            }
}

