// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateChannelMembership`](crate::client::fluent_builders::CreateChannelMembership) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::CreateChannelMembership::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::CreateChannelMembership::set_channel_arn): <p>The ARN of the channel to which you're adding users.</p>
    ///   - [`member_arn(impl Into<String>)`](crate::client::fluent_builders::CreateChannelMembership::member_arn) / [`set_member_arn(Option<String>)`](crate::client::fluent_builders::CreateChannelMembership::set_member_arn): <p>The <code>AppInstanceUserArn</code> of the member you want to add to the channel.</p>
    ///   - [`r#type(ChannelMembershipType)`](crate::client::fluent_builders::CreateChannelMembership::type) / [`set_type(Option<ChannelMembershipType>)`](crate::client::fluent_builders::CreateChannelMembership::set_type): <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::CreateChannelMembership::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::CreateChannelMembership::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    ///   - [`sub_channel_id(impl Into<String>)`](crate::client::fluent_builders::CreateChannelMembership::sub_channel_id) / [`set_sub_channel_id(Option<String>)`](crate::client::fluent_builders::CreateChannelMembership::set_sub_channel_id): <p>The ID of the SubChannel in the request.</p> <note>   <p>Only required when creating membership in a SubChannel for a moderator in an elastic channel.</p>  </note>
                            /// - On success, responds with [`CreateChannelMembershipOutput`](crate::output::CreateChannelMembershipOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::output::CreateChannelMembershipOutput::channel_arn): <p>The ARN of the channel.</p>
    ///   - [`member(Option<Identity>)`](crate::output::CreateChannelMembershipOutput::member): <p>The ARN and metadata of the member being added.</p>
    ///   - [`sub_channel_id(Option<String>)`](crate::output::CreateChannelMembershipOutput::sub_channel_id): <p>The ID of the SubChannel in the response.</p>
                            /// - On failure, responds with [`SdkError<CreateChannelMembershipError>`](crate::error::CreateChannelMembershipError)
    pub fn create_channel_membership(&self) -> crate::client::fluent_builders::CreateChannelMembership {
                                crate::client::fluent_builders::CreateChannelMembership::new(self.handle.clone())
                            }
}

