// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeChannelMembershipForAppInstanceUser`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::set_channel_arn): <p>The ARN of the channel to which the user belongs.</p>
    ///   - [`app_instance_user_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::app_instance_user_arn) / [`set_app_instance_user_arn(Option<String>)`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::set_app_instance_user_arn): <p>The ARN of the user in a channel.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
                            /// - On success, responds with [`DescribeChannelMembershipForAppInstanceUserOutput`](crate::output::DescribeChannelMembershipForAppInstanceUserOutput) with field(s):
    ///   - [`channel_membership(Option<ChannelMembershipForAppInstanceUserSummary>)`](crate::output::DescribeChannelMembershipForAppInstanceUserOutput::channel_membership): <p>The channel to which a user belongs.</p>
                            /// - On failure, responds with [`SdkError<DescribeChannelMembershipForAppInstanceUserError>`](crate::error::DescribeChannelMembershipForAppInstanceUserError)
    pub fn describe_channel_membership_for_app_instance_user(&self) -> crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser {
                                crate::client::fluent_builders::DescribeChannelMembershipForAppInstanceUser::new(self.handle.clone())
                            }
}

