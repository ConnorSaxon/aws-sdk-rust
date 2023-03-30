// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteIntegrationResponse`](crate::client::fluent_builders::DeleteIntegrationResponse) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`resource_id(impl Into<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::resource_id) / [`set_resource_id(Option<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::set_resource_id): <p>Specifies a delete integration response request's resource identifier.</p>
    ///   - [`http_method(impl Into<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::http_method) / [`set_http_method(Option<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::set_http_method): <p>Specifies a delete integration response request's HTTP method.</p>
    ///   - [`status_code(impl Into<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::status_code) / [`set_status_code(Option<String>)`](crate::client::fluent_builders::DeleteIntegrationResponse::set_status_code): <p>Specifies a delete integration response request's status code.</p>
                            /// - On success, responds with [`DeleteIntegrationResponseOutput`](crate::output::DeleteIntegrationResponseOutput)
                            /// - On failure, responds with [`SdkError<DeleteIntegrationResponseError>`](crate::error::DeleteIntegrationResponseError)
    pub fn delete_integration_response(&self) -> crate::client::fluent_builders::DeleteIntegrationResponse {
                                crate::client::fluent_builders::DeleteIntegrationResponse::new(self.handle.clone())
                            }
}

