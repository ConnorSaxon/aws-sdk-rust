// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendEmail`](crate::client::fluent_builders::SendEmail) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`from_email_address(impl Into<String>)`](crate::client::fluent_builders::SendEmail::from_email_address) / [`set_from_email_address(Option<String>)`](crate::client::fluent_builders::SendEmail::set_from_email_address): <p>The email address that you want to use as the "From" address for the email. The address that you specify has to be verified. </p>
    ///   - [`destination(Destination)`](crate::client::fluent_builders::SendEmail::destination) / [`set_destination(Option<Destination>)`](crate::client::fluent_builders::SendEmail::set_destination): <p>An object that contains the recipients of the email message.</p>
    ///   - [`reply_to_addresses(Vec<String>)`](crate::client::fluent_builders::SendEmail::reply_to_addresses) / [`set_reply_to_addresses(Option<Vec<String>>)`](crate::client::fluent_builders::SendEmail::set_reply_to_addresses): <p>The "Reply-to" email addresses for the message. When the recipient replies to the message, each Reply-to address receives the reply.</p>
    ///   - [`feedback_forwarding_email_address(impl Into<String>)`](crate::client::fluent_builders::SendEmail::feedback_forwarding_email_address) / [`set_feedback_forwarding_email_address(Option<String>)`](crate::client::fluent_builders::SendEmail::set_feedback_forwarding_email_address): <p>The address that Amazon Pinpoint should send bounce and complaint notifications to.</p>
    ///   - [`content(EmailContent)`](crate::client::fluent_builders::SendEmail::content) / [`set_content(Option<EmailContent>)`](crate::client::fluent_builders::SendEmail::set_content): <p>An object that contains the body of the message. You can send either a Simple message or a Raw message.</p>
    ///   - [`email_tags(Vec<MessageTag>)`](crate::client::fluent_builders::SendEmail::email_tags) / [`set_email_tags(Option<Vec<MessageTag>>)`](crate::client::fluent_builders::SendEmail::set_email_tags): <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using the <code>SendEmail</code> operation. Tags correspond to characteristics of the email that you define, so that you can publish email sending events. </p>
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::SendEmail::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::SendEmail::set_configuration_set_name): <p>The name of the configuration set that you want to use when sending the email.</p>
                            /// - On success, responds with [`SendEmailOutput`](crate::output::SendEmailOutput) with field(s):
    ///   - [`message_id(Option<String>)`](crate::output::SendEmailOutput::message_id): <p>A unique identifier for the message that is generated when Amazon Pinpoint accepts the message.</p> <note>   <p>It is possible for Amazon Pinpoint to accept a message without sending it. This can happen when the message you're trying to send has an attachment doesn't pass a virus check, or when you send a templated email that contains invalid personalization content, for example.</p>  </note>
                            /// - On failure, responds with [`SdkError<SendEmailError>`](crate::error::SendEmailError)
    pub fn send_email(&self) -> crate::client::fluent_builders::SendEmail {
                                crate::client::fluent_builders::SendEmail::new(self.handle.clone())
                            }
}

