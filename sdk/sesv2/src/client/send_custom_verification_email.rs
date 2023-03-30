// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SendCustomVerificationEmail`](crate::client::fluent_builders::SendCustomVerificationEmail) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`email_address(impl Into<String>)`](crate::client::fluent_builders::SendCustomVerificationEmail::email_address) / [`set_email_address(Option<String>)`](crate::client::fluent_builders::SendCustomVerificationEmail::set_email_address): <p>The email address to verify.</p>
    ///   - [`template_name(impl Into<String>)`](crate::client::fluent_builders::SendCustomVerificationEmail::template_name) / [`set_template_name(Option<String>)`](crate::client::fluent_builders::SendCustomVerificationEmail::set_template_name): <p>The name of the custom verification email template to use when sending the verification email.</p>
    ///   - [`configuration_set_name(impl Into<String>)`](crate::client::fluent_builders::SendCustomVerificationEmail::configuration_set_name) / [`set_configuration_set_name(Option<String>)`](crate::client::fluent_builders::SendCustomVerificationEmail::set_configuration_set_name): <p>Name of a configuration set to use when sending the verification email.</p>
                            /// - On success, responds with [`SendCustomVerificationEmailOutput`](crate::output::SendCustomVerificationEmailOutput) with field(s):
    ///   - [`message_id(Option<String>)`](crate::output::SendCustomVerificationEmailOutput::message_id): <p>The unique message identifier returned from the <code>SendCustomVerificationEmail</code> operation.</p>
                            /// - On failure, responds with [`SdkError<SendCustomVerificationEmailError>`](crate::error::SendCustomVerificationEmailError)
    pub fn send_custom_verification_email(&self) -> crate::client::fluent_builders::SendCustomVerificationEmail {
                                crate::client::fluent_builders::SendCustomVerificationEmail::new(self.handle.clone())
                            }
}

