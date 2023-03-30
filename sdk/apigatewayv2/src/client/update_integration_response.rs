// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateIntegrationResponse`](crate::client::fluent_builders::UpdateIntegrationResponse) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::api_id) / [`set_api_id(Option<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_api_id): <p>The API identifier.</p>
    ///   - [`content_handling_strategy(ContentHandlingStrategy)`](crate::client::fluent_builders::UpdateIntegrationResponse::content_handling_strategy) / [`set_content_handling_strategy(Option<ContentHandlingStrategy>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_content_handling_strategy): <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>  <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>  <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>  <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    ///   - [`integration_id(impl Into<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::integration_id) / [`set_integration_id(Option<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_integration_id): <p>The integration ID.</p>
    ///   - [`integration_response_id(impl Into<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::integration_response_id) / [`set_integration_response_id(Option<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_integration_response_id): <p>The integration response ID.</p>
    ///   - [`integration_response_key(impl Into<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::integration_response_key) / [`set_integration_response_key(Option<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_integration_response_key): <p>The integration response key.</p>
    ///   - [`response_parameters(HashMap<String, String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::response_parameters) / [`set_response_parameters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_response_parameters): <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.<replaceable>   {name}  </replaceable> , where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.<replaceable>   {name}  </replaceable> or integration.response.body.<replaceable>   {JSON-expression}  </replaceable> , where <replaceable>   {name}  </replaceable> is a valid and unique response header name and <replaceable>   {JSON-expression}  </replaceable> is a valid JSON expression without the $ prefix.</p>
    ///   - [`response_templates(HashMap<String, String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::response_templates) / [`set_response_templates(Option<HashMap<String, String>>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_response_templates): <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    ///   - [`template_selection_expression(impl Into<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::template_selection_expression) / [`set_template_selection_expression(Option<String>)`](crate::client::fluent_builders::UpdateIntegrationResponse::set_template_selection_expression): <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
                            /// - On success, responds with [`UpdateIntegrationResponseOutput`](crate::output::UpdateIntegrationResponseOutput) with field(s):
    ///   - [`content_handling_strategy(Option<ContentHandlingStrategy>)`](crate::output::UpdateIntegrationResponseOutput::content_handling_strategy): <p>Supported only for WebSocket APIs. Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>  <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>  <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>  <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    ///   - [`integration_response_id(Option<String>)`](crate::output::UpdateIntegrationResponseOutput::integration_response_id): <p>The integration response ID.</p>
    ///   - [`integration_response_key(Option<String>)`](crate::output::UpdateIntegrationResponseOutput::integration_response_key): <p>The integration response key.</p>
    ///   - [`response_parameters(Option<HashMap<String, String>>)`](crate::output::UpdateIntegrationResponseOutput::response_parameters): <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where name is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where name is a valid and unique response header name and JSON-expression is a valid JSON expression without the $ prefix.</p>
    ///   - [`response_templates(Option<HashMap<String, String>>)`](crate::output::UpdateIntegrationResponseOutput::response_templates): <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    ///   - [`template_selection_expression(Option<String>)`](crate::output::UpdateIntegrationResponseOutput::template_selection_expression): <p>The template selection expressions for the integration response.</p>
                            /// - On failure, responds with [`SdkError<UpdateIntegrationResponseError>`](crate::error::UpdateIntegrationResponseError)
    pub fn update_integration_response(&self) -> crate::client::fluent_builders::UpdateIntegrationResponse {
                                crate::client::fluent_builders::UpdateIntegrationResponse::new(self.handle.clone())
                            }
}

