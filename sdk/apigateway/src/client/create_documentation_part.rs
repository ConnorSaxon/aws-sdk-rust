// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateDocumentationPart`](crate::client::fluent_builders::CreateDocumentationPart) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`rest_api_id(impl Into<String>)`](crate::client::fluent_builders::CreateDocumentationPart::rest_api_id) / [`set_rest_api_id(Option<String>)`](crate::client::fluent_builders::CreateDocumentationPart::set_rest_api_id): <p>The string identifier of the associated RestApi.</p>
    ///   - [`location(DocumentationPartLocation)`](crate::client::fluent_builders::CreateDocumentationPart::location) / [`set_location(Option<DocumentationPartLocation>)`](crate::client::fluent_builders::CreateDocumentationPart::set_location): <p>The location of the targeted API entity of the to-be-created documentation part.</p>
    ///   - [`properties(impl Into<String>)`](crate::client::fluent_builders::CreateDocumentationPart::properties) / [`set_properties(Option<String>)`](crate::client::fluent_builders::CreateDocumentationPart::set_properties): <p>The new documentation content map of the targeted API entity. Enclosed key-value pairs are API-specific, but only OpenAPI-compliant key-value pairs can be exported and, hence, published.</p>
                            /// - On success, responds with [`CreateDocumentationPartOutput`](crate::output::CreateDocumentationPartOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::CreateDocumentationPartOutput::id): <p>The DocumentationPart identifier, generated by API Gateway when the <code>DocumentationPart</code> is created.</p>
    ///   - [`location(Option<DocumentationPartLocation>)`](crate::output::CreateDocumentationPartOutput::location): <p>The location of the API entity to which the documentation applies. Valid fields depend on the targeted API entity type. All the valid location fields are not required. If not explicitly specified, a valid location field is treated as a wildcard and associated documentation content may be inherited by matching entities, unless overridden.</p>
    ///   - [`properties(Option<String>)`](crate::output::CreateDocumentationPartOutput::properties): <p>A content map of API-specific key-value pairs describing the targeted API entity. The map must be encoded as a JSON string, e.g., <code>"{ \"description\": \"The API does ...\" }"</code>. Only OpenAPI-compliant documentation-related fields from the properties map are exported and, hence, published as part of the API entity definitions, while the original documentation parts are exported in a OpenAPI extension of <code>x-amazon-apigateway-documentation</code>.</p>
                            /// - On failure, responds with [`SdkError<CreateDocumentationPartError>`](crate::error::CreateDocumentationPartError)
    pub fn create_documentation_part(&self) -> crate::client::fluent_builders::CreateDocumentationPart {
                                crate::client::fluent_builders::CreateDocumentationPart::new(self.handle.clone())
                            }
}

