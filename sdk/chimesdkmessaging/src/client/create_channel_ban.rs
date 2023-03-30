// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateChannelBan`](crate::client::fluent_builders::CreateChannelBan) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::CreateChannelBan::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::CreateChannelBan::set_channel_arn): <p>The ARN of the ban request.</p>
    ///   - [`member_arn(impl Into<String>)`](crate::client::fluent_builders::CreateChannelBan::member_arn) / [`set_member_arn(Option<String>)`](crate::client::fluent_builders::CreateChannelBan::set_member_arn): <p>The <code>AppInstanceUserArn</code> of the member being banned.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::CreateChannelBan::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::CreateChannelBan::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
                            /// - On success, responds with [`CreateChannelBanOutput`](crate::output::CreateChannelBanOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::output::CreateChannelBanOutput::channel_arn): <p>The ARN of the response to the ban request.</p>
    ///   - [`member(Option<Identity>)`](crate::output::CreateChannelBanOutput::member): <p>The <code>ChannelArn</code> and <code>BannedIdentity</code> of the member in the ban response.</p>
                            /// - On failure, responds with [`SdkError<CreateChannelBanError>`](crate::error::CreateChannelBanError)
    pub fn create_channel_ban(&self) -> crate::client::fluent_builders::CreateChannelBan {
                                crate::client::fluent_builders::CreateChannelBan::new(self.handle.clone())
                            }
}

