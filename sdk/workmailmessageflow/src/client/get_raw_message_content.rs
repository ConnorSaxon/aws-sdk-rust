// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetRawMessageContent`](crate::client::fluent_builders::GetRawMessageContent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`message_id(impl Into<String>)`](crate::client::fluent_builders::GetRawMessageContent::message_id) / [`set_message_id(Option<String>)`](crate::client::fluent_builders::GetRawMessageContent::set_message_id): <p>The identifier of the email message to retrieve.</p>
                            /// - On success, responds with [`GetRawMessageContentOutput`](crate::output::GetRawMessageContentOutput) with field(s):
    ///   - [`message_content(ByteStream)`](crate::output::GetRawMessageContentOutput::message_content): <p>The raw content of the email message, in MIME format.</p>
                            /// - On failure, responds with [`SdkError<GetRawMessageContentError>`](crate::error::GetRawMessageContentError)
    pub fn get_raw_message_content(&self) -> crate::client::fluent_builders::GetRawMessageContent {
                                crate::client::fluent_builders::GetRawMessageContent::new(self.handle.clone())
                            }
}

