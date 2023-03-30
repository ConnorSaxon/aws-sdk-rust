// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SetIdentityFeedbackForwardingEnabled`](crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`identity(impl Into<String>)`](crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled::identity) / [`set_identity(Option<String>)`](crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled::set_identity): <p>The identity for which to set bounce and complaint notification forwarding. Examples: <code>user@example.com</code>, <code>example.com</code>.</p>
    ///   - [`forwarding_enabled(bool)`](crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled::forwarding_enabled) / [`set_forwarding_enabled(bool)`](crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled::set_forwarding_enabled): <p>Sets whether Amazon SES will forward bounce and complaint notifications as email. <code>true</code> specifies that Amazon SES will forward bounce and complaint notifications as email, in addition to any Amazon SNS topic publishing otherwise specified. <code>false</code> specifies that Amazon SES will publish bounce and complaint notifications only through Amazon SNS. This value can only be set to <code>false</code> when Amazon SNS topics are set for both <code>Bounce</code> and <code>Complaint</code> notification types.</p>
                            /// - On success, responds with [`SetIdentityFeedbackForwardingEnabledOutput`](crate::output::SetIdentityFeedbackForwardingEnabledOutput)
                            /// - On failure, responds with [`SdkError<SetIdentityFeedbackForwardingEnabledError>`](crate::error::SetIdentityFeedbackForwardingEnabledError)
    pub fn set_identity_feedback_forwarding_enabled(&self) -> crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled {
                                crate::client::fluent_builders::SetIdentityFeedbackForwardingEnabled::new(self.handle.clone())
                            }
}

