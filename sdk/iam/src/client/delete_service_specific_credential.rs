// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteServiceSpecificCredential`](crate::client::fluent_builders::DeleteServiceSpecificCredential) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::client::fluent_builders::DeleteServiceSpecificCredential::user_name) / [`set_user_name(Option<String>)`](crate::client::fluent_builders::DeleteServiceSpecificCredential::set_user_name): <p>The name of the IAM user associated with the service-specific credential. If this value is not specified, then the operation assumes the user whose credentials are used to call the operation.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`service_specific_credential_id(impl Into<String>)`](crate::client::fluent_builders::DeleteServiceSpecificCredential::service_specific_credential_id) / [`set_service_specific_credential_id(Option<String>)`](crate::client::fluent_builders::DeleteServiceSpecificCredential::set_service_specific_credential_id): <p>The unique identifier of the service-specific credential. You can get this value by calling <code>ListServiceSpecificCredentials</code>.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
                            /// - On success, responds with [`DeleteServiceSpecificCredentialOutput`](crate::output::DeleteServiceSpecificCredentialOutput)
                            /// - On failure, responds with [`SdkError<DeleteServiceSpecificCredentialError>`](crate::error::DeleteServiceSpecificCredentialError)
    pub fn delete_service_specific_credential(&self) -> crate::client::fluent_builders::DeleteServiceSpecificCredential {
                                crate::client::fluent_builders::DeleteServiceSpecificCredential::new(self.handle.clone())
                            }
}

