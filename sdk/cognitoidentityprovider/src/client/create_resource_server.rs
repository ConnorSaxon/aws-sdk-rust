// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateResourceServer`](crate::client::fluent_builders::CreateResourceServer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::CreateResourceServer::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::CreateResourceServer::set_user_pool_id): <p>The user pool ID for the user pool.</p>
    ///   - [`identifier(impl Into<String>)`](crate::client::fluent_builders::CreateResourceServer::identifier) / [`set_identifier(Option<String>)`](crate::client::fluent_builders::CreateResourceServer::set_identifier): <p>A unique resource server identifier for the resource server. This could be an HTTPS endpoint where the resource server is located, such as <code>https://my-weather-api.example.com</code>.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateResourceServer::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateResourceServer::set_name): <p>A friendly name for the resource server.</p>
    ///   - [`scopes(Vec<ResourceServerScopeType>)`](crate::client::fluent_builders::CreateResourceServer::scopes) / [`set_scopes(Option<Vec<ResourceServerScopeType>>)`](crate::client::fluent_builders::CreateResourceServer::set_scopes): <p>A list of scopes. Each scope is a key-value map with the keys <code>name</code> and <code>description</code>.</p>
                            /// - On success, responds with [`CreateResourceServerOutput`](crate::output::CreateResourceServerOutput) with field(s):
    ///   - [`resource_server(Option<ResourceServerType>)`](crate::output::CreateResourceServerOutput::resource_server): <p>The newly created resource server.</p>
                            /// - On failure, responds with [`SdkError<CreateResourceServerError>`](crate::error::CreateResourceServerError)
    pub fn create_resource_server(&self) -> crate::client::fluent_builders::CreateResourceServer {
                                crate::client::fluent_builders::CreateResourceServer::new(self.handle.clone())
                            }
}

