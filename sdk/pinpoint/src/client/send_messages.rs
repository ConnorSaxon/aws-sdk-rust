// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendMessages`](crate::client::fluent_builders::SendMessages) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::client::fluent_builders::SendMessages::application_id) / [`set_application_id(Option<String>)`](crate::client::fluent_builders::SendMessages::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`message_request(MessageRequest)`](crate::client::fluent_builders::SendMessages::message_request) / [`set_message_request(Option<MessageRequest>)`](crate::client::fluent_builders::SendMessages::set_message_request): <p>Specifies the configuration and other settings for a message.</p>
                            /// - On success, responds with [`SendMessagesOutput`](crate::output::SendMessagesOutput) with field(s):
    ///   - [`message_response(Option<MessageResponse>)`](crate::output::SendMessagesOutput::message_response): <p>Provides information about the results of a request to send a message to an endpoint address.</p>
                            /// - On failure, responds with [`SdkError<SendMessagesError>`](crate::error::SendMessagesError)
    pub fn send_messages(&self) -> crate::client::fluent_builders::SendMessages {
                                crate::client::fluent_builders::SendMessages::new(self.handle.clone())
                            }
}

