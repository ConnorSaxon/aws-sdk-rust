// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchCreateChannelMembership`](crate::client::fluent_builders::BatchCreateChannelMembership) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::client::fluent_builders::BatchCreateChannelMembership::channel_arn) / [`set_channel_arn(Option<String>)`](crate::client::fluent_builders::BatchCreateChannelMembership::set_channel_arn): <p>The ARN of the channel to which you're adding users.</p>
    ///   - [`r#type(ChannelMembershipType)`](crate::client::fluent_builders::BatchCreateChannelMembership::type) / [`set_type(Option<ChannelMembershipType>)`](crate::client::fluent_builders::BatchCreateChannelMembership::set_type): <p>The membership type of a user, <code>DEFAULT</code> or <code>HIDDEN</code>. Default members are always returned as part of <code>ListChannelMemberships</code>. Hidden members are only returned if the type filter in <code>ListChannelMemberships</code> equals <code>HIDDEN</code>. Otherwise hidden members are not returned. This is only supported by moderators.</p>
    ///   - [`member_arns(Vec<String>)`](crate::client::fluent_builders::BatchCreateChannelMembership::member_arns) / [`set_member_arns(Option<Vec<String>>)`](crate::client::fluent_builders::BatchCreateChannelMembership::set_member_arns): <p>The ARNs of the members you want to add to the channel.</p>
    ///   - [`chime_bearer(impl Into<String>)`](crate::client::fluent_builders::BatchCreateChannelMembership::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::client::fluent_builders::BatchCreateChannelMembership::set_chime_bearer): <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
                            /// - On success, responds with [`BatchCreateChannelMembershipOutput`](crate::output::BatchCreateChannelMembershipOutput) with field(s):
    ///   - [`batch_channel_memberships(Option<BatchChannelMemberships>)`](crate::output::BatchCreateChannelMembershipOutput::batch_channel_memberships): <p>The list of channel memberships in the response.</p>
    ///   - [`errors(Option<Vec<BatchCreateChannelMembershipError>>)`](crate::output::BatchCreateChannelMembershipOutput::errors): <p>If the action fails for one or more of the memberships in the request, a list of the memberships is returned, along with error codes and error messages.</p>
                            /// - On failure, responds with [`SdkError<BatchCreateChannelMembershipError>`](crate::error::BatchCreateChannelMembershipError)
    pub fn batch_create_channel_membership(&self) -> crate::client::fluent_builders::BatchCreateChannelMembership {
                                crate::client::fluent_builders::BatchCreateChannelMembership::new(self.handle.clone())
                            }
}

