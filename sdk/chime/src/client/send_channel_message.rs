// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendChannelMessage`](crate::client::fluent_builders::SendChannelMessage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::SendChannelMessage::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::SendChannelMessage::set_channel_arn): <p>The ARN of the channel.</p>
    ///   - [`content(impl Into<String>)`](crate::client::fluent_builders::SendChannelMessage::content) / [`set_content(Option<String>)`](crate::client::fluent_builders::SendChannelMessage::set_content): <p>The content of the message.</p>
    ///   - [`r#type(ChannelMessageType)`](crate::client::fluent_builders::SendChannelMessage::type) / [`set_type(Option<ChannelMessageType>)`](crate::client::fluent_builders::SendChannelMessage::set_type): <p>The type of message, <code>STANDARD</code> or <code>CONTROL</code>.</p>
    ///   - [`persistence(ChannelMessagePersistenceType)`](crate::client::fluent_builders::SendChannelMessage::persistence) / [`set_persistence(Option<ChannelMessagePersistenceType>)`](crate::client::fluent_builders::SendChannelMessage::set_persistence): <p>Boolean that controls whether the message is persisted on the back end. Required.</p>
    ///   - [`metadata(impl Into<String>)`](crate::client::fluent_builders::SendChannelMessage::metadata) / [`set_metadata(Option<String>)`](crate::client::fluent_builders::SendChannelMessage::set_metadata): <p>The optional metadata for each message.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::client::fluent_builders::SendChannelMessage::client_request_token) / [`set_client_request_token(Option<String>)`](crate::client::fluent_builders::SendChannelMessage::set_client_request_token): <p>The <code>Idempotency</code> token for each client request.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::SendChannelMessage::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::SendChannelMessage::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
                            /// - On success, responds with [`SendChannelMessageOutput`](crate::output::SendChannelMessageOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::output::SendChannelMessageOutput::channel_arn): <p>The ARN of the channel.</p>
    ///   - [`message_id(Option<String>)`](crate::output::SendChannelMessageOutput::message_id): <p>The ID string assigned to each message.</p>
                            /// - On failure, responds with [`SdkError<SendChannelMessageError>`](crate::error::SendChannelMessageError)
    pub fn send_channel_message(&self) -> crate::client::fluent_builders::SendChannelMessage {
                                crate::client::fluent_builders::SendChannelMessage::new(self.handle.clone())
                            }
}

