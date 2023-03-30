// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateSoftwareToken`](crate::client::fluent_builders::AssociateSoftwareToken) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::AssociateSoftwareToken::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::AssociateSoftwareToken::set_access_token): <p>A valid access token that Amazon Cognito issued to the user whose software token you want to generate.</p>
    ///   - [`session(impl Into<String>)`](crate::client::fluent_builders::AssociateSoftwareToken::session) / [`set_session(Option<String>)`](crate::client::fluent_builders::AssociateSoftwareToken::set_session): <p>The session that should be passed both ways in challenge-response calls to the service. This allows authentication of the user as part of the MFA setup process.</p>
                            /// - On success, responds with [`AssociateSoftwareTokenOutput`](crate::output::AssociateSoftwareTokenOutput) with field(s):
    ///   - [`secret_code(Option<String>)`](crate::output::AssociateSoftwareTokenOutput::secret_code): <p>A unique generated shared secret code that is used in the TOTP algorithm to generate a one-time code.</p>
    ///   - [`session(Option<String>)`](crate::output::AssociateSoftwareTokenOutput::session): <p>The session that should be passed both ways in challenge-response calls to the service. This allows authentication of the user as part of the MFA setup process.</p>
                            /// - On failure, responds with [`SdkError<AssociateSoftwareTokenError>`](crate::error::AssociateSoftwareTokenError)
    pub fn associate_software_token(&self) -> crate::client::fluent_builders::AssociateSoftwareToken {
                                crate::client::fluent_builders::AssociateSoftwareToken::new(self.handle.clone())
                            }
}

