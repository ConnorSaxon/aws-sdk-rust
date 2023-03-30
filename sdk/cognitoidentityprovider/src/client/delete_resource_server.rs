// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteResourceServer`](crate::client::fluent_builders::DeleteResourceServer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::DeleteResourceServer::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::DeleteResourceServer::set_user_pool_id): <p>The user pool ID for the user pool that hosts the resource server.</p>
    ///   - [`identifier(impl Into<String>)`](crate::client::fluent_builders::DeleteResourceServer::identifier) / [`set_identifier(Option<String>)`](crate::client::fluent_builders::DeleteResourceServer::set_identifier): <p>The identifier for the resource server.</p>
                            /// - On success, responds with [`DeleteResourceServerOutput`](crate::output::DeleteResourceServerOutput)
                            /// - On failure, responds with [`SdkError<DeleteResourceServerError>`](crate::error::DeleteResourceServerError)
    pub fn delete_resource_server(&self) -> crate::client::fluent_builders::DeleteResourceServer {
                                crate::client::fluent_builders::DeleteResourceServer::new(self.handle.clone())
                            }
}

