// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteUserAttributes`](crate::client::fluent_builders::DeleteUserAttributes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_attribute_names(Vec<String>)`](crate::client::fluent_builders::DeleteUserAttributes::user_attribute_names) / [`set_user_attribute_names(Option<Vec<String>>)`](crate::client::fluent_builders::DeleteUserAttributes::set_user_attribute_names): <p>An array of strings representing the user attribute names you want to delete.</p>  <p>For custom attributes, you must prependattach the <code>custom:</code> prefix to the front of the attribute name.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::DeleteUserAttributes::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::DeleteUserAttributes::set_access_token): <p>A valid access token that Amazon Cognito issued to the user whose attributes you want to delete.</p>
                            /// - On success, responds with [`DeleteUserAttributesOutput`](crate::output::DeleteUserAttributesOutput)
                            /// - On failure, responds with [`SdkError<DeleteUserAttributesError>`](crate::error::DeleteUserAttributesError)
    pub fn delete_user_attributes(&self) -> crate::client::fluent_builders::DeleteUserAttributes {
                                crate::client::fluent_builders::DeleteUserAttributes::new(self.handle.clone())
                            }
}

