// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEmailIdentity`](crate::client::fluent_builders::GetEmailIdentity) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`email_identity(impl Into<String>)`](crate::client::fluent_builders::GetEmailIdentity::email_identity) / [`set_email_identity(Option<String>)`](crate::client::fluent_builders::GetEmailIdentity::set_email_identity): <p>The email identity.</p>
                            /// - On success, responds with [`GetEmailIdentityOutput`](crate::output::GetEmailIdentityOutput) with field(s):
    ///   - [`identity_type(Option<IdentityType>)`](crate::output::GetEmailIdentityOutput::identity_type): <p>The email identity type. Note: the <code>MANAGED_DOMAIN</code> identity type is not supported.</p>
    ///   - [`feedback_forwarding_status(bool)`](crate::output::GetEmailIdentityOutput::feedback_forwarding_status): <p>The feedback forwarding configuration for the identity.</p>  <p>If the value is <code>true</code>, you receive email notifications when bounce or complaint events occur. These notifications are sent to the address that you specified in the <code>Return-Path</code> header of the original email.</p>  <p>You're required to have a method of tracking bounces and complaints. If you haven't set up another mechanism for receiving bounce or complaint notifications (for example, by setting up an event destination), you receive an email notification when these events occur (even if this setting is disabled).</p>
    ///   - [`verified_for_sending_status(bool)`](crate::output::GetEmailIdentityOutput::verified_for_sending_status): <p>Specifies whether or not the identity is verified. You can only send email from verified email addresses or domains. For more information about verifying identities, see the <a href="https://docs.aws.amazon.com/pinpoint/latest/userguide/channels-email-manage-verify.html">Amazon Pinpoint User Guide</a>.</p>
    ///   - [`dkim_attributes(Option<DkimAttributes>)`](crate::output::GetEmailIdentityOutput::dkim_attributes): <p>An object that contains information about the DKIM attributes for the identity.</p>
    ///   - [`mail_from_attributes(Option<MailFromAttributes>)`](crate::output::GetEmailIdentityOutput::mail_from_attributes): <p>An object that contains information about the Mail-From attributes for the email identity.</p>
    ///   - [`policies(Option<HashMap<String, String>>)`](crate::output::GetEmailIdentityOutput::policies): <p>A map of policy names to policies.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::GetEmailIdentityOutput::tags): <p>An array of objects that define the tags (keys and values) that are associated with the email identity.</p>
    ///   - [`configuration_set_name(Option<String>)`](crate::output::GetEmailIdentityOutput::configuration_set_name): <p>The configuration set used by default when sending from this identity.</p>
    ///   - [`verification_status(Option<VerificationStatus>)`](crate::output::GetEmailIdentityOutput::verification_status): <p>The verification status of the identity. The status can be one of the following:</p>  <ul>   <li> <p> <code>PENDING</code> – The verification process was initiated, but Amazon SES hasn't yet been able to verify the identity.</p> </li>   <li> <p> <code>SUCCESS</code> – The verification process completed successfully.</p> </li>   <li> <p> <code>FAILED</code> – The verification process failed.</p> </li>   <li> <p> <code>TEMPORARY_FAILURE</code> – A temporary issue is preventing Amazon SES from determining the verification status of the identity.</p> </li>   <li> <p> <code>NOT_STARTED</code> – The verification process hasn't been initiated for the identity.</p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<GetEmailIdentityError>`](crate::error::GetEmailIdentityError)
    pub fn get_email_identity(&self) -> crate::client::fluent_builders::GetEmailIdentity {
                                crate::client::fluent_builders::GetEmailIdentity::new(self.handle.clone())
                            }
}

