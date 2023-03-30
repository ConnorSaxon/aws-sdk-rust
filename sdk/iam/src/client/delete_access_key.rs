// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAccessKey`](crate::client::fluent_builders::DeleteAccessKey) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::client::fluent_builders::DeleteAccessKey::user_name) / [`set_user_name(Option<String>)`](crate::client::fluent_builders::DeleteAccessKey::set_user_name): <p>The name of the user whose access key pair you want to delete.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`access_key_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAccessKey::access_key_id) / [`set_access_key_id(Option<String>)`](crate::client::fluent_builders::DeleteAccessKey::set_access_key_id): <p>The access key ID for the access key ID and secret access key you want to delete.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
                            /// - On success, responds with [`DeleteAccessKeyOutput`](crate::output::DeleteAccessKeyOutput)
                            /// - On failure, responds with [`SdkError<DeleteAccessKeyError>`](crate::error::DeleteAccessKeyError)
    pub fn delete_access_key(&self) -> crate::client::fluent_builders::DeleteAccessKey {
                                crate::client::fluent_builders::DeleteAccessKey::new(self.handle.clone())
                            }
}

