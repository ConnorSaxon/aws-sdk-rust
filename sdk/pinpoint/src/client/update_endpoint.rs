// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateEndpoint`](crate::client::fluent_builders::UpdateEndpoint) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::UpdateEndpoint::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::UpdateEndpoint::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`endpoint_id(impl Into<String>)`](crate::client::fluent_builders::UpdateEndpoint::endpoint_id) / [`set_endpoint_id(Option<String>)`](crate::client::fluent_builders::UpdateEndpoint::set_endpoint_id): <p>The unique identifier for the endpoint.</p>
    ///   - [`endpoint_request(EndpointRequest)`](crate::client::fluent_builders::UpdateEndpoint::endpoint_request) / [`set_endpoint_request(Option<EndpointRequest>)`](crate::client::fluent_builders::UpdateEndpoint::set_endpoint_request): <p>Specifies the channel type and other settings for an endpoint.</p>
                            /// - On success, responds with [`UpdateEndpointOutput`](crate::output::UpdateEndpointOutput) with field(s):
    ///   - [`message_body(Option<MessageBody>)`](crate::output::UpdateEndpointOutput::message_body): <p>Provides information about an API request or response.</p>
                            /// - On failure, responds with [`SdkError<UpdateEndpointError>`](crate::error::UpdateEndpointError)
    pub fn update_endpoint(&self) -> crate::client::fluent_builders::UpdateEndpoint {
                                crate::client::fluent_builders::UpdateEndpoint::new(self.handle.clone())
                            }
}

