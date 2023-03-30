// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetOpenIdToken`](crate::client::fluent_builders::GetOpenIdToken) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity_id(impl Into<String>)`](crate::client::fluent_builders::GetOpenIdToken::identity_id) / [`set_identity_id(Option<String>)`](crate::client::fluent_builders::GetOpenIdToken::set_identity_id): <p>A unique identifier in the format REGION:GUID.</p>
    ///   - [`logins(HashMap<String, String>)`](crate::client::fluent_builders::GetOpenIdToken::logins) / [`set_logins(Option<HashMap<String, String>>)`](crate::client::fluent_builders::GetOpenIdToken::set_logins): <p>A set of optional name-value pairs that map provider names to provider tokens. When using graph.facebook.com and www.amazon.com, supply the access_token returned from the provider's authflow. For accounts.google.com, an Amazon Cognito user pool provider, or any other OpenID Connect provider, always include the <code>id_token</code>.</p>
                            /// - On success, responds with [`GetOpenIdTokenOutput`](crate::output::GetOpenIdTokenOutput) with field(s):
    ///   - [`identity_id(Option<String>)`](crate::output::GetOpenIdTokenOutput::identity_id): <p>A unique identifier in the format REGION:GUID. Note that the IdentityId returned may not match the one passed on input.</p>
    ///   - [`token(Option<String>)`](crate::output::GetOpenIdTokenOutput::token): <p>An OpenID token, valid for 10 minutes.</p>
                            /// - On failure, responds with [`SdkError<GetOpenIdTokenError>`](crate::error::GetOpenIdTokenError)
    pub fn get_open_id_token(&self) -> crate::client::fluent_builders::GetOpenIdToken {
                                crate::client::fluent_builders::GetOpenIdToken::new(self.handle.clone())
                            }
}

