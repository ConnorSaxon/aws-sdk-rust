// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAuthorizer`](crate::client::fluent_builders::DeleteAuthorizer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAuthorizer::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::DeleteAuthorizer::set_api_id): <p>The API identifier.</p>
    ///   - [`authorizer_id(impl Into<String>)`](crate::client::fluent_builders::DeleteAuthorizer::authorizer_id) / [`set_authorizer_id(Option<String>)`](crate::client::fluent_builders::DeleteAuthorizer::set_authorizer_id): <p>The authorizer identifier.</p>
                            /// - On success, responds with [`DeleteAuthorizerOutput`](crate::output::DeleteAuthorizerOutput)
                            /// - On failure, responds with [`SdkError<DeleteAuthorizerError>`](crate::error::DeleteAuthorizerError)
    pub fn delete_authorizer(&self) -> crate::client::fluent_builders::DeleteAuthorizer {
                                crate::client::fluent_builders::DeleteAuthorizer::new(self.handle.clone())
                            }
}

