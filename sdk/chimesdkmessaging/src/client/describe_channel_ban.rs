// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeChannelBan`](crate::client::fluent_builders::DescribeChannelBan) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeChannelBan::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::DescribeChannelBan::set_channel_arn): <p>The ARN of the channel from which the user is banned.</p>
    ///   - [`member_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeChannelBan::member_arn) / [`set_member_arn(Option<String>)`](crate::client::fluent_builders::DescribeChannelBan::set_member_arn): <p>The <code>AppInstanceUserArn</code> of the member being banned.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::DescribeChannelBan::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::DescribeChannelBan::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
                            /// - On success, responds with [`DescribeChannelBanOutput`](crate::output::DescribeChannelBanOutput) with field(s):
    ///   - [`channel_ban(Option<ChannelBan>)`](crate::output::DescribeChannelBanOutput::channel_ban): <p>The details of the ban.</p>
                            /// - On failure, responds with [`SdkError<DescribeChannelBanError>`](crate::error::DescribeChannelBanError)
    pub fn describe_channel_ban(&self) -> crate::client::fluent_builders::DescribeChannelBan {
                                crate::client::fluent_builders::DescribeChannelBan::new(self.handle.clone())
                            }
}

