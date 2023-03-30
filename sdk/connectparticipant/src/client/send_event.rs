// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendEvent`](crate::client::fluent_builders::SendEvent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`content_type(impl Into<String>)`](crate::client::fluent_builders::SendEvent::content_type) / [`set_content_type(Option<String>)`](crate::client::fluent_builders::SendEvent::set_content_type): <p>The content type of the request. Supported types are:</p>  <ul>   <li> <p>application/vnd.amazonaws.connect.event.typing</p> </li>   <li> <p>application/vnd.amazonaws.connect.event.connection.acknowledged</p> </li>   <li> <p>application/vnd.amazonaws.connect.event.message.delivered</p> </li>   <li> <p>application/vnd.amazonaws.connect.event.message.read</p> </li>  </ul>
    ///   - [`content(impl Into<String>)`](crate::client::fluent_builders::SendEvent::content) / [`set_content(Option<String>)`](crate::client::fluent_builders::SendEvent::set_content): <p>The content of the event to be sent (for example, message text). For content related to message receipts, this is supported in the form of a JSON string.</p>  <p>Sample Content: "{\"messageId\":\"11111111-aaaa-bbbb-cccc-EXAMPLE01234\"}"</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::SendEvent::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::SendEvent::set_client_token): <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    ///   - [`connection_token(impl Into<String>)`](crate::client::fluent_builders::SendEvent::connection_token) / [`set_connection_token(Option<String>)`](crate::client::fluent_builders::SendEvent::set_connection_token): <p>The authentication token associated with the participant's connection.</p>
                            /// - On success, responds with [`SendEventOutput`](crate::output::SendEventOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::output::SendEventOutput::id): <p>The ID of the response.</p>
    ///   - [`absolute_time(Option<String>)`](crate::output::SendEventOutput::absolute_time): <p>The time when the event was sent.</p>  <p>It's specified in ISO 8601 format: yyyy-MM-ddThh:mm:ss.SSSZ. For example, 2019-11-08T02:41:28.172Z.</p>
                            /// - On failure, responds with [`SdkError<SendEventError>`](crate::error::SendEventError)
    pub fn send_event(&self) -> crate::client::fluent_builders::SendEvent {
                                crate::client::fluent_builders::SendEvent::new(self.handle.clone())
                            }
}

