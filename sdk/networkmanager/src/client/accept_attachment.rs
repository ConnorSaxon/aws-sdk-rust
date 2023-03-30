// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AcceptAttachment`](crate::client::fluent_builders::AcceptAttachment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`attachment_id(impl Into<String>)`](crate::client::fluent_builders::AcceptAttachment::attachment_id) / [`set_attachment_id(Option<String>)`](crate::client::fluent_builders::AcceptAttachment::set_attachment_id): <p>The ID of the attachment. </p>
                            /// - On success, responds with [`AcceptAttachmentOutput`](crate::output::AcceptAttachmentOutput) with field(s):
    ///   - [`attachment(Option<Attachment>)`](crate::output::AcceptAttachmentOutput::attachment): <p>The response to the attachment request. </p>
                            /// - On failure, responds with [`SdkError<AcceptAttachmentError>`](crate::error::AcceptAttachmentError)
    pub fn accept_attachment(&self) -> crate::client::fluent_builders::AcceptAttachment {
                                crate::client::fluent_builders::AcceptAttachment::new(self.handle.clone())
                            }
}

